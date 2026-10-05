//! Behavioral checks at the approved public typed-client / OWP peer seam.
#![allow(clippy::result_large_err)] // Tungstenite's handshake callback requires this response type.
use futures_util::{SinkExt, StreamExt};
use la_cal_client::{Client, ClientConfig, messages::uci_002_5::ServiceStatus};
use la_cal_client::{ClientError, ConnectionFailure, ReceiveError};
use tokio::net::TcpListener;
use tokio::time::{Duration, timeout};
use tokio_tungstenite::tungstenite::Message;

type Peer = tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>;

async fn listener() -> (TcpListener, ClientConfig) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let config = ClientConfig::new(
        format!("ws://{}/", listener.local_addr().unwrap()),
        "tester",
    );
    (listener, config)
}

async fn handshake(listener: TcpListener) -> Peer {
    handshake_with_config(listener, None).await
}

async fn handshake_with_config(
    listener: TcpListener,
    config: Option<tokio_tungstenite::tungstenite::protocol::WebSocketConfig>,
) -> Peer {
    let (tcp, _) = listener.accept().await.unwrap();
    let mut ws = tokio_tungstenite::accept_hdr_async_with_config(
        tcp,
        |_: &tokio_tungstenite::tungstenite::handshake::server::Request,
         mut response: tokio_tungstenite::tungstenite::handshake::server::Response| {
            response
                .headers_mut()
                .insert("Sec-WebSocket-Protocol", "owp".parse().unwrap());
            Ok(response)
        },
        config,
    )
    .await
    .unwrap();
    assert!(text(&mut ws).await.starts_with("INIT "));
    ws.send(Message::text("+OK")).await.unwrap();
    ws.send(Message::text(INFO)).await.unwrap();
    ws
}

async fn text(ws: &mut Peer) -> String {
    ws.next()
        .await
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap()
        .to_string()
}

async fn finish(ws: &mut Peer) {
    assert!(matches!(
        ws.next().await.unwrap().unwrap(),
        Message::Close(_)
    ));
    ws.flush().await.unwrap();
}

const INFO: &str = r#"INFO {"version":"1.0","server_id":"peer","uuids":{"system":"123e4567-e89b-12d3-a456-426614174000","service":"123e4567-e89b-12d3-a456-426614174001"},"system_label":"test"}"#;
const STATUS: &str = include_str!("../../../fixtures/service-status.json");

#[tokio::test]
async fn canceled_sent_publication_owns_its_response_while_reception_stays_live() {
    timeout(Duration::from_secs(3), async {
        let (listener, config) = listener().await;
        let (sent, observed) = tokio::sync::oneshot::channel();
        let (proceed, ready) = tokio::sync::oneshot::channel();
        let peer = tokio::spawn(async move {
            let mut ws = handshake(listener).await;
            let sub = text(&mut ws).await;
            let sid = sub.split_whitespace().nth(1).unwrap().to_owned();
            ws.send(Message::text("+OK")).await.unwrap();
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            sent.send(()).unwrap();
            ready.await.unwrap();
            ws.send(Message::text(format!("MSG {sid} {STATUS}")))
                .await
                .unwrap();
            ws.send(Message::Ping(vec![7].into())).await.unwrap();
            assert_eq!(
                ws.next().await.unwrap().unwrap(),
                Message::Pong(vec![7].into())
            );
            assert!(
                timeout(Duration::from_millis(40), ws.next()).await.is_err(),
                "cancellation must not advance the queue, close or retry"
            );
            ws.send(Message::text(
                "-ERR Invalid-Message canceled publication only",
            ))
            .await
            .unwrap();
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            ws.send(Message::text("+OK")).await.unwrap();
            ws.send(Message::text(format!("MSG {sid} {STATUS}")))
                .await
                .unwrap();
            finish(&mut ws).await;
        });
        let client = Client::connect(config).await.unwrap();
        let mut reports = client.subscribe::<ServiceStatus>().await.unwrap();
        let sender = client.clone();
        let canceled = tokio::spawn(async move {
            sender
                .send(&serde_json::from_str::<ServiceStatus>(STATUS).unwrap())
                .await
        });
        observed.await.unwrap();
        canceled.abort();
        assert!(canceled.await.unwrap_err().is_cancelled());
        let status = serde_json::from_str::<ServiceStatus>(STATUS).unwrap();
        let next = client.send(&status);
        tokio::pin!(next);
        assert!(futures_util::poll!(&mut next).is_pending()); // Public call is admitted.
        proceed.send(()).unwrap();
        assert_eq!(reports.next().await.unwrap().unwrap(), status);
        next.await.unwrap(); // Must not receive the canceled PUB's rejection.
        assert_eq!(reports.next().await.unwrap().unwrap(), status);
        client.close().await.unwrap();
        assert!(reports.next().await.is_none());
        peer.await.unwrap();
    })
    .await
    .expect("cancellation retains attribution and independent reception");
}

