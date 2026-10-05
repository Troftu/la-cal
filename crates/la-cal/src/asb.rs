//! Project-defined OMS JSON-in-Protobuf adapter over a single Zenoh session.
//!
//! Only the startup subscriber dispatches to local OWP registrations. Publication
//! completion is distinct from that delivery. Started blocking workers cannot be
//! canceled: a deadline may leave publication uncertain, and cleanup never joins
//! such a worker indefinitely.

use std::sync::Arc;
use std::time::Duration;

use prost::Message;
use sleet_types::owp::OwpError;
use tokio::task::JoinHandle;
use tokio::time::{self, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use zenoh::Wait;
use zenoh::sample::SampleKind;

use crate::config::{SUPPORTED_TOPICS, ServerConfig};
use crate::router::Router;
use crate::schema::model::Schema;
use crate::validator::validate_oms_json;

mod envelope {
    include!(concat!(env!("OUT_DIR"), "/la_cal.asb.rs"));
}
use envelope::OmsJsonEnvelope;

const KEY_PREFIX: &str = "la-cal/uci/002.5.0/";
const SUBSCRIBER_KEY: &str = "la-cal/uci/002.5.0/*";

/// Async publication surface shared by the OWP frontend.
pub struct Adapter {
    session: Arc<zenoh::Session>,
    schema: Arc<Schema>,
    operation_deadline: Duration,
    cleanup_deadline: Duration,
    stop: CancellationToken,
    receiver: JoinHandle<()>,
}

impl Adapter {
    /// Establish router readiness and the single wildcard subscriber before OWP
    /// admission. An unsuccessful partial startup closes its acquired session.
    pub async fn open(
        config: &ServerConfig,
        schema: Arc<Schema>,
        router: Arc<Router>,
        shutdown: CancellationToken,
    ) -> Result<Self, String> {
        let deadline = Duration::from_millis(config.asb_startup_timeout_ms);
        let operation_deadline = Duration::from_millis(config.asb_operation_timeout_ms);
        let zenoh_config = zenoh::Config::from_json5(&format!(
            r#"{{
            mode: "client",
            connect: {{ endpoints: ["{}"], timeout_ms: {} }},
            listen: {{ endpoints: [] }},
            scouting: {{ multicast: {{ enabled: false }}, gossip: {{ enabled: false }} }}
        }}"#,
            config.zenoh_router, config.asb_startup_timeout_ms
        ))
        .map_err(|e| format!("Zenoh configuration: {e}"))?;
        let session = tokio::select! {
            () = shutdown.cancelled() => return Err("ASB startup cancelled".to_owned()),
            result = timeout(deadline, zenoh::open(zenoh_config)) => {
                Arc::new(result.map_err(|_| "Zenoh session startup deadline".to_owned())?
                    .map_err(|e| format!("Zenoh session startup: {e}"))?)
            }
        };
        let startup = timeout(deadline, async {
            wait_for_stable_router(&session).await;
            session
                .declare_subscriber(SUBSCRIBER_KEY)
                .await
                .map_err(|e| format!("ASB subscriber startup: {e}"))
        });
        let subscriber_result = tokio::select! {
            () = shutdown.cancelled() => Err("ASB startup cancelled".to_owned()),
            result = startup => result.map_err(|_| "ASB router/subscriber readiness deadline".to_owned()).and_then(std::convert::identity),
        };
        let subscriber = match subscriber_result {
            Ok(subscriber) => subscriber,
            Err(message) => {
                let _ = timeout(deadline, session.close()).await;
                return Err(message);
            }
        };
        let stop = CancellationToken::new();
        let receiver_stop = stop.clone();
        let receiver_schema = Arc::clone(&schema);
        let receiver = tokio::spawn(async move {
            loop {
                let sample = tokio::select! {
                    () = receiver_stop.cancelled() => break,
                    result = subscriber.recv_async() => match result {
                        Ok(sample) => sample,
                        Err(e) => {
                            error!(error = %e, "ASB subscriber failed");
                            shutdown.cancel();
                            break;
                        }
                    }
                };
                let schema = Arc::clone(&receiver_schema);
                // Copy/decode/validate away from connection I/O, including
                // potentially expensive sample buffer consolidation.
                let assessment = tokio::task::spawn_blocking(move || {
                    if sample.kind() != SampleKind::Put {
                        return Err("only PUT samples carry OMS messages".to_owned());
                    }
                    let topic = sample
                        .key_expr()
                        .as_str()
                        .strip_prefix(KEY_PREFIX)
                        .filter(|topic| SUPPORTED_TOPICS.contains(topic))
                        .ok_or_else(|| format!("unsupported exact ASB key {}", sample.key_expr()))?
                        .to_owned();
                    let envelope = OmsJsonEnvelope::decode(sample.payload().to_bytes().as_ref())
                        .map_err(|e| format!("invalid Protobuf envelope: {e}"))?;
                    let raw_json = String::from_utf8(envelope.oms_json)
                        .map_err(|e| format!("invalid UTF-8 OMS JSON: {e}"))?;
                    assess(&schema, &topic, &raw_json)?;
                    Ok((topic, raw_json))
                });
                match timeout(operation_deadline, assessment).await {
                    Ok(Ok(Ok((topic, raw_json)))) => {
                        router.publish(&topic, &topic, &raw_json).await;
                    }
                    Ok(Ok(Err(details))) => warn!(details, "discarding invalid ASB sample"),
                    Ok(Err(e)) => error!(error = %e, "ASB receive worker failed"),
                    Err(_) => error!("ASB receive assessment deadline; sample not forwarded"),
                }
            }
            if let Err(e) = subscriber.undeclare().await {
                warn!(error = %e, "ASB subscriber cleanup failed");
            }
        });
        info!(key = SUBSCRIBER_KEY, "ASB subscriber established");
        Ok(Self {
            session,
            schema,
            operation_deadline,
            cleanup_deadline: deadline,
            stop,
            receiver,
        })
    }

    /// Validate, encode original bytes and finish Zenoh publication. This async
    /// operation owns offload and its deadline; timeout does not prove absence
    /// of publication. Authorization remains at the OWP frontend.
    pub async fn publish(&self, topic: String, raw_json: String) -> Result<(), (OwpError, String)> {
        let session = Arc::clone(&self.session);
        let schema = Arc::clone(&self.schema);
        let work = tokio::task::spawn_blocking(move || {
            assess(&schema, &topic, &raw_json).map_err(|e| (OwpError::InvalidMessage, e))?;
            let bytes = OmsJsonEnvelope {
                oms_json: raw_json.into_bytes(),
            }
            .encode_to_vec();
            session
                .put(format!("{KEY_PREFIX}{topic}"), bytes)
                .congestion_control(zenoh::qos::CongestionControl::Block)
                .wait()
                .map_err(|e| {
                    (
                        OwpError::InternalError,
                        format!("Zenoh publication failed: {e}"),
                    )
                })
        });
        timeout(self.operation_deadline, work)
            .await
            .map_err(|_| {
                (
                    OwpError::InternalError,
                    "ASB publication deadline; publication outcome uncertain".to_owned(),
                )
            })?
            .map_err(|e| {
                (
                    OwpError::InternalError,
                    format!("ASB publication worker failed: {e}"),
                )
            })?
    }

    /// Stop local delivery, undeclare the startup subscriber and close the
    /// session with bounded waits. No per-client operation owns these resources.
    pub async fn close(&mut self) -> Result<(), String> {
        self.stop.cancel();
        let receiver_result = timeout(self.cleanup_deadline, &mut self.receiver).await;
        if receiver_result.is_err() {
            self.receiver.abort();
        }
        let session_result = timeout(self.cleanup_deadline, self.session.close()).await;
        receiver_result
            .map_err(|_| "ASB subscriber cleanup deadline".to_owned())?
            .map_err(|e| format!("ASB subscriber task: {e}"))?;
        session_result
            .map_err(|_| "ASB session cleanup deadline".to_owned())?
            .map_err(|e| format!("ASB session cleanup: {e}"))
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        self.stop.cancel();
        self.receiver.abort();
    }
}

