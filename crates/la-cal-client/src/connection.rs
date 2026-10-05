//! Private OWP negotiation, serialized operation attribution and independent reception.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use owp_types::owp::{InfoPayload, ServerOp, parse_server, validate_identifier};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot, watch};
use tokio::time::{Instant, timeout, timeout_at};
use tokio_tungstenite::tungstenite::{
    Error as WsError, Message as Frame, client::IntoClientRequest,
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::{Admissions, ClientConfig, ClientError, ConnectionFailure, Delivery, ReceiveError};
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
            Some(Ok(Frame::Ping(_) | Frame::Pong(_))) => {
                socket
                    .flush()
                    .await
                    .map_err(|e| failure(Transport, &e.to_string(), false))?;
            }
            frame => return frame_to_operation(frame),
        }
    }
}

// Callers handle Ping/Pong separately according to their I/O phase.
fn frame_to_operation(frame: Option<Result<Frame, WsError>>) -> Result<ServerOp, ClientError> {
    match frame {
        Some(Ok(Frame::Text(text))) => {
            parse_server(&text).map_err(|e| failure(Protocol, &e.to_string(), false))
        }
        Some(Ok(Frame::Close(_))) | None => {
            Err(failure(Transport, "unexpected connection closure", false))
        }
        Some(Ok(_)) => Err(failure(
            Protocol,
            "OWP requires textual WebSocket messages",
            false,
        )),
        Some(Err(error)) => Err(failure(Transport, &error.to_string(), false)),
    }
}