#[tokio::test]
async fn acknowledgment_expiry_never_sends_queued_operations_and_reports_uncertain_outcome() {
    timeout(Duration::from_secs(3), async {
        let (listener, mut config) = listener().await;
        config.operation_timeout = Duration::from_millis(100);
        config.close_timeout = Duration::from_millis(500);
        let (sent, observed) = tokio::sync::oneshot::channel();
        let (late, expired) = tokio::sync::oneshot::channel();
        let peer = tokio::spawn(async move {
            let mut ws = handshake(listener).await;
            for _ in 0..2 {
                assert!(text(&mut ws).await.starts_with("SUB "));
                ws.send(Message::text("+OK")).await.unwrap();
            }
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            sent.send(()).unwrap();
            expired.await.unwrap();
            // This acknowledgment is too late; it cannot complete any queued call.
            ws.send(Message::text("+OK")).await.unwrap();
            finish(&mut ws).await; // The next frame must be Close, never another operation.
        });
        let client = Client::connect(config).await.unwrap();
        let clone = client.clone();
        let mut first = client.subscribe::<ServiceStatus>().await.unwrap();
        let mut second = clone.subscribe::<ServiceStatus>().await.unwrap();
        let status = serde_json::from_str::<ServiceStatus>(STATUS).unwrap();
        let pending = client.send(&status);
        tokio::pin!(pending);
        assert!(futures_util::poll!(&mut pending).is_pending());
        observed.await.unwrap();
        let queued = clone.send(&status);
        let queued_sub = clone.subscribe::<ServiceStatus>();
        tokio::pin!(queued, queued_sub);
        assert!(futures_util::poll!(&mut queued).is_pending());
        assert!(futures_util::poll!(&mut queued_sub).is_pending());
        let sent_error = pending.await.unwrap_err();
        let ClientError::Connection {
            kind: ConnectionFailure::Deadline,
            reason,
            transmitted: true,
        } = sent_error
        else {
            panic!("sent PUB outcome must be uncertain")
        };
        let unsent = ClientError::Connection {
            kind: ConnectionFailure::Deadline,
            reason: reason.clone(),
            transmitted: false,
        };
        assert_eq!(queued.await, Err(unsent.clone()));
        assert!(matches!(queued_sub.await, Err(error) if error == unsent));
        assert_eq!(clone.send(&status).await, Err(unsent.clone()));
        for stream in [&mut first, &mut second] {
            assert_eq!(
                timeout(Duration::from_millis(150), stream.next())
                    .await
                    .expect("terminal notification must not wait for peer cleanup"),
                Some(Err(ReceiveError::Connection {
                    kind: ConnectionFailure::Deadline,
                    details: reason.clone()
                }))
            );
            assert!(stream.next().await.is_none());
        }
        late.send(()).unwrap();
        assert_eq!(client.close().await, Err(unsent.clone()));
        assert_eq!(clone.close().await, Err(unsent));
        peer.await.unwrap();
    })
    .await
    .expect("ack expiry closes the shared connection without queue advancement");
}