async fn wait_for_stable_router(session: &zenoh::Session) {
    let mut previous = Vec::new();
    loop {
        let mut current: Vec<_> = session.info().routers_zid().await.collect();
        current.sort_unstable();
        if !current.is_empty() && current == previous {
            info!(routers = ?current, "stable ASB router readiness");
            return;
        }
        previous = current;
        time::sleep(Duration::from_millis(100)).await;
    }
}

fn assess(schema: &Schema, topic: &str, raw_json: &str) -> Result<(), String> {
    let start = std::time::Instant::now();
    let value: serde_json::Value = serde_json::from_str(raw_json).map_err(|e| {
        metrics::counter!("sleet_validation_failures_total", "category" => "invalid_json")
            .increment(1);
        format!("invalid JSON payload: {e}")
    })?;
    let root = value
        .as_object()
        .filter(|root| root.len() == 1)
        .ok_or_else(|| "OMS JSON requires exactly one root".to_owned())?;
    if !SUPPORTED_TOPICS.contains(&topic) || !root.contains_key(topic) {
        metrics::counter!("sleet_validation_failures_total", "category" => "topic_binding")
            .increment(1);
        return Err(format!(
            "topic {topic} requires its identically named supported UCI 002.5.0 root; received {}",
            root.keys().next().expect("one root")
        ));
    }
    let result = validate_oms_json(schema, &value).map(|_| ()).map_err(|e| {
        metrics::counter!("sleet_validation_failures_total", "category" => "schema_validation")
            .increment(1);
        format!("schema validation failed: {e:?}")
    });
    metrics::histogram!("sleet_validation_duration_seconds").record(start.elapsed().as_secs_f64());
    result
}
