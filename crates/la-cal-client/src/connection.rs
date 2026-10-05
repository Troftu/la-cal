//! Private OWP negotiation, serialized operation attribution and independent reception.
use std::collections::HashMap;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use owp_types::owp::{InfoPayload, ServerOp, parse_server, validate_identifier};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot, watch};
use tokio::time::{Instant, timeout, timeout_at};
use tokio_tungstenite::tungstenite::{Message as Frame, client::IntoClientRequest};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::{ClientConfig, ClientError, ConnectionFailure, Delivery, ReceiveError};
use ConnectionFailure::{Deadline, Protocol, Transport};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;
type Reply = oneshot::Sender<Result<(), ClientError>>;
type Registrations = HashMap<String, mpsc::UnboundedSender<Delivery>>;

pub enum Operation {
    Publish(String),
    Subscribe {
        sid: String,
        topic: &'static str,
        delivery: mpsc::UnboundedSender<Delivery>,
    },
    Unsubscribe(String),
}
pub struct Command {
    pub operation: Operation,
    pub reply: Reply,
}

pub async fn connect(config: &ClientConfig) -> Result<Socket, ClientError> {
    if !validate_identifier(&config.service_id)
        || [
            config.connection_timeout,
            config.operation_timeout,
            config.close_timeout,
        ]
        .iter()
        .any(|duration| duration.is_zero() || Instant::now().checked_add(*duration).is_none())
    {
        return Err(ClientError::Configuration(
            "valid participant and nonzero representable deadlines required".into(),
        ));
    }
    let mut request = config
        .endpoint
        .as_str()
        .into_client_request()
        .map_err(|e| ClientError::Configuration(e.to_string()))?;
    let loopback = request.uri().host().is_some_and(|host| {
        host == "localhost"
            || host
                .trim_matches(['[', ']'])
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
    });
    if request.uri().scheme_str() != Some("ws") || !loopback {
        return Err(ClientError::Configuration(
            "endpoint must be plain loopback ws".into(),
        ));
    }
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        "owp".parse().expect("fixed header"),
    );
    timeout(config.connection_timeout, async {
        let (mut socket, response) = tokio_tungstenite::connect_async(request).await
            .map_err(|e| ClientError::Negotiation(e.to_string()))?;
        if response.headers().get("Sec-WebSocket-Protocol").is_none_or(|h| h != "owp") {
            return Err(ClientError::Negotiation("server did not select owp".into()));
        }
        let init = serde_json::json!({"versions":["1.0"], "schema":"002.5.0", "verbose":true, "service_id":config.service_id});
        socket.send(Frame::text(format!("INIT {init}"))).await.map_err(|e| ClientError::Negotiation(e.to_string()))?;
        match read(&mut socket).await.map_err(|e| ClientError::Negotiation(e.to_string()))? {
            ServerOp::Ok => (),
            ServerOp::Err { error, details } => return Err(ClientError::Rejected { error, details }),
            _ => return Err(ClientError::Negotiation("expected INIT acknowledgment before INFO".into())),
        }
        match read(&mut socket).await.map_err(|e| ClientError::Negotiation(e.to_string()))? {
            ServerOp::Info(info) if valid_info(&info) => Ok(socket),
            ServerOp::Err { error, details } => Err(ClientError::Rejected { error, details }),
            _ => Err(ClientError::Negotiation("expected selected OWP 1.0 INFO".into())),
        }
        // INFO has no schema field in Rev B Table 5.1-5. Successful INIT accepts
        // the requested 002.5.0 schema; do not invent an INFO schema extension.
    }).await.map_err(|_| ClientError::Negotiation("connection deadline expired".into()))?
}

fn valid_info(info: &InfoPayload) -> bool {
    info.version == "1.0" && validate_identifier(&info.server_id)
}

async fn read(socket: &mut Socket) -> Result<ServerOp, ClientError> {
    loop {
        match socket.next().await {
            Some(Ok(Frame::Text(text))) => {
                return parse_server(&text).map_err(|e| failure(Protocol, &e.to_string(), false));
            }
            Some(Ok(Frame::Ping(_) | Frame::Pong(_))) => {
                socket
                    .flush()
                    .await
                    .map_err(|e| failure(Transport, &e.to_string(), false))?;
            }
            Some(Ok(Frame::Close(_))) | None => {
                return Err(failure(Transport, "unexpected connection closure", false));
            }
            Some(Ok(_)) => {
                return Err(failure(
                    Protocol,
                    "OWP requires textual WebSocket messages",
                    false,
                ));
            }
            Some(Err(e)) => return Err(failure(Transport, &e.to_string(), false)),
        }
    }
}

struct Pending {
    command: Command,
    deadline: Instant,
}

fn failure(kind: ConnectionFailure, reason: &str, transmitted: bool) -> ClientError {
    ClientError::Connection {
        kind,
        reason: reason.into(),
        transmitted,
    }
}

fn transmission(error: ClientError, transmitted: bool) -> ClientError {
    match error {
        ClientError::Connection { kind, reason, .. } => failure(kind, &reason, transmitted),
        other => other,
    }
}