#[tokio::test]
async fn any_clone_close_stops_admissions_drains_operations_and_reports_drain_rejection() {
    for rejected in [false, true] {
        timeout(Duration::from_secs(3), async {
            let (listener, config) = listener().await;
            let (sent, observed) = tokio::sync::oneshot::channel();
            let (drain, closing) = tokio::sync::oneshot::channel();
            let peer = tokio::spawn(async move {
                let mut ws = handshake(listener).await;
                let sub = text(&mut ws).await;
                let sid = sub.split_whitespace().nth(1).unwrap().to_owned();
                ws.send(Message::text("+OK")).await.unwrap();
                assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
                sent.send(()).unwrap();
                closing.await.unwrap();
                assert!(
                    timeout(Duration::from_millis(40), ws.next()).await.is_err(),
                    "close must drain, not interrupt the sent operation"
                );
                ws.send(Message::text(format!("MSG {sid} {STATUS}")))
                    .await
                    .unwrap();
                ws.send(Message::text(if rejected {
                    "-ERR Invalid-Message final publication denied"
                } else {
                    "+OK"
                }))
                .await
                .unwrap();
                assert!(
                    text(&mut ws).await.starts_with("PUB ServiceStatus "),
                    "already-admitted queued operation must drain even after an ordinary rejection"
                );
                ws.send(Message::text("+OK")).await.unwrap();
                finish(&mut ws).await;
            });
            let client = Client::connect(config).await.unwrap();
            let clone = client.clone();
            let mut reports = client.subscribe::<ServiceStatus>().await.unwrap();
            let status = serde_json::from_str::<ServiceStatus>(STATUS).unwrap();
            let first = client.send(&status);
            tokio::pin!(first);
            assert!(futures_util::poll!(&mut first).is_pending());
            observed.await.unwrap();
            let second = client.send(&status);
            tokio::pin!(second);
            assert!(futures_util::poll!(&mut second).is_pending());
            let teardown = clone.close();
            let other_close = client.close();
            tokio::pin!(teardown, other_close);
            assert!(futures_util::poll!(&mut teardown).is_pending());
            assert!(futures_util::poll!(&mut other_close).is_pending());
            assert_eq!(client.send(&status).await, Err(ClientError::Closed));
            assert!(matches!(
                clone.subscribe::<ServiceStatus>().await,
                Err(ClientError::Closed)
            ));
            drain.send(()).unwrap();
            assert_eq!(reports.next().await.unwrap().unwrap(), status);
            let expected = if rejected {
                Err(ClientError::Rejected {
                    error: la_cal_client::OwpError::InvalidMessage,
                    details: Some("final publication denied".into()),
                })
            } else {
                Ok(())
            };
            assert_eq!(first.await, expected);
            second.await.unwrap();
            assert_eq!(teardown.await, expected);
            assert_eq!(other_close.await, expected);
            assert!(reports.next().await.is_none());
            peer.await.unwrap();
        })
        .await
        .expect("shared close drains and finishes background I/O");
    }
}

#[tokio::test]
async fn drain_expiry_still_closes_websocket_after_close_waiter_is_canceled() {
    timeout(Duration::from_secs(2), async {
        let (listener, mut config) = listener().await;
        config.operation_timeout = Duration::from_secs(1);
        config.close_timeout = Duration::from_millis(100);
        let (sent, observed) = tokio::sync::oneshot::channel();
        let peer = tokio::spawn(async move {
            let mut ws = handshake(listener).await;
            assert!(text(&mut ws).await.starts_with("SUB "));
            ws.send(Message::text("+OK")).await.unwrap();
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            sent.send(()).unwrap();
            // Cleanup gets a fresh bounded wait after drain expiry, not an expired
            // deadline that suppresses even transmitting the WebSocket Close.
            finish(&mut ws).await;
        });
        let client = Client::connect(config).await.unwrap();
        let clone = client.clone();
        let mut reports = client.subscribe::<ServiceStatus>().await.unwrap();
        let status = serde_json::from_str::<ServiceStatus>(STATUS).unwrap();
        let pending = client.send(&status);
        tokio::pin!(pending);
        assert!(futures_util::poll!(&mut pending).is_pending());
        observed.await.unwrap();
        let queued = clone.send(&status);
        tokio::pin!(queued);
        assert!(futures_util::poll!(&mut queued).is_pending());
        {
            let teardown = clone.close();
            tokio::pin!(teardown);
            assert!(futures_util::poll!(&mut teardown).is_pending());
            // Cancellation of this waiter must not cancel shared shutdown.
        }
        assert_eq!(client.send(&status).await, Err(ClientError::Closed));
        let sent_error = pending.await.unwrap_err();
        let ClientError::Connection {
            kind: ConnectionFailure::Deadline,
            reason,
            transmitted: true,
        } = sent_error
        else {
            panic!("unfinished sent publication is uncertain")
        };
        assert_eq!(reason, "close/drain deadline expired");
        let unsent = ClientError::Connection {
            kind: ConnectionFailure::Deadline,
            reason: reason.clone(),
            transmitted: false,
        };
        assert_eq!(queued.await, Err(unsent.clone()));
        assert_eq!(
            reports.next().await,
            Some(Err(ReceiveError::Connection {
                kind: ConnectionFailure::Deadline,
                details: reason
            }))
        );
        assert!(reports.next().await.is_none());
        assert_eq!(client.close().await, Err(unsent));
        peer.await.unwrap();
    })
    .await
    .expect("drain failure does not prevent bounded cleanup");
}

