//! Project-defined OMS JSON-in-Protobuf adapter over a single Zenoh session.
//!
//! Only the startup subscriber dispatches to local OWP registrations. Publication
//! completion is distinct from that delivery. Started blocking workers cannot be
//! canceled: a deadline may leave publication uncertain, and cleanup never joins
//! such a worker indefinitely.
//! Router observation uses stable Zenoh 1.10.1 ZID APIs and the project-selected
//! OMS 2.5 / UCI 002.5.0 profile. Outages entirely between snapshots may be missed;
//! the polling period does not bound the transport's underlying loss detection.

use std::ops::Deref;
use std::sync::{Arc, OnceLock};
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

/// Zenoh 1.10.1 resource destructors can synchronously lock/enqueue/close.
/// Keep raw resources guarded even in worker outputs: cancellation can drop a
/// completed output or the last Arc on an async thread. The fallback drop thread
/// is independent of Tokio, which may already be shutting down/cancelling queued
/// blocking jobs. Explicit cleanup instead consumes resources inside a worker.
struct BlockingDrop<T: Send + 'static> {
    value: Option<T>,
}

impl<T: Send + 'static> BlockingDrop<T> {
    const fn new(value: T) -> Self {
        Self { value: Some(value) }
    }

    /// Only consume on a blocking worker, including the resulting raw Drop.
    fn into_inner(mut self) -> T {
        self.value.take().expect("resource present")
    }
}

impl<T: Send + 'static> Deref for BlockingDrop<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.as_ref().expect("resource present")
    }
}

impl<T: Send + 'static> Drop for BlockingDrop<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            // Never join: raw Drop is as noncancellable as a started operation.
            drop(std::thread::spawn(move || drop(value)));
        }
    }
}

type SessionHandle = Arc<BlockingDrop<zenoh::Session>>;

/// Construct AND resolve synchronous Zenoh builders inside the worker. Merely
/// wrapping their `IntoFuture` in timeout is not a bound: several use `ready(wait())`.
/// All resource inputs/outputs must be guarded until consumed on a worker.
async fn bounded_worker<T, F>(deadline: Duration, operation: &str, work: F) -> Result<T, String>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, String> + Send + 'static,
{
    let mut worker = tokio::task::spawn_blocking(work);
    if let Ok(result) = timeout(deadline, &mut worker).await {
        result.map_err(|e| format!("{operation} worker: {e}"))?
    } else {
        // Abort can prevent an unstarted job. Started jobs continue, including
        // cleanup of a late result; there is deliberately no subsequent join.
        worker.abort();
        Err(format!("{operation} deadline"))
    }
}

/// First infrastructure failure is terminal for this run, even if Zenoh reconnects.
struct RunState {
    failure: OnceLock<String>,
    stop: CancellationToken,
    shutdown: CancellationToken,
}

impl RunState {
    fn fail(&self, message: String) {
        if self.failure.set(message).is_ok() {
            error!(
                details = self.failure.get().expect("failure set"),
                "ASB run failed"
            );
        }
        self.stop.cancel();
        self.shutdown.cancel();
    }

    fn publication_error(&self) -> Option<(OwpError, String)> {
        self.failure
            .get()
            .cloned()
            .or_else(|| {
                self.stop
                    .is_cancelled()
                    .then(|| "ASB is stopping".to_owned())
            })
            .map(|message| (OwpError::InternalError, message))
    }
}

/// Async publication surface shared by the OWP frontend.
pub struct Adapter {
    session: Option<SessionHandle>,
    schema: Arc<Schema>,
    operation_deadline: Duration,
    cleanup_deadline: Duration,
    state: Arc<RunState>,
    receiver: JoinHandle<Result<(), String>>,
    observer: JoinHandle<()>,
}

