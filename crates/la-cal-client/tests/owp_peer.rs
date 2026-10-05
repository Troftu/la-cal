//! Behavioral checks at the approved public typed-client / OWP peer seam.
#![allow(clippy::result_large_err)] // Tungstenite's handshake callback requires this response type.
use futures_util::{SinkExt, StreamExt};
use la_cal_client::{Client, ClientConfig, messages::uci_002_5::ServiceStatus};
use la_cal_client::{ClientError, ReceiveError};
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
    let (tcp, _) = listener.accept().await.unwrap();
    let mut ws = tokio_tungstenite::accept_hdr_async(
        tcp,
        |_: &tokio_tungstenite::tungstenite::handshake::server::Request,
         mut response: tokio_tungstenite::tungstenite::handshake::server::Response| {
            response
                .headers_mut()
                .insert("Sec-WebSocket-Protocol", "owp".parse().unwrap());
            Ok(response)
        },
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
        Some("UNKNOWN"),
    ] {
        timeout(Duration::from_secs(5), async {
            let (listener, config) = listener().await;
            let peer = tokio::spawn(async move {
                let mut ws = handshake(listener).await;
                assert!(text(&mut ws).await.starts_with("SUB "));
                ws.send(Message::text("+OK")).await.unwrap();
                if let Some(frame) = failure {
                    ws.send(Message::text(frame)).await.unwrap();
                    finish(&mut ws).await;
                }
                // Dropping the peer without a closing handshake is unexpected loss.
            });
            let client = Client::connect(config).await.unwrap();
            let mut subscription = client.subscribe::<ServiceStatus>().await.unwrap();
            assert!(matches!(
                subscription.next().await,
                Some(Err(ReceiveError::Connection { .. }))
            ));
            assert!(subscription.next().await.is_none());
            assert!(client.close().await.is_err());
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