#[tokio::test]
async fn close_deadline_also_bounds_an_in_progress_socket_write() {
    timeout(Duration::from_secs(4), async {
        let (listener, mut config) = listener().await;
        config.operation_timeout = Duration::from_secs(2);
        config.close_timeout = Duration::from_millis(100);
        let (writing, observed) = tokio::sync::oneshot::channel();
        let (release, released) = tokio::sync::oneshot::channel();
        let peer = tokio::spawn(async move {
            let ws = handshake(listener).await;
            // Observe transmission at the real TCP boundary without draining the
            // WebSocket message. The large typed payload forces write backpressure.
            let mut byte = [0];
            assert_eq!(ws.get_ref().peek(&mut byte).await.unwrap(), 1);
            writing.send(()).unwrap();
            released.await.unwrap();
            drop(ws);
        });
        let client = Client::connect(config).await.unwrap();
        let mut status = serde_json::from_str::<ServiceStatus>(STATUS).unwrap();
        // Client encoding deliberately does not assess XSD facets. This oversized
        // label is a transport-pressure fixture, not a schema-valid OMS specimen.
        status.message.message_data.service_id.descriptive_label =
            Some("x".repeat(16 * 1024 * 1024));
        let publication = client.send(&status);
        tokio::pin!(publication);
        assert!(futures_util::poll!(&mut publication).is_pending());
        observed.await.unwrap();
        let closure = timeout(Duration::from_millis(500), client.close())
            .await
            .expect("close must apply its drain bound to a write already underway")
            .unwrap_err();
        assert!(matches!(
            closure,
            ClientError::Connection {
                kind: ConnectionFailure::Deadline,
                transmitted: false,
                ..
            }
        ));
        assert!(matches!(
            publication.await,
            Err(ClientError::Connection {
                kind: ConnectionFailure::Deadline,
                transmitted: true,
                ..
            })
        ));
        release.send(()).unwrap();
        peer.await.unwrap();
    })
    .await
    .expect("socket backpressure cannot hold shared close indefinitely");
}

#[tokio::test]
async fn canceled_backpressured_publication_keeps_message_and_control_reception_live() {
    timeout(Duration::from_secs(6), async {
        let (listener, mut config) = listener().await;
        config.operation_timeout = Duration::from_secs(3);
        let (writing, observed) = tokio::sync::oneshot::channel();
        let (proceed, canceled) = tokio::sync::oneshot::channel();
        let (drain, received) = tokio::sync::oneshot::channel();
        let peer = tokio::spawn(async move {
            // The pressure label plus its OMS wrapper exceeds the peer library's
            // default 16 MiB frame cap. Keep that fixture and allow its full frame
            // so the real peer can drain it after proving independent reception.
            let mut ws = handshake_with_config(
                listener,
                Some(
                    tokio_tungstenite::tungstenite::protocol::WebSocketConfig::default()
                        .max_frame_size(Some(32 * 1024 * 1024)),
                ),
            )
            .await;
            let sub = text(&mut ws).await;
            let sid = sub.split_whitespace().nth(1).unwrap().to_owned();
            ws.send(Message::text("+OK")).await.unwrap();
            let mut byte = [0];
            assert_eq!(ws.get_ref().peek(&mut byte).await.unwrap(), 1);
            writing.send(()).unwrap();
            canceled.await.unwrap();
            ws.send(Message::text(format!("MSG {sid} {STATUS}")))
                .await
                .unwrap();
            ws.send(Message::Ping(vec![9].into())).await.unwrap();
            ws.send(Message::text(format!("MSG {sid} {STATUS}")))
                .await
                .unwrap();
            // The application must receive both MSGs, including the one after
            // Ping, while the PUB is still blocked on peer-side backpressure.
            received.await.unwrap();
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            assert_eq!(
                ws.next().await.unwrap().unwrap(),
                Message::Pong(vec![9].into())
            );
            assert!(
                timeout(Duration::from_millis(40), ws.next()).await.is_err(),
                "the canceled PUB still owns its acknowledgment"
            );
            ws.send(Message::text(
                "-ERR Invalid-Message canceled pressure publication only",
            ))
            .await
            .unwrap();
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            ws.send(Message::text("+OK")).await.unwrap();
            finish(&mut ws).await;
        });
        let client = Client::connect(config).await.unwrap();
        let mut reports = client.subscribe::<ServiceStatus>().await.unwrap();
        let sender = client.clone();
        let publication = tokio::spawn(async move {
            let mut pressure = serde_json::from_str::<ServiceStatus>(STATUS).unwrap();
            pressure.message.message_data.service_id.descriptive_label =
                Some("x".repeat(16 * 1024 * 1024));
            sender.send(&pressure).await
        });
        observed.await.unwrap();
        publication.abort();
        assert!(publication.await.unwrap_err().is_cancelled());
        let status = serde_json::from_str::<ServiceStatus>(STATUS).unwrap();
        let queued = client.send(&status);
        tokio::pin!(queued);
        assert!(futures_util::poll!(&mut queued).is_pending());
        proceed.send(()).unwrap();
        for _ in 0..2 {
            assert_eq!(
                timeout(Duration::from_millis(300), reports.next())
                    .await
                    .expect("MSG dispatch must remain live during a canceled backpressured write")
                    .unwrap()
                    .unwrap(),
                status
            );
        }
        assert!(futures_util::poll!(&mut queued).is_pending());
        drain.send(()).unwrap();
        queued.await.unwrap();
        client.close().await.unwrap();
        assert!(reports.next().await.is_none());
        peer.await.unwrap();
    })
    .await
    .expect("canceled write cannot block incoming OWP or WebSocket controls");
}

