//! Cloneable typed OWP client for OMS 2.5 / UCI 002.5.0.
//!
//! Applications own persistent stream consumption, processing and response matching.
//! `send` success is an OWP Publication Acknowledgment only.
//!
//! ```no_run
//! use futures_util::StreamExt;
//! use la_cal_client::{Client, ClientConfig, messages::uci_002_5::ServiceStatus};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::connect(ClientConfig::new("ws://127.0.0.1:9000/", "baseline-tester")).await?;
//! let mut reports = client.subscribe::<ServiceStatus>().await?;
//! let report: ServiceStatus = serde_json::from_str(include_str!("../../../fixtures/service-status.json"))?;
//! client.clone().send(&report).await?;
//! if let Some(item) = reports.next().await {
//!     let report = item?;
//!     println!("{:?}", report.message.message_data.service_state);
//! }
//! reports.close().await?;
//! client.close().await?;
//! # Ok(()) }
//! ```

mod connection;
pub mod messages;

use std::pin::Pin;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::Stream;
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::{mpsc, oneshot, watch};
use tokio::time::Instant;

use connection::{Command, Operation};
pub use owp_types::owp::OwpError;

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// A generated, publishable global UCI root. Internal XSD types do not implement this trait.
///
/// ```compile_fail
/// use la_cal_client::{Message, messages::uci_002_5::ServiceStatusMt};
/// fn publishable<T: Message>() {}
/// publishable::<ServiceStatusMt>(); // A body is not an OMS JSON global root.
/// ```
pub trait Message:
    sealed::Sealed + Serialize + DeserializeOwned + Send + Sync + Unpin + 'static
{
    /// OMS JSON global element name.
    const ROOT: &'static str;
    /// OWP topic inferred from the root.
    const TOPIC: &'static str;
    /// Pinned UCI schema profile.
    const PROFILE: &'static str;
}

/// Loopback connection identity and per-stage deadlines.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Plain loopback WebSocket URL (no TLS backend).
    pub endpoint: String,
    /// Authorized OWP participant identifier, distinct from application readiness.
    pub service_id: String,
    /// Whole WebSocket / INIT / INFO establishment deadline.
    pub connection_timeout: Duration,
    /// Deadline for each transmitted acknowledgment-tracked operation.
    pub operation_timeout: Duration,
    /// Deadline per cleanup wait: drain admitted operations, then WebSocket closure.
    /// Each wait gets its own bound, including cleanup after drain failure.
    pub close_timeout: Duration,
}

impl ClientConfig {
    /// Configure a participant with five-second per-stage defaults.
    pub fn new(endpoint: impl Into<String>, service_id: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            service_id: service_id.into(),
            connection_timeout: Duration::from_secs(5),
            operation_timeout: Duration::from_secs(5),
            close_timeout: Duration::from_secs(5),
        }
    }
}

/// Attributable client operation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    /// Invalid endpoint, participant or deadline configuration.
    Configuration(String),
    /// WebSocket or OWP establishment failed.
    Negotiation(String),
    /// Local typed OMS JSON encoding failed; nothing was transmitted.
    Encoding(String),
    /// The server rejected the operation; its exact code and details are retained.
    Rejected {
        /// Standard OWP error code.
        error: OwpError,
        /// Unmodified server diagnostic details.
        details: Option<String>,
    },
    /// Deadline, transport or protocol failure.
    Connection {
        /// Whether the failure was a transport, protocol or deadline failure.
        kind: ConnectionFailure,
        /// Cause of failure.
        reason: String,
        /// Whether transmission was attempted. A publication's outcome is uncertain when true.
        transmitted: bool,
    },
    /// Shared connection is gracefully closed or closing; this operation was not transmitted.
    /// Failed connections instead retain their terminal connection cause.
    Closed,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Configuration(s) => write!(f, "invalid configuration: {s}"),
            Self::Negotiation(s) => write!(f, "connection negotiation: {s}"),
            Self::Encoding(s) => write!(f, "encoding: {s}"),
            Self::Rejected { error, details } => write!(f, "OWP rejection: {error} {details:?}"),
            Self::Connection {
                reason,
                transmitted,
                kind,
            } => write!(
                f,
                "{kind:?} failure (transmission attempted: {transmitted}): {reason}"
            ),
            Self::Closed => f.write_str("connection is closed or closing"),
        }
    }
}
impl std::error::Error for ClientError {}