impl Adapter {
    /// Establish router readiness and the single wildcard subscriber before OWP
    /// admission. An unsuccessful partial startup closes its acquired session.
    #[allow(clippy::too_many_lines)]
    pub async fn open(
        config: &ServerConfig,
        schema: Arc<Schema>,
        router: Arc<Router>,
        shutdown: CancellationToken,
    ) -> Result<Self, String> {
        let deadline = Duration::from_millis(config.asb_startup_timeout_ms);
        let operation_deadline = Duration::from_millis(config.asb_operation_timeout_ms);
        let cleanup_deadline = Duration::from_millis(config.asb_cleanup_timeout_ms);
        let poll_interval = Duration::from_millis(config.asb_router_poll_interval_ms);
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
            biased;
            () = shutdown.cancelled() => return Err("ASB startup cancelled".to_owned()),
            result = bounded_worker(deadline, "Zenoh session startup", move || {
                zenoh::open(zenoh_config).wait()
                    .map(|session| Arc::new(BlockingDrop::new(session)))
                    .map_err(|e| format!("Zenoh session startup: {e}"))
            }) => result?,
        };
        let startup = async {
            let routers = timeout(
                deadline,
                wait_for_stable_router(&session, poll_interval, deadline),
            )
            .await
            .map_err(|_| "ASB router readiness deadline".to_owned())??;
            let worker_session = Arc::clone(&session);
            let subscriber = bounded_worker(deadline, "ASB subscriber startup", move || {
                worker_session
                    .declare_subscriber(SUBSCRIBER_KEY)
                    .wait()
                    .map(BlockingDrop::new)
                    .map_err(|e| format!("ASB subscriber startup: {e}"))
            })
            .await?;
            Ok((routers, subscriber))
        };
        let subscriber_result = tokio::select! {
            biased;
            () = shutdown.cancelled() => Err("ASB startup cancelled".to_owned()),
            result = startup => result,
        };
        let (routers, subscriber) = match subscriber_result {
            Ok(resources) => resources,
            Err(message) => {
                return match close_session(session, cleanup_deadline).await {
                    Ok(()) => Err(message),
                    Err(cleanup) => Err(format!("{message}; {cleanup}")),
                };
            }
        };
        let state = Arc::new(RunState {
            failure: OnceLock::new(),
            stop: shutdown.child_token(),
            shutdown,
        });
        let receiver_state = Arc::clone(&state);
        let receiver_schema = Arc::clone(&schema);
        let receiver = tokio::spawn(async move {
            loop {
                let sample = tokio::select! {
                    biased;
                    () = receiver_state.stop.cancelled() => break,
                    result = subscriber.recv_async() => match result {
                        Ok(sample) => sample,
                        Err(e) => {
                            receiver_state.fail(format!("ASB subscriber failed: {e}"));
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
                let result = tokio::select! {
                    biased;
                    () = receiver_state.stop.cancelled() => break,
                    result = timeout(operation_deadline, assessment) => result,
                };
                match result {
                    Ok(Ok(Ok((topic, raw_json)))) => {
                        tokio::select! {
                            biased;
                            () = receiver_state.stop.cancelled() => break,
                            () = router.publish(&topic, &topic, &raw_json) => {}
                        }
                    }
                    Ok(Ok(Err(details))) => warn!(details, "discarding invalid ASB sample"),
                    Ok(Err(e)) => receiver_state.fail(format!("ASB receive worker failed: {e}")),
                    Err(_) => receiver_state
                        .fail("ASB receive assessment deadline; sample not forwarded".to_owned()),
                }
            }
            bounded_worker(cleanup_deadline, "ASB subscriber undeclare", move || {
                subscriber
                    .into_inner()
                    .undeclare()
                    .wait()
                    .map_err(|e| format!("ASB subscriber cleanup: {e}"))
            })
            .await
        });
        let observer_session = Arc::clone(&session);
        let observer_state = Arc::clone(&state);
        let observer = tokio::spawn(async move {
            let mut interval = time::interval(poll_interval);
            interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
            loop {
                tokio::select! {
                    biased;
                    () = observer_state.stop.cancelled() => break,
                    _ = interval.tick() => {}
                }
                let snapshot = tokio::select! {
                    biased;
                    () = observer_state.stop.cancelled() => break,
                    result = router_snapshot(Arc::clone(&observer_session), operation_deadline) => result,
                };
                match snapshot {
                    Ok(current) if current == routers => {}
                    Ok(current) => {
                        observer_state.fail(format!(
                            "ASB router loss: startup ZIDs {routers:?}, observed {current:?}"
                        ));
                        break;
                    }
                    Err(message) => {
                        observer_state.fail(message);
                        break;
                    }
                }
            }
        });
        info!(key = SUBSCRIBER_KEY, "ASB subscriber established");
        Ok(Self {
            session: Some(session),
            schema,
            operation_deadline,
            cleanup_deadline,
            state,
            receiver,
            observer,
        })
    }

    /// Validate, encode original bytes and finish Zenoh publication. This async
    /// operation owns offload and its deadline; timeout does not prove absence
    /// of publication. Authorization remains at the OWP frontend.
    pub async fn publish(&self, topic: String, raw_json: String) -> Result<(), (OwpError, String)> {
        if let Some(error) = self.state.publication_error() {
            return Err(error);
        }
        let session = Arc::clone(self.session.as_ref().expect("running session"));
        let schema = Arc::clone(&self.schema);
        let state = Arc::clone(&self.state);
        let work = tokio::task::spawn_blocking(move || {
            if let Some(error) = state.publication_error() {
                return Err(error);
            }
            assess(&schema, &topic, &raw_json).map_err(|e| (OwpError::InvalidMessage, e))?;
            let bytes = OmsJsonEnvelope {
                oms_json: raw_json.into_bytes(),
            }
            .encode_to_vec();
            if let Some(error) = state.publication_error() {
                return Err(error);
            }
            let result = session
                .put(format!("{KEY_PREFIX}{topic}"), bytes)
                .congestion_control(zenoh::qos::CongestionControl::Block)
                .wait()
                .map_err(|e| {
                    (
                        OwpError::InternalError,
                        format!("Zenoh publication failed: {e}"),
                    )
                });
            if let Err((_, message)) = &result {
                state.fail(message.clone());
            }
            result
        });
        let result = tokio::select! {
            biased;
            () = self.state.stop.cancelled() => {
                let (error, message) = self.state.publication_error().expect("stopped");
                return Err((error, format!("{message}; started publication outcome uncertain")));
            },
            result = timeout(self.operation_deadline, work) => result,
        }
        .map_err(|_| {
            (
                OwpError::InternalError,
                "ASB publication deadline; publication outcome uncertain".to_owned(),
            )
        })
        .and_then(|result| {
            result.map_err(|e| {
                (
                    OwpError::InternalError,
                    format!("ASB publication worker failed: {e}"),
                )
            })
        })
        .and_then(std::convert::identity);
        if let Err((OwpError::InternalError, message)) = &result {
            self.state.fail(message.clone());
        }
        // A successful SDK put must not acknowledge success after observed loss.
        if let Some(error) = self.state.publication_error() {
            return Err(error);
        }
        result
    }

    /// First terminal infrastructure failure; retained through reconnect/cleanup.
    pub fn failure(&self) -> Option<String> {
        self.state.failure.get().cloned()
    }

    /// Stop local delivery, undeclare the startup subscriber and close the
    /// session with bounded waits. No per-client operation owns these resources.
    pub async fn close(&mut self) -> Result<(), String> {
        self.state.stop.cancel();
        let observer_result =
            finish_task(&mut self.observer, self.cleanup_deadline, "router observer").await;
        let receiver_result = finish_task(&mut self.receiver, self.cleanup_deadline, "subscriber")
            .await
            .and_then(std::convert::identity);
        // Always attempt session cleanup, including subscriber/observer failures.
        let session_result = if let Some(session) = self.session.take() {
            close_session(session, self.cleanup_deadline).await
        } else {
            Ok(())
        };
        let errors: Vec<_> = [observer_result, receiver_result, session_result]
            .into_iter()
            .filter_map(Result::err)
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
}

impl Drop for Adapter {
    fn drop(&mut self) {
        self.state.stop.cancel();
        self.receiver.abort();
        self.observer.abort();
    }
}

async fn close_session(session: SessionHandle, deadline: Duration) -> Result<(), String> {
    bounded_worker(deadline, "ASB session cleanup", move || {
        let result = session
            .close()
            .wait()
            .map_err(|e| format!("ASB session cleanup: {e}"));
        // Dispose of the last owned raw session here when possible. If another
        // started worker still owns it, BlockingDrop handles its eventual release.
        if let Ok(resource) = Arc::try_unwrap(session) {
            drop(resource.into_inner());
        }
        result
    })
    .await
}

async fn finish_task<T>(
    task: &mut JoinHandle<T>,
    deadline: Duration,
    resource: &str,
) -> Result<T, String> {
    if let Ok(result) = timeout(deadline, &mut *task).await {
        result.map_err(|e| format!("ASB {resource} task: {e}"))
    } else {
        task.abort();
        // Bound reclamation too; never join a started blocking worker here.
        let _ = timeout(deadline, task).await;
        Err(format!("ASB {resource} cleanup deadline"))
    }
}

async fn router_snapshot(
    session: SessionHandle,
    deadline: Duration,
) -> Result<Vec<zenoh::session::ZenohId>, String> {
    bounded_worker(deadline, "ASB router observation", move || {
        // Iterator traversal also queries transport state, so collect it here.
        let mut routers: Vec<_> = session.info().routers_zid().wait().collect();
        routers.sort_unstable();
        Ok(routers)
    })
    .await
}

async fn wait_for_stable_router(
    session: &SessionHandle,
    poll_interval: Duration,
    deadline: Duration,
) -> Result<Vec<zenoh::session::ZenohId>, String> {
    let mut previous = Vec::new();
    loop {
        let current = router_snapshot(Arc::clone(session), deadline).await?;
        if !current.is_empty() && current == previous {
            info!(routers = ?current, "stable ASB router readiness");
            return Ok(current);
        }
        previous = current;
        time::sleep(poll_interval).await;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn worker_deadline_returns_while_synchronous_work_is_still_blocked() {
        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
        let (finished_tx, mut finished_rx) = tokio::sync::oneshot::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let mut operation = tokio::spawn(bounded_worker(
            Duration::from_millis(20),
            "test operation",
            move || {
                let _ = started_tx.send(());
                // A finite backstop also releases the worker if an assertion fails.
                let _ = release_rx.recv_timeout(Duration::from_secs(1));
                let _ = finished_tx.send(());
                Ok(())
            },
        ));
        started_rx.await.unwrap();
        let outcome = timeout(Duration::from_millis(200), &mut operation).await;
        let still_running = finished_rx.try_recv().is_err();
        let _ = release_tx.send(());
        finished_rx.await.unwrap();
        assert!(
            still_running,
            "deadline must return before the started worker finishes"
        );
        assert_eq!(
            outcome.unwrap().unwrap().unwrap_err(),
            "test operation deadline"
        );
    }

    struct BlockingDestructor {
        started: Option<tokio::sync::oneshot::Sender<std::thread::ThreadId>>,
        release: std::sync::Mutex<std::sync::mpsc::Receiver<()>>,
    }

    impl Drop for BlockingDestructor {
        fn drop(&mut self) {
            let _ = self
                .started
                .take()
                .unwrap()
                .send(std::thread::current().id());
            let _ = self
                .release
                .get_mut()
                .unwrap()
                .recv_timeout(Duration::from_secs(1));
        }
    }

    #[tokio::test]
    async fn last_resource_owner_drop_keeps_the_async_thread_responsive() {
        let async_thread = std::thread::current().id();
        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let resource = Arc::new(BlockingDrop::new(BlockingDestructor {
            started: Some(started_tx),
            release: std::sync::Mutex::new(release_rx),
        }));
        let other_owner = Arc::clone(&resource);
        drop(resource);
        let start = std::time::Instant::now();
        drop(other_owner);
        let drop_thread = timeout(Duration::from_millis(200), started_rx).await;
        let elapsed = start.elapsed();
        let _ = release_tx.send(());
        assert!(
            elapsed < Duration::from_millis(200),
            "last-owner Drop must not wait on synchronous cleanup"
        );
        assert_ne!(drop_thread.unwrap().unwrap(), async_thread);
    }

    #[tokio::test]
    async fn timed_out_acquisition_disposes_its_late_resource() {
        let (worker_started_tx, worker_started_rx) = tokio::sync::oneshot::channel();
        let (worker_release_tx, worker_release_rx) = std::sync::mpsc::channel();
        let (drop_started_tx, drop_started_rx) = tokio::sync::oneshot::channel();
        let (drop_release_tx, drop_release_rx) = std::sync::mpsc::channel();
        let mut operation = tokio::spawn(bounded_worker(
            Duration::from_millis(20),
            "test acquisition",
            move || {
                let _ = worker_started_tx.send(());
                let _ = worker_release_rx.recv_timeout(Duration::from_secs(1));
                Ok(BlockingDrop::new(BlockingDestructor {
                    started: Some(drop_started_tx),
                    release: std::sync::Mutex::new(drop_release_rx),
                }))
            },
        ));
        worker_started_rx.await.unwrap();
        let outcome = timeout(Duration::from_millis(200), &mut operation).await;
        let _ = worker_release_tx.send(());
        let cleanup = timeout(Duration::from_millis(200), drop_started_rx).await;
        let _ = drop_release_tx.send(());
        assert!(
            matches!(outcome.unwrap().unwrap(), Err(message) if message == "test acquisition deadline")
        );
        assert!(
            cleanup.unwrap().is_ok(),
            "a resource acquired after timeout must still be disposed"
        );
    }

    async fn open_router(config: zenoh::Config) -> SessionHandle {
        bounded_worker(Duration::from_secs(5), "test router startup", move || {
            zenoh::open(config)
                .wait()
                .map(|session| Arc::new(BlockingDrop::new(session)))
                .map_err(|e| e.to_string())
        })
        .await
        .unwrap()
    }

    async fn setup() -> (ServerConfig, zenoh::Config, SessionHandle) {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);
        let router_config = zenoh::Config::from_json5(&format!(
            r#"{{ mode: "router", listen: {{ endpoints: ["tcp/{addr}"] }}, scouting: {{ multicast: {{ enabled: false }}, gossip: {{ enabled: false }} }} }}"#
        )).unwrap();
        let router_session = open_router(router_config.clone()).await;
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let temp = tempfile::tempdir().unwrap();
        let config_path = temp.path().join("server.toml");
        let text = std::fs::read_to_string(root.join("config/server.toml"))
            .unwrap()
            .replace(
                "spec/002.5.0/",
                &format!("{}/spec/002.5.0/", root.display()),
            )
            .replace(
                "config/services.d",
                &format!("{}/config/services.d", root.display()),
            );
        std::fs::write(&config_path, text).unwrap();
        let mut config = crate::config::load(&config_path).unwrap().server;
        config.zenoh_router = format!("tcp/{addr}");
        (config, router_config, router_session)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn detected_router_loss_latches_failure_and_rejects_publication() {
        let (config, router_config, router_session) = setup().await;
        let shutdown = CancellationToken::new();
        let mut adapter = Adapter::open(
            &config,
            Arc::new(Schema::new()),
            Arc::new(Router::new()),
            shutdown.clone(),
        )
        .await
        .unwrap();
        assert!(adapter.failure().is_none());

        close_session(router_session, Duration::from_secs(5))
            .await
            .unwrap();
        timeout(Duration::from_secs(5), shutdown.cancelled())
            .await
            .expect("router loss must stop the run");
        let failure = adapter.failure().expect("failure remains observable");
        let (error, details) = adapter
            .publish("ServiceStatus".to_owned(), "not JSON".to_owned())
            .await
            .unwrap_err();
        assert_eq!(error, OwpError::InternalError);
        assert!(details.contains(&failure));
        // Restore the endpoint while the original adapter remains alive. The
        // latched failure must survive SDK reconnect and cannot admit a new PUB.
        let restored_router = open_router(router_config).await;
        assert_eq!(adapter.failure(), Some(failure.clone()));
        assert_eq!(
            adapter
                .publish("ServiceStatus".to_owned(), "not JSON".to_owned())
                .await
                .unwrap_err()
                .0,
            OwpError::InternalError
        );
        adapter.close().await.unwrap();
        assert_eq!(adapter.failure(), Some(failure));
        close_session(restored_router, Duration::from_secs(5))
            .await
            .unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn validation_rejection_is_nonterminal_and_close_stops_publication() {
        let (config, _, router_session) = setup().await;
        let shutdown = CancellationToken::new();
        let mut adapter = Adapter::open(
            &config,
            Arc::new(Schema::new()),
            Arc::new(Router::new()),
            shutdown.clone(),
        )
        .await
        .unwrap();
        assert_eq!(
            adapter
                .publish("ServiceStatus".to_owned(), "not JSON".to_owned())
                .await
                .unwrap_err()
                .0,
            OwpError::InvalidMessage
        );
        assert!(adapter.failure().is_none());
        assert!(!shutdown.is_cancelled());
        adapter.close().await.unwrap();
        assert_eq!(
            adapter
                .publish("ServiceStatus".to_owned(), "not JSON".to_owned())
                .await
                .unwrap_err()
                .0,
            OwpError::InternalError
        );
        assert!(
            adapter.failure().is_none(),
            "requested closure is not a failed run"
        );
        close_session(router_session, Duration::from_secs(5))
            .await
            .unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn partial_startup_readiness_deadline_finishes_cleanup() {
        let (mut config, _, router_session) = setup().await;
        config.asb_startup_timeout_ms = 1_000;
        config.asb_router_poll_interval_ms = 60_000;
        let result = timeout(
            Duration::from_secs(3),
            Adapter::open(
                &config,
                Arc::new(Schema::new()),
                Arc::new(Router::new()),
                CancellationToken::new(),
            ),
        )
        .await
        .expect("partial startup and session cleanup must finish");
        let Err(error) = result else {
            panic!("startup cannot acquire two stable snapshots before the deadline");
        };
        assert!(error.contains("router readiness deadline"), "{error}");
        config.asb_router_poll_interval_ms = 100;
        let mut fresh = Adapter::open(
            &config,
            Arc::new(Schema::new()),
            Arc::new(Router::new()),
            CancellationToken::new(),
        )
        .await
        .expect("a fresh run can start after partial-startup cleanup");
        fresh.close().await.unwrap();
        close_session(router_session, Duration::from_secs(5))
            .await
            .unwrap();
    }
}
