//! Native process/OWP and Zenoh boundary evidence (tickets 14–16).
//! Extended 2026-10-05: ticket 17 typed client exchange for all selected roots.

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use la_cal_client::{
    Client, ClientConfig,
    messages::uci_002_5::{ServiceLifecycleCommand, ServiceLifecycleCommandStatus, ServiceStatus},
};
use prost::Message as _;
use sleet_types::owp::{OwpError, ServerOp, parse_server};
use tokio::time::{sleep, timeout};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

struct OwnedServer(Child);

// Independent wire oracle from the approved envelope contract.
#[derive(prost::Message)]
struct Envelope {
    #[prost(bytes = "vec", tag = "1")]
    oms_json: Vec<u8>,
}

impl Drop for OwnedServer {
    fn drop(&mut self) {
        // Always reap the process, including assertion/panic paths.
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_process_negotiates_validates_and_delivers_service_status() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixture = std::fs::read_to_string(root.join("fixtures/service-status.json"))
        .expect("read independently specified UCI 002.5.0 fixture");
    let temp = tempfile::tempdir().expect("temporary launch configuration");
    let router_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let router_addr = router_listener.local_addr().unwrap();
    drop(router_listener);
    let router_config = std::fs::read_to_string(root.join("config/zenoh-router.json5"))
        .unwrap()
        .replace("127.0.0.1:7447", &router_addr.to_string());
    let router_path = temp.path().join("router.json5");
    std::fs::write(&router_path, router_config).unwrap();
    let mut router = OwnedServer(
        Command::new(std::env::var_os("ZENOHD").unwrap_or_else(|| "zenohd".into()))
            .args(["-c", router_path.to_str().unwrap()])
            .env("RUST_LOG", "error")
            .stdin(Stdio::null())
            .spawn()
            .expect("launch native zenohd (install on PATH or set ZENOHD)"),
    );
    timeout(Duration::from_secs(5), async {
        loop {
            assert!(
                router.0.try_wait().unwrap().is_none(),
                "router exited during startup"
            );
            if tokio::net::TcpStream::connect(router_addr).await.is_ok() {
                break;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("bounded router listener readiness");
    let observer_config = zenoh::Config::from_json5(&format!(
        r#"{{
        mode: "client", connect: {{ endpoints: ["tcp/{router_addr}"] }},
        scouting: {{ multicast: {{ enabled: false }}, gossip: {{ enabled: false }} }}
    }}"#
    ))
    .unwrap();
    let observer = timeout(Duration::from_secs(5), zenoh::open(observer_config))
        .await
        .unwrap()
        .unwrap();
    let samples = observer
        .declare_subscriber("la-cal/uci/002.5.0/*")
        .await
        .unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("choose loopback port");
    let addr = listener.local_addr().expect("local address");
    drop(listener);
    // Reuse checked-in configuration with isolated server/router ports.
    let config = std::fs::read_to_string(root.join("config/server.toml"))
        .expect("read native configuration")
        .replace("127.0.0.1:9000", &addr.to_string())
        .replace("tcp/127.0.0.1:7447", &format!("tcp/{router_addr}"));
    let config_path = temp.path().join("server.toml");
    std::fs::write(&config_path, config).expect("write launch configuration");
    let mut server = OwnedServer(
        Command::new(env!("CARGO_BIN_EXE_la-cal"))
            .arg(config_path)
            .current_dir(&root)
            .env("RUST_LOG", "error")
            .stdin(Stdio::null())
            .spawn()
            .expect("launch project-owned native server"),
    );
    let mut ws = timeout(Duration::from_secs(5), async {
        loop {
            assert!(server.0.try_wait().expect("check server exit").is_none());
            let mut request = format!("ws://{addr}/").into_client_request().unwrap();
            request
                .headers_mut()
                .insert("Sec-WebSocket-Protocol", "owp".parse().unwrap());
            match tokio_tungstenite::connect_async(request).await {
                Ok((ws, response)) => {
                    assert_eq!(response.headers()["Sec-WebSocket-Protocol"], "owp");
                    break ws;
                }
                Err(tokio_tungstenite::tungstenite::Error::Io(e))
                    if e.kind() == std::io::ErrorKind::ConnectionRefused =>
                {
                    sleep(Duration::from_millis(20)).await;
                }
                Err(error) => panic!("WebSocket establishment failed: {error}"),
            }
        }
    })
    .await
    .expect("bounded server readiness");

    timeout(Duration::from_secs(5), async {
        ws.send(Message::text(
            r#"INIT {"versions":["1.0"],"schema":"002.5.0","verbose":true,"service_id":"baseline-tester"}"#,
        ))
        .await
        .unwrap();
        assert!(matches!(read_operation(&mut ws).await, ServerOp::Ok));
        let ServerOp::Info(info) = read_operation(&mut ws).await else {
            panic!("expected negotiated INFO");
        };
        assert_eq!(info.version, "1.0");
        assert_eq!(info.server_id, "la-cal-baseline");
        assert_eq!(info.uuids.service, "123e4567-e89b-12d3-a456-426614174001");
        assert_eq!(info.uuids.system, "123e4567-e89b-12d3-a456-426614174000");
        assert_topic_policy(&mut ws, &fixture).await;
        ws.send(Message::text("SUB status ServiceStatus ServiceStatus"))
            .await
            .unwrap();
        assert!(matches!(read_operation(&mut ws).await, ServerOp::Ok));
        ws.send(Message::text(format!("PUB ServiceStatus {fixture}")))
            .await
            .unwrap();
        let mut acknowledged = false;
        let mut delivered = false;
        for _ in 0..2 {
            match read_operation(&mut ws).await {
                ServerOp::Ok => {
                    assert!(!acknowledged, "duplicate publication acknowledgment");
                    acknowledged = true;
                }
                ServerOp::Msg { sid, payload } => {
                    assert!(!delivered, "duplicate delivery");
                    assert_eq!(sid, "status");
                    assert_eq!(&*payload, fixture, "original accepted JSON must survive routing");
                    delivered = true;
                }
                other => panic!("expected PUB acknowledgment or MSG, received {other:?}"),
            }
        }
        assert!(acknowledged && delivered);
        let sample = timeout(Duration::from_secs(5), samples.recv_async())
            .await.expect("OWP PUB must traverse Zenoh").unwrap();
        assert_eq!(sample.key_expr().as_str(), "la-cal/uci/002.5.0/ServiceStatus");
        let envelope = Envelope::decode(sample.payload().to_bytes().as_ref()).unwrap();
        assert_eq!(envelope.oms_json, fixture.as_bytes(), "original bytes on the ASB");
        // ASB ingress is independent of OWP PUB. Invalid samples must not leak
        // to OWP, and ordinary unknown Protobuf fields must not break delivery.
        let status_key = "la-cal/uci/002.5.0/ServiceStatus";
        let invalid_payloads = [
            vec![0xff], // Malformed Protobuf.
            vec![], // Missing/empty payload.
            Envelope { oms_json: vec![0xff] }.encode_to_vec(),
            Envelope { oms_json: b"{not JSON}".to_vec() }.encode_to_vec(),
            Envelope { oms_json: b"{\"ServiceLifecycleCommand\":{}}".to_vec() }.encode_to_vec(),
            Envelope { oms_json: b"{\"ServiceStatus\":{}}".to_vec() }.encode_to_vec(),
        ];
        for bytes in invalid_payloads {
            observer.put(status_key, bytes).await.unwrap();
        }
        observer.put("la-cal/uci/002.5.0/UnsupportedRoot",
            Envelope { oms_json: fixture.as_bytes().to_vec() }.encode_to_vec())
            .await.unwrap();
        let incoming_json = format!("\n {fixture} \n");
        let mut incoming_envelope = Envelope { oms_json: incoming_json.as_bytes().to_vec() }.encode_to_vec();
        incoming_envelope.extend([0x10, 0x07]); // Unknown field 2, varint 7.
        observer.put(status_key, incoming_envelope).await.unwrap();
        let ServerOp::Msg { sid, payload } = read_operation(&mut ws).await else {
            panic!("valid ASB ingress must deliver without an OWP acknowledgment");
        };
        assert_eq!(sid, "status");
        assert_eq!(&*payload, incoming_json, "inbound original bytes, following invalid samples");
        ws.send(Message::text("UNSUB status")).await.unwrap();
        assert!(matches!(read_operation(&mut ws).await, ServerOp::Ok));
        // Removing and recreating local registration must retain the one
        // startup listener. The connection can still exchange ASB messages.
        ws.send(Message::text("SUB again ServiceStatus ServiceStatus")).await.unwrap();
        assert!(matches!(read_operation(&mut ws).await, ServerOp::Ok));
        observer.put(status_key, Envelope { oms_json: fixture.as_bytes().to_vec() }.encode_to_vec())
            .await.unwrap();
        assert!(matches!(read_operation(&mut ws).await, ServerOp::Msg { sid, payload }
            if sid == "again" && *payload == fixture));
        ws.close(None).await.unwrap();
        println!("native ASB: OWP 1.0 / UCI 002.5.0 / PUB +OK and MSG / independent Zenoh envelope observation / validated inbound original bytes / listener survives UNSUB");
    })
    .await
    .expect("bounded OWP exchange");
    samples.undeclare().await.unwrap();
    let samples = observer
        .declare_subscriber("la-cal/uci/002.5.0/*")
        .await
        .unwrap();
    timeout(Duration::from_secs(5), async {
        let client = Client::connect(ClientConfig::new(format!("ws://{addr}/"), "baseline-tester")).await.unwrap();
        let commands = include_str!("../../../fixtures/service-lifecycle-command.json");
        let results = include_str!("../../../fixtures/service-lifecycle-command-status.json");
        typed_exchange::<ServiceLifecycleCommand>(&client, commands).await;
        typed_exchange::<ServiceLifecycleCommandStatus>(&client, results).await;
        typed_exchange::<ServiceStatus>(&client, &fixture).await;
        let expected = [("ServiceLifecycleCommand", commands), ("ServiceLifecycleCommandStatus", results), ("ServiceStatus", fixture.as_str())];
        let mut counts = [0; 3];
        for _ in 0..6 {
            let sample = samples.recv_async().await.unwrap();
            let index = expected.iter().position(|(root, _)| sample.key_expr().as_str() == format!("la-cal/uci/002.5.0/{root}")).expect("selected exact ASB key");
            counts[index] += 1;
            let envelope = Envelope::decode(sample.payload().to_bytes().as_ref()).unwrap();
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&envelope.oms_json).unwrap(), serde_json::from_str::<serde_json::Value>(expected[index].1).unwrap());
        }
        assert_eq!(counts, [2, 2, 2], "both typed publications of every root traverse the adapter");
        let derived = include_str!("../../../fixtures/service-lifecycle-command-status-derived.json");
        typed_exchange::<ServiceLifecycleCommandStatus>(&client, derived).await;
        for _ in 0..2 {
            let sample = samples.recv_async().await.unwrap();
            assert_eq!(sample.key_expr().as_str(), "la-cal/uci/002.5.0/ServiceLifecycleCommandStatus");
            let envelope = Envelope::decode(sample.payload().to_bytes().as_ref()).unwrap();
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&envelope.oms_json).unwrap(), serde_json::from_str::<serde_json::Value>(derived).unwrap(), "derived type tags, Version and SystemID survive actual adapter traversal");
        }
        client.close().await.unwrap();
        println!("native typed client: all three roots / real Zenoh envelopes / independent repeated subscriptions and explicit UNSUB");
    }).await.expect("bounded deployed typed-client exchange");
    samples.undeclare().await.unwrap();
    observer.close().await.unwrap();
    let signal = Command::new("kill")
        .args(["-TERM", &server.0.id().to_string()])
        .status()
        .unwrap();
    assert!(signal.success());
    timeout(Duration::from_secs(5), async {
        loop {
            if let Some(status) = server.0.try_wait().unwrap() {
                assert!(status.success(), "graceful server/session cleanup failed");
                break;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("bounded graceful server shutdown");
}

async fn typed_exchange<T: la_cal_client::Message>(client: &Client, fixture: &str) {
    let expected: serde_json::Value = serde_json::from_str(fixture).unwrap();
    let message: T = serde_json::from_str(fixture).unwrap();
    let mut first = client.subscribe::<T>().await.unwrap();
    let mut second = client.subscribe::<T>().await.unwrap();
    client.send(&message).await.unwrap();
    for delivered in [
        first.next().await.unwrap().unwrap(),
        second.next().await.unwrap().unwrap(),
    ] {
        assert_eq!(serde_json::to_value(delivered).unwrap(), expected);
    }
    first.close().await.unwrap();
    client.send(&message).await.unwrap();
    assert_eq!(
        serde_json::to_value(second.next().await.unwrap().unwrap()).unwrap(),
        expected
    );
    second.close().await.unwrap();
}

async fn assert_topic_policy(
    ws: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    fixture: &str,
) {
    for (sid, topic) in [
        ("commands", "ServiceLifecycleCommand"),
        ("results", "ServiceLifecycleCommandStatus"),
    ] {
        ws.send(Message::text(format!("SUB {sid} {topic} {topic}")))
            .await
            .unwrap();
        assert!(matches!(read_operation(ws).await, ServerOp::Ok));
        // The actual schema-valid ServiceStatus fixture cannot be sent on
        // another message-type topic, even without its optional binding.
        ws.send(Message::text(format!("PUB {topic} {fixture}")))
            .await
            .unwrap();
        assert!(matches!(
            read_operation(ws).await,
            ServerOp::Err {
                error: OwpError::InvalidMessage,
                details: Some(_)
            }
        ));
    }
}

#[tokio::test]
async fn missing_router_fails_startup_before_owp_admission() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let temp = tempfile::tempdir().unwrap();
    // Hold the router port without accepting/handshaking: there is no valid
    // router even though TCP connection alone could appear ready.
    let router_port = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let server_port = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let server_addr = server_port.local_addr().unwrap();
    drop(server_port);
    let config = std::fs::read_to_string(root.join("config/server.toml"))
        .unwrap()
        .replace("127.0.0.1:9000", &server_addr.to_string())
        .replace(
            "tcp/127.0.0.1:7447",
            &format!("tcp/{}", router_port.local_addr().unwrap()),
        )
        .replace(
            "asb_startup_timeout_ms = 5000",
            "asb_startup_timeout_ms = 300",
        );
    let config_path = temp.path().join("server.toml");
    std::fs::write(&config_path, config).unwrap();
    let mut server = OwnedServer(
        Command::new(env!("CARGO_BIN_EXE_la-cal"))
            .arg(config_path)
            .current_dir(&root)
            .env("RUST_LOG", "error")
            .stdin(Stdio::null())
            .spawn()
            .unwrap(),
    );
    timeout(Duration::from_secs(3), async {
        loop {
            assert!(
                tokio::net::TcpStream::connect(server_addr).await.is_err(),
                "OWP admission must wait for actual ASB readiness"
            );
            if let Some(status) = server.0.try_wait().unwrap() {
                assert!(!status.success(), "missing router must fail visibly");
                break;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("ASB startup must be bounded");
}

async fn read_operation(
    ws: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> ServerOp {
    match ws.next().await {
        Some(Ok(Message::Text(text))) => parse_server(&text).expect("valid OWP server frame"),
        other => panic!("expected OWP text, received {other:?}"),
    }
}