/// Machine-readable classification of a shared connection failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionFailure {
    /// Underlying WebSocket/TCP failure or unexpected closure.
    Transport,
    /// Malformed OWP framing, unsupported profile or unsolicited acknowledgment.
    Protocol,
    /// A configured operation, drain or closure deadline expired.
    Deadline,
}

/// A nonterminal typed decoding failure or terminal connection failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiveError {
    /// This payload could not decode to the subscription's root; subsequent messages remain available.
    Decoding {
        /// Expected generated global root.
        root: &'static str,
        /// Typed decoding details.
        details: String,
    },
    /// Terminal transport/protocol/deadline failure, followed by stream end.
    Connection {
        /// Machine-readable failure category.
        kind: ConnectionFailure,
        /// Connection-level cause.
        details: String,
    },
}
impl std::fmt::Display for ReceiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decoding { root, details } => write!(f, "cannot decode {root}: {details}"),
            Self::Connection { kind, details } => write!(f, "{kind:?} failure: {details}"),
        }
    }
}
impl std::error::Error for ReceiveError {}

struct Shared {
    // The admission lock gives close one boundary across all clones.
    admissions: Arc<Mutex<Admissions>>,
    commands: mpsc::UnboundedSender<Command>,
    shutdown: watch::Sender<Option<Instant>>,
    finished: watch::Receiver<Option<Result<(), ClientError>>>,
    next_sid: AtomicU64,
    close_timeout: Duration,
}

pub(crate) enum Admissions {
    Open,
    Closing,
    Failed(ClientError),
}

/// Cloneable typed client. Clones share one OWP connection and operation attribution queue.
#[derive(Clone)]
pub struct Client {
    shared: Arc<Shared>,
}

impl Client {
    /// Return only after WebSocket `owp`, verbose INIT success and OWP 1.0 INFO.
    pub async fn connect(config: ClientConfig) -> Result<Self, ClientError> {
        let socket = connection::connect(&config).await?;
        let (commands, receiver) = mpsc::unbounded_channel();
        let (shutdown, shutdown_receiver) = watch::channel(None);
        let (finished_sender, finished) = watch::channel(None);
        let admissions = Arc::new(Mutex::new(Admissions::Open));
        tokio::spawn(connection::run(
            socket,
            receiver,
            shutdown_receiver,
            finished_sender,
            config.operation_timeout,
            config.close_timeout,
            admissions.clone(),
        ));
        Ok(Self {
            shared: Arc::new(Shared {
                admissions,
                commands,
                shutdown,
                finished,
                next_sid: AtomicU64::new(1),
                close_timeout: config.close_timeout,
            }),
        })
    }

    /// Complete on successful OWP Publication Acknowledgment, not application receipt or completion.
    ///
    /// Canceling the caller's wait after transmission does not cancel or retry the
    /// publication. Background I/O consumes its response before sending the next
    /// tracked operation. A missing acknowledgment fails the shared connection;
    /// `Connection { transmitted: true, .. }` leaves publication outcome uncertain.
    pub async fn send<T: Message>(&self, message: &T) -> Result<(), ClientError> {
        let json =
            serde_json::to_string(message).map_err(|e| ClientError::Encoding(e.to_string()))?;
        self.operation(Operation::Publish(format!("PUB {} {json}", T::TOPIC)))
            .await
    }