fn prepare(operation: &Operation, registrations: &mut Registrations) -> String {
    match operation {
        Operation::Publish(frame) => frame.clone(),
        Operation::Subscribe {
            sid,
            topic,
            delivery,
        } => {
            registrations.insert(sid.clone(), delivery.clone());
            format!("SUB {sid} {topic} {topic}")
        }
        Operation::Unsubscribe(sid) => {
            registrations.remove(sid);
            format!("UNSUB {sid}")
        }
    }
}

fn acknowledge(
    pending: Pending,
    result: Result<(), ClientError>,
    registrations: &mut Registrations,
) {
    if result.is_err()
        && let Operation::Subscribe { sid, .. } = &pending.command.operation
    {
        registrations.remove(sid);
    }
    let _ = pending.command.reply.send(result);
}

#[allow(clippy::too_many_lines)] // One connection-level attribution state machine.
pub async fn run(
    mut socket: Socket,
    mut commands: mpsc::UnboundedReceiver<Command>,
    mut shutdown: watch::Receiver<Option<Instant>>,
    finished: watch::Sender<Option<Result<(), ClientError>>>,
    operation_timeout: Duration,
) {
    let mut registrations = Registrations::new();
    let mut pending: Option<Pending> = None;
    let mut close_deadline = None;
    let mut shutdown_open = true;
    let result = loop {
        let deadline = pending
            .as_ref()
            .map(|p| p.deadline)
            .into_iter()
            .chain(close_deadline)
            .min();
        tokio::select! {
            biased;
            changed = shutdown.changed(), if close_deadline.is_none() && shutdown_open => {
                if changed.is_ok() {
                    close_deadline = *shutdown.borrow_and_update();
                    commands.close();
                }
                else { shutdown_open = false; }
                // Last owner drop closes the command channel; no ownership cycle.
            }
            () = wait_deadline(deadline) => {
                break Err(failure(Deadline, if close_deadline.is_some_and(|d| Instant::now() >= d) { "close/drain deadline expired" } else { "acknowledgment deadline expired" }, pending.is_some()));
            }
            incoming = read(&mut socket) => {
                match incoming {
                    Ok(ServerOp::Msg { sid, payload }) => {
                        if let Some(delivery) = registrations.get(&sid) { let _ = delivery.send(Ok(payload)); }
                        // Late MSG after local unsubscribe/drop is discarded.
                    }
                    Ok(ServerOp::Ok) => {
                        let Some(operation) = pending.take() else { break Err(failure(Protocol, "unexpected +OK without pending operation", false)); };
                        acknowledge(operation, Ok(()), &mut registrations);
                    }
                    Ok(ServerOp::Err { error, details }) => {
                        let Some(operation) = pending.take() else { break Err(failure(Protocol, "unexpected -ERR without pending operation", false)); };
                        acknowledge(operation, Err(ClientError::Rejected { error, details }), &mut registrations);
                    }
                    Ok(ServerOp::Info(info)) if valid_info(&info) => {},
                    Ok(_) => break Err(failure(Protocol, "unsupported server operation/profile", pending.is_some())),
                    Err(error) => break Err(transmission(error, pending.is_some())),
                }
            }
            command = commands.recv(), if pending.is_none() => {
                let Some(command) = command else { break Ok(()); };
                let frame = prepare(&command.operation, &mut registrations);
                let deadline = Instant::now() + operation_timeout;
                pending = Some(Pending { command, deadline });
                let write_deadline = close_deadline.map_or(deadline, |d| d.min(deadline));
                match timeout_at(write_deadline, socket.send(Frame::text(frame))).await {
                    Ok(Ok(())) => (),
                    Ok(Err(e)) => break Err(failure(Transport, &e.to_string(), true)),
                    Err(_) => break Err(failure(Deadline, "socket write deadline expired", true)),
                }
            }
        }
    };
    commands.close();
    if let Some(operation) = pending.take() {
        let error = result.clone().err().unwrap_or(ClientError::Closed);
        acknowledge(operation, Err(error), &mut registrations);
    }
    while let Ok(command) = commands.try_recv() {
        let error = result
            .as_ref()
            .err()
            .map_or(ClientError::Closed, |e| transmission(e.clone(), false));
        let _ = command.reply.send(Err(error));
    }
    if let Err(ClientError::Connection { kind, reason, .. }) = &result {
        for delivery in registrations.values() {
            let _ = delivery.send(Err(ReceiveError::Connection {
                kind: *kind,
                details: reason.clone(),
            }));
        }
    }
    drop(registrations);
    let deadline = close_deadline.unwrap_or_else(|| Instant::now() + operation_timeout);
    let closure = timeout_at(deadline, async {
        socket
            .close(None)
            .await
            .map_err(|e| failure(Transport, &e.to_string(), false))?;
        while let Some(frame) = socket.next().await {
            match frame {
                Ok(Frame::Close(_))
                | Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed) => return Ok(()),
                Err(e) => return Err(failure(Transport, &e.to_string(), false)),
                Ok(_) => (),
            }
        }
        Ok(())
    })
    .await
    .unwrap_or_else(|_| {
        Err(failure(
            Deadline,
            "WebSocket closure deadline expired",
            false,
        ))
    });
    finished.send_replace(Some(result.and(closure)));
}

async fn wait_deadline(deadline: Option<Instant>) {
    if let Some(deadline) = deadline {
        tokio::time::sleep_until(deadline).await;
    } else {
        std::future::pending::<()>().await;
    }
}