#[tokio::test]
async fn closure_timeout_is_visible_to_every_clone_and_live_stream() {
    timeout(Duration::from_secs(2), async {
        let (listener, mut config) = listener().await;
        config.close_timeout = Duration::from_millis(100);
        let (release, released) = tokio::sync::oneshot::channel();
        let peer = tokio::spawn(async move {
            let mut ws = handshake(listener).await;
            for _ in 0..2 {
                assert!(text(&mut ws).await.starts_with("SUB "));
                ws.send(Message::text("+OK")).await.unwrap();
            }
            // Do not read the Close frame: no automatic handshake reply is produced.
            released.await.unwrap();
            assert!(matches!(
                ws.next().await.unwrap().unwrap(),
                Message::Close(_)
            ));
        });
        let client = Client::connect(config).await.unwrap();
        let clone = client.clone();
        let mut first = client.subscribe::<ServiceStatus>().await.unwrap();
        let mut second = clone.subscribe::<ServiceStatus>().await.unwrap();
        let error = clone.close().await.unwrap_err();
        assert!(matches!(
            error,
            ClientError::Connection {
                kind: ConnectionFailure::Deadline,
                transmitted: false,
                ..
            }
        ));
        assert_eq!(client.close().await, Err(error.clone()));
        for stream in [&mut first, &mut second] {
            assert!(matches!(
                stream.next().await,
                Some(Err(ReceiveError::Connection {
                    kind: ConnectionFailure::Deadline,
                    ..
                }))
            ));
            assert!(stream.next().await.is_none());
        }
        assert_eq!(
            client
                .send(&serde_json::from_str::<ServiceStatus>(STATUS).unwrap())
                .await,
            Err(error)
        );
        release.send(()).unwrap();
        peer.await.unwrap();
    })
    .await
    .expect("closure failure is bounded and shared");
}

#[tokio::test]
async fn connects_with_fixed_profile_and_send_waits_for_publication_acknowledgment() {
    timeout(Duration::from_secs(5), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("ws://{}/", listener.local_addr().unwrap());
        let peer = tokio::spawn(async move {
            let (tcp, _) = listener.accept().await.unwrap();
            let mut ws = tokio_tungstenite::accept_hdr_async(tcp, |request: &tokio_tungstenite::tungstenite::handshake::server::Request, mut response: tokio_tungstenite::tungstenite::handshake::server::Response| {
                assert_eq!(request.headers()["Sec-WebSocket-Protocol"], "owp");
                response.headers_mut().insert("Sec-WebSocket-Protocol", "owp".parse().unwrap());
                Ok(response)
            }).await.unwrap();
            let init = ws.next().await.unwrap().unwrap().into_text().unwrap();
            let init: serde_json::Value = serde_json::from_str(init.strip_prefix("INIT ").unwrap()).unwrap();
            assert_eq!(init, serde_json::json!({"versions":["1.0"],"schema":"002.5.0","verbose":true,"service_id":"tester"}));
            ws.send(Message::text("+OK")).await.unwrap();
            ws.send(Message::text(INFO)).await.unwrap();
            let publication = ws.next().await.unwrap().unwrap().into_text().unwrap();
            let json = publication.strip_prefix("PUB ServiceStatus ").unwrap();
            assert_eq!(serde_json::from_str::<serde_json::Value>(json).unwrap(), serde_json::from_str::<serde_json::Value>(STATUS).unwrap());
            ws.send(Message::text("+OK")).await.unwrap();
            assert!(matches!(ws.next().await.unwrap().unwrap(), Message::Close(_)));
            ws.flush().await.unwrap();
        });
        let client = Client::connect(ClientConfig::new(endpoint, "tester")).await.unwrap();
        let status: ServiceStatus = serde_json::from_str(STATUS).unwrap();
        client.send(&status).await.unwrap();
        client.close().await.unwrap();
        peer.await.unwrap();
    }).await.expect("bounded public client exchange");
}

#[tokio::test]
async fn last_owner_drop_closes_transport_without_an_ownership_cycle() {
    timeout(Duration::from_secs(2), async {
        let (listener, config) = listener().await;
        let peer = tokio::spawn(async move {
            let mut ws = handshake(listener).await;
            finish(&mut ws).await;
        });
        drop(Client::connect(config).await.unwrap());
        peer.await.unwrap();
    })
    .await
    .expect("last-owner cleanup must make progress");
}