    /// Create an independent ordinary registration and persistent application-owned typed stream.
    pub async fn subscribe<T: Message>(&self) -> Result<Subscription<T>, ClientError> {
        let number = self
            .shared
            .next_sid
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| ClientError::Configuration("subscription identifiers exhausted".into()))?;
        let sid = format!("s{number}");
        let (delivery, receiver) = mpsc::unbounded_channel();
        // If subscribe is canceled, Drop queues cleanup even after a transmitted SUB.
        let mut subscription = Subscription {
            client: self.clone(),
            sid: Some(sid.clone()),
            receiver,
            marker: std::marker::PhantomData,
        };
        let result = self
            .operation(Operation::Subscribe {
                sid,
                topic: T::TOPIC,
                delivery,
            })
            .await;
        if let Err(error) = result {
            // A definite rejection creates no registration. Connection failures
            // dispose of every registration; only cancellation needs the Drop guard.
            subscription.sid = None;
            return Err(error);
        }
        Ok(subscription)
    }

    /// Stop admissions from every clone, drain admitted operations and close the shared connection.
    ///
    /// Drain and WebSocket closure each use `close_timeout`. A drain failure still
    /// progresses bounded cleanup. A rejection encountered during drain is returned
    /// after finishing admitted operations; terminal failure takes precedence.
    /// Canceling this waiter leaves shutdown running. All close waiters observe the
    /// same completion result; await final publications before starting close.
    pub async fn close(&self) -> Result<(), ClientError> {
        {
            let mut admitted = self.shared.admissions.lock().expect("admission lock");
            if matches!(*admitted, Admissions::Open) {
                *admitted = Admissions::Closing;
                drop(admitted);
                self.shared
                    .shutdown
                    .send_replace(Some(Instant::now() + self.shared.close_timeout));
            }
        }
        let mut finished = self.shared.finished.clone();
        loop {
            let result = finished.borrow().clone();
            if let Some(result) = result {
                return result;
            }
            finished.changed().await.map_err(|_| ClientError::Closed)?;
        }
    }

    fn enqueue(&self, command: Command) -> Result<(), ClientError> {
        let admitted = self.shared.admissions.lock().expect("admission lock");
        match &*admitted {
            Admissions::Open => (),
            Admissions::Closing => return Err(ClientError::Closed),
            Admissions::Failed(error) => return Err(error.clone()),
        }
        let result = self
            .shared
            .commands
            .send(command)
            .map_err(|_| ClientError::Closed);
        drop(admitted);
        result
    }

    async fn operation(&self, operation: Operation) -> Result<(), ClientError> {
        let (reply, result) = oneshot::channel();
        self.enqueue(Command { operation, reply })?;
        result.await.unwrap_or(Err(ClientError::Closed))
    }
}

type Delivery = Result<Arc<str>, ReceiveError>;

/// Persistent typed reception for exactly one connection-local OWP registration.
pub struct Subscription<T: Message> {
    client: Client,
    sid: Option<String>,
    receiver: mpsc::UnboundedReceiver<Delivery>,
    marker: std::marker::PhantomData<T>,
}

impl<T: Message> Subscription<T> {
    /// Consume this stream, stop local delivery and await its UNSUB acknowledgment.
    pub async fn close(mut self) -> Result<(), ClientError> {
        self.receiver.close();
        let sid = self.sid.take().expect("active subscription");
        self.client.operation(Operation::Unsubscribe(sid)).await
    }
}

impl<T: Message> Stream for Subscription<T> {
    type Item = Result<T, ReceiveError>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().receiver.poll_recv(cx).map(|item| {
            item.map(|result| {
                result.and_then(|json| {
                    serde_json::from_str(&json).map_err(|e| ReceiveError::Decoding {
                        root: T::ROOT,
                        details: e.to_string(),
                    })
                })
            })
        })
    }
}

impl<T: Message> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.receiver.close();
        if let Some(sid) = self.sid.take() {
            let (reply, _) = oneshot::channel();
            let _ = self.client.enqueue(Command {
                operation: Operation::Unsubscribe(sid),
                reply,
            });
        }
    }
}