struct Pending {
    command: Command,
    deadline: Instant,
    transmitted: bool,
    response: Option<Result<(), ClientError>>,
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
    socket: Socket,
    mut commands: mpsc::UnboundedReceiver<Command>,
    mut shutdown: watch::Receiver<Option<Instant>>,
    finished: watch::Sender<Option<Result<(), ClientError>>>,
    operation_timeout: Duration,
    close_timeout: Duration,
    admissions: Arc<Mutex<Admissions>>,
) {
    // Both halves share Tungstenite's framing and automatic control responses.
    // A blocked flush releases the split lock, allowing read progress. Never
    // await a control flush inside the reader: it can also be backpressured.
    let (mut writer, mut reader) = socket.split();
    let mut needs_flush = false;
    let mut registrations = Registrations::new();
    let mut pending: Option<Pending> = None;
    let mut close_deadline = None;
    let mut drain_rejection = None;
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
                break Err(failure(Deadline, if close_deadline.is_some_and(|d| Instant::now() >= d) { "close/drain deadline expired" } else if needs_flush { "socket write deadline expired" } else { "acknowledgment deadline expired" }, pending.as_ref().is_some_and(|p| p.transmitted)));
            }
            flushed = std::future::poll_fn(|cx| {
                // Before admitting another operation the previous flush has
                // completed, so polling this flush attempts its queued frame.
                if let Some(operation) = &mut pending { operation.transmitted = true; }
                writer.poll_flush_unpin(cx)
            }), if needs_flush => {
                if let Err(error) = flushed { break Err(failure(Transport, &error.to_string(), pending.as_ref().is_some_and(|p| p.transmitted))); }
                needs_flush = false;
            }
            incoming = reader.next() => {
                let incoming = match incoming {
                    Some(Ok(Frame::Ping(_))) => { needs_flush = true; continue; }
                    Some(Ok(Frame::Pong(_))) => continue,
                    frame => frame_to_operation(frame),
                };
                match incoming {
                    Ok(ServerOp::Msg { sid, payload }) => {
                        if let Some(delivery) = registrations.get(&sid) { let _ = delivery.send(Ok(payload)); }
                        // Late MSG after local unsubscribe/drop is discarded.
                    }
                    Ok(ServerOp::Ok) => {
                        if let Err(error) = record_response(&mut pending, Ok(())) { break Err(error); }
                    }
                    Ok(ServerOp::Err { error, details }) => {
                        let rejection = ClientError::Rejected { error, details };
                        if let Err(error) = record_response(&mut pending, Err(rejection.clone())) { break Err(error); }
                        if close_deadline.is_some() && drain_rejection.is_none() {
                            drain_rejection = Some(rejection);
                        }
                    }
                    Ok(ServerOp::Info(info)) if valid_info(&info) => {},
                    Ok(_) => break Err(failure(Protocol, "unsupported server operation/profile", pending.as_ref().is_some_and(|p| p.transmitted))),
                    Err(error) => break Err(transmission(error, pending.as_ref().is_some_and(|p| p.transmitted))),
                }
            }
            command = commands.recv(), if pending.is_none() && !needs_flush => {
                let Some(command) = command else { break drain_rejection.map_or(Ok(()), Err); };
                let frame = prepare(&command.operation, &mut registrations);
                let deadline = Instant::now() + operation_timeout;
                pending = Some(Pending { command, deadline, transmitted: false, response: None });
                // SplitSink's slot is empty after the preceding flush. feed only
                // queues this frame there; network writes occur in the flush
                // branch above, concurrently with reception and shutdown.
                if let Err(error) = writer.feed(Frame::text(frame)).await { break Err(failure(Transport, &error.to_string(), false)); }
                needs_flush = true;
            }
        }
        // A peer response may arrive before local flushing finishes. Retain both
        // write and response ownership until they complete; no new PUB/SUB/UNSUB
        // can overtake the write, and its deadline remains active throughout.
        if !needs_flush && pending.as_ref().is_some_and(|p| p.response.is_some()) {
            let mut operation = pending.take().expect("pending response");
            let response = operation.response.take().expect("recorded response");
            acknowledge(operation, response, &mut registrations);
        }
    };
    latch_failure(&admissions, &result);
    commands.close();
    if let Some(operation) = pending.take() {
        let error = result.clone().err().unwrap_or(ClientError::Closed);
        let transmitted = operation.transmitted;
        acknowledge(
            operation,
            Err(transmission(error, transmitted)),
            &mut registrations,
        );
    }
    while let Ok(command) = commands.try_recv() {
        let error = result
            .as_ref()
            .err()
            .map_or(ClientError::Closed, |e| transmission(e.clone(), false));
        let _ = command.reply.send(Err(error));
    }
    finish_streams_on_failure(&mut registrations, &result);
    // Reunite the original pair for bounded closure, retaining Tungstenite's
    // partial-write buffer and automatic controls; an unattempted sink slot drops.
    let mut socket = writer.reunite(reader).expect("same WebSocket split pair");
    let deadline = Instant::now() + close_timeout;
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
    // Keep the original terminal cause. A closure failure still takes precedence
    // over an ordinary drain rejection and must be visible to live streams.
    let result = match (&result, closure) {
        (Err(ClientError::Connection { .. }), _) => result,
        (_, Err(error)) => Err(error),
        _ => result,
    }
    .map_err(|e| transmission(e, false));
    latch_failure(&admissions, &result);
    finish_streams_on_failure(&mut registrations, &result);
    drop(registrations);
    finished.send_replace(Some(result));
}

fn record_response(
    pending: &mut Option<Pending>,
    response: Result<(), ClientError>,
) -> Result<(), ClientError> {
    let Some(operation) = pending else {
        return Err(failure(
            Protocol,
            "unexpected acknowledgment without pending operation",
            false,
        ));
    };
    if operation.response.is_some() {
        return Err(failure(
            Protocol,
            "duplicate acknowledgment for pending operation",
            operation.transmitted,
        ));
    }
    operation.response = Some(response);
    Ok(())
}

fn finish_streams_on_failure(registrations: &mut Registrations, result: &Result<(), ClientError>) {
    if let Err(ClientError::Connection { kind, reason, .. }) = result {
        for (_, delivery) in registrations.drain() {
            let _ = delivery.send(Err(ReceiveError::Connection {
                kind: *kind,
                details: reason.clone(),
            }));
        }
    }
}

fn latch_failure(admissions: &Mutex<Admissions>, result: &Result<(), ClientError>) {
    if let Err(error @ ClientError::Connection { .. }) = result {
        *admissions.lock().expect("admission lock") =
            Admissions::Failed(transmission(error.clone(), false));
    }
}

async fn wait_deadline(deadline: Option<Instant>) {
    if let Some(deadline) = deadline {
        tokio::time::sleep_until(deadline).await;
    } else {
        std::future::pending::<()>().await;
    }
}