#[tokio::test]
async fn independent_streams_decode_errors_interleaved_messages_and_cleanup() {
    timeout(Duration::from_secs(5), async {
        let (listener, config) = listener().await;
        let peer = tokio::spawn(async move {
            let mut ws = handshake(listener).await;
            let first = text(&mut ws).await;
            let sid1 = first.split_whitespace().nth(1).unwrap().to_owned();
            assert_eq!(first, format!("SUB {sid1} ServiceStatus ServiceStatus"));
            // Delivery can precede SUB acknowledgment: the stream must retain it.
            ws.send(Message::text(format!("MSG {sid1} {STATUS}")))
                .await
                .unwrap();
            ws.send(Message::text("+OK")).await.unwrap();
            let second = text(&mut ws).await;
            let sid2 = second.split_whitespace().nth(1).unwrap().to_owned();
            assert_ne!(sid1, sid2);
            ws.send(Message::text("+OK")).await.unwrap();
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            ws.send(Message::Ping(vec![1, 2, 3].into())).await.unwrap();
            for payload in [
                "not-json",
                r#"{"ServiceStatus":{}}"#,
                r#"{"ServiceLifecycleCommand":{}}"#,
            ] {
                ws.send(Message::text(format!("MSG {sid1} {payload}")))
                    .await
                    .unwrap();
            }
            ws.send(Message::text(format!("MSG {sid1} {STATUS}")))
                .await
                .unwrap();
            ws.send(Message::text(format!("MSG {sid2} {STATUS}")))
                .await
                .unwrap();
            // The client processes MSG and ping while send is still waiting.
            assert_eq!(
                ws.next().await.unwrap().unwrap(),
                Message::Pong(vec![1, 2, 3].into())
            );
            ws.send(Message::text("+OK")).await.unwrap();
            assert_eq!(text(&mut ws).await, format!("UNSUB {sid1}"));
            ws.send(Message::text("+OK")).await.unwrap();
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            ws.send(Message::text(format!("MSG {sid2} {STATUS}")))
                .await
                .unwrap();
            ws.send(Message::text("+OK")).await.unwrap();
            assert_eq!(text(&mut ws).await, format!("UNSUB {sid2}"));
            ws.send(Message::text("+OK")).await.unwrap();
            // An operation after best-effort Drop cleanup is an acknowledgment barrier.
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            ws.send(Message::text("+OK")).await.unwrap();
            finish(&mut ws).await;
        });
        let client = Client::connect(config).await.unwrap();
        let clone = client.clone();
        drop(clone);
        let mut first = client.subscribe::<ServiceStatus>().await.unwrap();
        let mut second = client.subscribe::<ServiceStatus>().await.unwrap();
        assert_eq!(
            first.next().await.unwrap().unwrap(),
            serde_json::from_str::<ServiceStatus>(STATUS).unwrap()
        );
        let sender = client.clone();
        let publication = tokio::spawn(async move {
            sender
                .send(&serde_json::from_str::<ServiceStatus>(STATUS).unwrap())
                .await
        });
        for _ in 0..3 {
            assert!(matches!(
                first.next().await.unwrap(),
                Err(ReceiveError::Decoding {
                    root: "ServiceStatus",
                    ..
                })
            ));
        }
        assert!(first.next().await.unwrap().is_ok());
        assert!(second.next().await.unwrap().is_ok());
        publication.await.unwrap().unwrap();
        first.close().await.unwrap();
        client
            .send(&serde_json::from_str::<ServiceStatus>(STATUS).unwrap())
            .await
            .unwrap();
        assert!(second.next().await.unwrap().is_ok());
        drop(second);
        client
            .send(&serde_json::from_str::<ServiceStatus>(STATUS).unwrap())
            .await
            .unwrap();
        client.clone().close().await.unwrap();
        assert_eq!(
            client
                .send(&serde_json::from_str::<ServiceStatus>(STATUS).unwrap())
                .await,
            Err(ClientError::Closed)
        );
        peer.await.unwrap();
    })
    .await
    .expect("independent stream and operation progress");
}

#[tokio::test]
async fn concurrent_calls_are_serialized_and_rejection_details_are_attributable() {
    timeout(Duration::from_secs(5), async {
        let (listener, config) = listener().await;
        let (sent, observed) = tokio::sync::oneshot::channel();
        let peer = tokio::spawn(async move {
            let mut ws = handshake(listener).await;
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            sent.send(()).unwrap();
            assert!(
                timeout(Duration::from_millis(40), ws.next()).await.is_err(),
                "no second transmitted operation awaiting the first response"
            );
            ws.send(Message::text(
                "-ERR Invalid-Message exact diagnostic with spaces",
            ))
            .await
            .unwrap();
            assert!(text(&mut ws).await.starts_with("SUB "));
            ws.send(Message::text("+OK")).await.unwrap();
            assert!(text(&mut ws).await.starts_with("UNSUB "));
            ws.send(Message::text("+OK")).await.unwrap();
            finish(&mut ws).await;
        });
        let client = Client::connect(config).await.unwrap();
        let sender = client.clone();
        let publication = tokio::spawn(async move {
            sender
                .send(&serde_json::from_str::<ServiceStatus>(STATUS).unwrap())
                .await
        });
        observed.await.unwrap();
        let subscription = client.subscribe::<ServiceStatus>().await.unwrap();
        assert_eq!(
            publication.await.unwrap(),
            Err(ClientError::Rejected {
                error: la_cal_client::OwpError::InvalidMessage,
                details: Some("exact diagnostic with spaces".into()),
            })
        );
        subscription.close().await.unwrap();
        client.close().await.unwrap();
        peer.await.unwrap();
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn unexpected_loss_and_malformed_protocol_are_terminal_receive_failures() {
    for failure in [
        None,
        Some("MSG"),
        Some("+OK unexpected fields"),
        Some("-ERR"),
        Some("-ERR Not-A-Standard-Error details"),
        Some("UNKNOWN"),
    ] {
        timeout(Duration::from_secs(5), async {
            let (listener, config) = listener().await;
            let peer = tokio::spawn(async move {
                let mut ws = handshake(listener).await;
                for _ in 0..2 {
                    assert!(text(&mut ws).await.starts_with("SUB "));
                    ws.send(Message::text("+OK")).await.unwrap();
                }
                if let Some(frame) = failure {
                    ws.send(Message::text(frame)).await.unwrap();
                    finish(&mut ws).await;
                }
                // Dropping the peer without a closing handshake is unexpected loss.
            });
            let client = Client::connect(config).await.unwrap();
            let clone = client.clone();
            let mut first = client.subscribe::<ServiceStatus>().await.unwrap();
            let mut second = clone.subscribe::<ServiceStatus>().await.unwrap();
            let terminal = first.next().await.unwrap().unwrap_err();
            let ReceiveError::Connection { kind, details } = &terminal else {
                panic!("malformed control or loss must be terminal")
            };
            assert_eq!(
                *kind,
                if failure.is_some() {
                    ConnectionFailure::Protocol
                } else {
                    ConnectionFailure::Transport
                }
            );
            assert_eq!(second.next().await, Some(Err(terminal.clone())));
            assert!(first.next().await.is_none());
            assert!(second.next().await.is_none());
            let shared_error = ClientError::Connection {
                kind: *kind,
                reason: details.clone(),
                transmitted: false,
            };
            assert_eq!(
                clone
                    .send(&serde_json::from_str::<ServiceStatus>(STATUS).unwrap())
                    .await,
                Err(shared_error.clone())
            );
            assert_eq!(client.close().await, Err(shared_error));
            peer.await.unwrap();
        })
        .await
        .expect("terminal error then stream end");
    }
}

#[tokio::test]
async fn connect_rejects_wrong_info_profile_and_bounds_negotiation() {
    for info in [Some(INFO.replace("1.0", "2.0")), None] {
        timeout(Duration::from_secs(2), async {
            let (listener, mut config) = listener().await;
            config.connection_timeout = Duration::from_millis(100);
            let peer = tokio::spawn(async move {
                let (tcp, _) = listener.accept().await.unwrap();
                let mut ws = tokio_tungstenite::accept_hdr_async(tcp, |_: &tokio_tungstenite::tungstenite::handshake::server::Request, mut response: tokio_tungstenite::tungstenite::handshake::server::Response| {
                    response.headers_mut().insert("Sec-WebSocket-Protocol", "owp".parse().unwrap());
                    Ok(response)
                }).await.unwrap();
                text(&mut ws).await;
                ws.send(Message::text("+OK")).await.unwrap();
                if let Some(info) = info { ws.send(Message::text(info)).await.unwrap(); }
                let _ = ws.next().await;
            });
            assert!(matches!(Client::connect(config).await, Err(ClientError::Negotiation(_))));
            peer.await.unwrap();
        }).await.unwrap();
    }
}

#[tokio::test]
async fn invalid_configuration_is_rejected_before_connecting() {
    for (endpoint, participant) in [
        ("wss://127.0.0.1/", "tester"),
        ("ws://example.com/", "tester"),
        ("ws://127.0.0.1/", "bad id"),
    ] {
        assert!(matches!(
            Client::connect(ClientConfig::new(endpoint, participant)).await,
            Err(ClientError::Configuration(_))
        ));
    }
    let mut config = ClientConfig::new("ws://127.0.0.1/", "tester");
    config.connection_timeout = Duration::ZERO;
    assert!(matches!(
        Client::connect(config).await,
        Err(ClientError::Configuration(_))
    ));
    let mut config = ClientConfig::new("ws://127.0.0.1/", "tester");
    config.operation_timeout = Duration::MAX;
    assert!(matches!(
        Client::connect(config).await,
        Err(ClientError::Configuration(_))
    ));
}

#[tokio::test]
async fn derived_payloads_keep_type_tags_and_fields_and_bad_derivations_are_nonterminal() {
    use la_cal_client::messages::uci_002_5::ServiceLifecycleCommandStatus;
    const DERIVED: &str =
        include_str!("../../../fixtures/service-lifecycle-command-status-derived.json");
    timeout(Duration::from_secs(5), async {
        let (listener, config) = listener().await;
        let peer = tokio::spawn(async move {
            let mut ws = handshake(listener).await;
            let sub = text(&mut ws).await;
            let sid = sub.split_whitespace().nth(1).unwrap();
            ws.send(Message::text("+OK")).await.unwrap();
            // Unrelated, abstract, unknown types and derived-only fields without
            // the required tag must be errors, never silently discarded data.
            for invalid in [
                DERIVED.replace("TaskPlanID_Type", "SecurityInformationType"),
                DERIVED.replace("TaskPlanID_Type", "MessageType"),
                DERIVED.replace("TaskPlanID_Type", "UnknownType"),
                DERIVED.replace("\"$type\": \"TaskPlanID_Type\",", ""),
                DERIVED.replace("\"Version\": 7", "\"UnknownField\": 7"),
            ] {
                ws.send(Message::text(format!("MSG {sid} {invalid}")))
                    .await
                    .unwrap();
            }
            ws.send(Message::text(format!("MSG {sid} {DERIVED}")))
                .await
                .unwrap();
            let publication = text(&mut ws).await;
            let json = publication
                .strip_prefix("PUB ServiceLifecycleCommandStatus ")
                .unwrap();
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(json).unwrap(),
                serde_json::from_str::<serde_json::Value>(DERIVED).unwrap()
            );
            ws.send(Message::text("+OK")).await.unwrap();
            finish(&mut ws).await;
        });
        let client = Client::connect(config).await.unwrap();
        let mut subscription = client
            .subscribe::<ServiceLifecycleCommandStatus>()
            .await
            .unwrap();
        for _ in 0..5 {
            assert!(matches!(
                subscription.next().await,
                Some(Err(ReceiveError::Decoding { .. }))
            ));
        }
        let message = subscription.next().await.unwrap().unwrap();
        client.send(&message).await.unwrap();
        client.close().await.unwrap();
        assert!(subscription.next().await.is_none());
        peer.await.unwrap();
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn rejected_subscription_and_unsubscribe_preserve_details_and_leave_client_usable() {
    timeout(Duration::from_secs(5), async {
        let (listener, config) = listener().await;
        let peer = tokio::spawn(async move {
            let mut ws = handshake(listener).await;
            assert!(text(&mut ws).await.starts_with("SUB "));
            ws.send(Message::text("-ERR Illegal-State subscription denied")).await.unwrap();
            assert!(text(&mut ws).await.starts_with("SUB "), "a rejected SUB did not create a registration to clean up");
            ws.send(Message::text("+OK")).await.unwrap();
            assert!(text(&mut ws).await.starts_with("UNSUB "));
            ws.send(Message::text("-ERR Internal-Error cleanup failed")).await.unwrap();
            assert!(text(&mut ws).await.starts_with("PUB ServiceStatus "));
            ws.send(Message::text("+OK")).await.unwrap();
            finish(&mut ws).await;
        });
        let client = Client::connect(config).await.unwrap();
        assert!(matches!(client.subscribe::<ServiceStatus>().await, Err(ClientError::Rejected { error: la_cal_client::OwpError::IllegalState, details: Some(details) }) if details == "subscription denied"));
        let subscription = client.subscribe::<ServiceStatus>().await.unwrap();
        assert_eq!(subscription.close().await, Err(ClientError::Rejected { error: la_cal_client::OwpError::InternalError, details: Some("cleanup failed".into()) }));
        client.send(&serde_json::from_str::<ServiceStatus>(STATUS).unwrap()).await.unwrap();
        client.close().await.unwrap();
        peer.await.unwrap();
    }).await.unwrap();
}
