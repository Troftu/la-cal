//! Native process/OWP and Zenoh boundary evidence (tickets 14–16).
//! Extended 2026-10-05: ticket 17 typed client exchange for all selected roots.
//! Extended 2026-10-05: ticket 19 ingress rejection and deployed router loss.
//! OWP behavior: OMS 2.5 OMSC-SPC-013 Rev B §§5.1.1.1.1.3–.4,
//! 5.1.1.1.3.2–.4 and 6.1.5; payload fixtures/schema: UCI 002.5.0.
//! The Protobuf/Zenoh mapping and terminal router-loss policy are project decisions.

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

type Socket =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

struct NativeHarness {
    server: OwnedServer,
    router: OwnedServer,
    addr: std::net::SocketAddr,
    router_addr: std::net::SocketAddr,
    router_path: PathBuf,
    log_path: PathBuf,
    _temp: tempfile::TempDir,
}

impl Drop for NativeHarness {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!(
                "native server diagnostics:\n{}",
                std::fs::read_to_string(&self.log_path).unwrap_or_default()
            );
        }
        // OwnedServer fields are dropped/reaped before their configuration and
        // diagnostic directory, including startup and timeout assertion paths.
    }
}

// Independent wire oracle from the approved envelope contract.
#[derive(prost::Message)]
struct Envelope {
    #[prost(bytes = "vec", tag = "1")]
    oms_json: Vec<u8>,
}

fn envelope_bytes(oms_json: &[u8]) -> Vec<u8> {
    Envelope {
        oms_json: oms_json.to_vec(),
    }
    .encode_to_vec()
}

impl Drop for OwnedServer {
    fn drop(&mut self) {
        // Always reap the process, including assertion/panic paths.
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

impl NativeHarness {
    async fn start() -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let temp = tempfile::tempdir().expect("temporary launch configuration");
        let router_addr = unused_loopback_addr();
        let router_config = std::fs::read_to_string(root.join("config/zenoh-router.json5"))
            .unwrap()
            .replace("127.0.0.1:7447", &router_addr.to_string());
        let router_path = temp.path().join("router.json5");
        std::fs::write(&router_path, router_config).unwrap();
        let router = start_router(&router_path, router_addr).await;
        let addr = unused_loopback_addr();
        let config = std::fs::read_to_string(root.join("config/server.toml"))
            .unwrap()
            .replace("127.0.0.1:9000", &addr.to_string())
            .replace("tcp/127.0.0.1:7447", &format!("tcp/{router_addr}"));
        let config_path = temp.path().join("server.toml");
        std::fs::write(&config_path, config).unwrap();
        let log_path = temp.path().join("server.log");
        let log = std::fs::File::create(&log_path).unwrap();
        let server = OwnedServer(
            Command::new(env!("CARGO_BIN_EXE_la-cal"))
                .arg(config_path)
                .current_dir(root)
                .env("RUST_LOG", "warn")
                .stdin(Stdio::null())
                .stderr(log.try_clone().unwrap())
                .stdout(log)
                .spawn()
                .expect("launch project-owned native server"),
        );
        Self {
            server,
            router,
            addr,
            router_addr,
            router_path,
            log_path,
            _temp: temp,
        }
    }

    async fn connect(&mut self) -> Socket {
        timeout(Duration::from_secs(5), async {
            loop {
                assert!(
                    self.server.0.try_wait().unwrap().is_none(),
                    "server exited during startup: {}",
                    std::fs::read_to_string(&self.log_path).unwrap()
                );
                let mut request = format!("ws://{}/", self.addr)
                    .into_client_request()
                    .unwrap();
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
        .expect("bounded server readiness")
    }
}

fn unused_loopback_addr() -> std::net::SocketAddr {
    std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
}

async fn start_router(path: &std::path::Path, addr: std::net::SocketAddr) -> OwnedServer {
    let mut router = OwnedServer(
        Command::new(std::env::var_os("ZENOHD").unwrap_or_else(|| "zenohd".into()))
            .arg("-c")
            .arg(path)
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
            if tokio::net::TcpStream::connect(addr).await.is_ok() {
                break;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("bounded router listener readiness");
    router
}

async fn negotiate(ws: &mut Socket) {
    ws.send(Message::text(
        r#"INIT {"versions":["1.0"],"schema":"002.5.0","verbose":true,"service_id":"baseline-tester"}"#,
    ))
    .await
    .unwrap();
    assert!(matches!(read_operation(ws).await, ServerOp::Ok));
    assert!(matches!(read_operation(ws).await, ServerOp::Info(_)));
}

async fn open_observer(router_addr: std::net::SocketAddr) -> zenoh::Session {
    let config = zenoh::Config::from_json5(&format!(
        r#"{{
        mode: "client", connect: {{ endpoints: ["tcp/{router_addr}"] }},
        scouting: {{ multicast: {{ enabled: false }}, gossip: {{ enabled: false }} }}
    }}"#
    ))
    .unwrap();
    timeout(Duration::from_secs(5), zenoh::open(config))
        .await
        .expect("bounded independent ASB observer startup")
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_process_negotiates_validates_and_delivers_service_status() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixture = std::fs::read_to_string(root.join("fixtures/service-status.json"))
        .expect("read independently specified UCI 002.5.0 fixture");
    let mut harness = NativeHarness::start().await;
    let router_addr = harness.router_addr;
    let addr = harness.addr;
    let observer = open_observer(router_addr).await;
    let samples = observer
        .declare_subscriber("la-cal/uci/002.5.0/*")
        .await
        .unwrap();
    let mut ws = harness.connect().await;

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
        // The UCI 002.5.0 ServiceStateEnum has five values, none STOPPED.
        // Unlike an empty root, this exercises validation deep inside an
        // otherwise complete selected message.
        let invalid_state = fixture.replace("\"NORMAL\"", "\"STOPPED\"");
        assert_ne!(invalid_state, fixture);
        // Reuse the same deep schema defect on deployed OWP PUB as on ASB
        // ingress. Rejection must be attributable, with this connection still
        // able to receive the valid ingress progress witnesses below.
        ws.send(Message::text(format!("PUB ServiceStatus {invalid_state}"))).await.unwrap();
        assert!(matches!(read_operation(&mut ws).await, ServerOp::Err {
            error: OwpError::InvalidMessage, details: Some(_)
        }));
        let commands = include_bytes!("../../../fixtures/service-lifecycle-command.json");
        let mut unknown_invalid = envelope_bytes(invalid_state.as_bytes());
        unknown_invalid.extend([0x10, 0x07]); // Ordinary unknown field 2.
        let invalid_payloads = [
            ("malformed Protobuf", status_key, vec![0xff]),
            ("missing payload", status_key, vec![]),
            ("explicit empty payload", status_key, vec![0x0a, 0x00]),
            ("invalid UTF-8", status_key, envelope_bytes(&[0xff])),
            ("invalid JSON", status_key, envelope_bytes(b"{not JSON}")),
            ("schema-valid root/topic mismatch", status_key, envelope_bytes(commands)),
            ("missing required schema fields", status_key, envelope_bytes(b"{\"ServiceStatus\":{}}")),
            ("invalid schema enum", status_key, envelope_bytes(invalid_state.as_bytes())),
            ("unsupported root", status_key, envelope_bytes(b"{\"UnsupportedRoot\":{}}")),
            ("unsupported exact key", "la-cal/uci/002.5.0/UnsupportedRoot", envelope_bytes(fixture.as_bytes())),
            ("unknown field with schema-invalid payload", status_key, unknown_invalid),
            ("unknown field without payload", status_key, vec![0x10, 0x07]),
        ];
        let invalid_count = invalid_payloads.len();
        for (index, (case, key, bytes)) in invalid_payloads.into_iter().enumerate() {
            observer.put(key, bytes).await.unwrap();
            // A distinct valid message after each invalid sample is a positive
            // progress witness: rejection cannot pass merely because delivery
            // is broken. All traffic comes from this real Zenoh publisher.
            let marker = format!("\n{fixture}{}\n", " ".repeat(index + 1));
            observer.put(status_key, envelope_bytes(marker.as_bytes())).await.unwrap();
            let ServerOp::Msg { sid, payload } = read_operation(&mut ws).await else {
                panic!("{case}: valid ingress following rejection must deliver");
            };
            assert_eq!(sid, "status", "{case}");
            assert_eq!(&*payload, marker, "{case}: invalid input must not leak or change following original bytes");
        }
        let incoming_json = format!("\n {fixture} \n");
        let mut incoming_envelope = Envelope { oms_json: incoming_json.as_bytes().to_vec() }.encode_to_vec();
        incoming_envelope.extend([0x10, 0x07]); // Unknown field 2, varint 7.
        observer.put(status_key, incoming_envelope).await.unwrap();
        let ServerOp::Msg { sid, payload } = read_operation(&mut ws).await else {
            panic!("valid ASB ingress must deliver without an OWP acknowledgment");
        };
        assert_eq!(sid, "status");
        assert_eq!(&*payload, incoming_json, "inbound original bytes, following invalid samples");
        let logs = std::fs::read_to_string(&harness.log_path).unwrap();
        assert_eq!(logs.matches("discarding invalid ASB sample").count(), invalid_count,
            "each malformed ingress sample must be visibly reported: {logs}");
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
        // A new connection reuses the same connection-local SID and receives
        // externally injected ingress after the old connection closed with
        // registrations still active. Closing a client must not tear down the
        // server's shared startup listener.
        let mut replacement = harness.connect().await;
        negotiate(&mut replacement).await;
        replacement.send(Message::text("SUB again ServiceStatus ServiceStatus")).await.unwrap();
        assert!(matches!(read_operation(&mut replacement).await, ServerOp::Ok));
        observer.put(status_key, envelope_bytes(incoming_json.as_bytes())).await.unwrap();
        assert!(matches!(read_operation(&mut replacement).await, ServerOp::Msg { sid, payload }
            if sid == "again" && *payload == incoming_json));
        replacement.close(None).await.unwrap();
        println!("native ASB: OWP 1.0 / UCI 002.5.0 / PUB +OK and MSG / independent Zenoh envelope observation / {invalid_count} reported ingress rejections with valid progress witnesses / unknown-field valid payload / original bytes / listener survives UNSUB and client close");
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
        .args(["-TERM", &harness.server.0.id().to_string()])
        .status()
        .unwrap();
    assert!(signal.success());
    timeout(Duration::from_secs(5), async {
        loop {
            if let Some(status) = harness.server.0.try_wait().unwrap() {
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

async fn assert_topic_policy(ws: &mut Socket, fixture: &str) {
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn deployed_slow_socket_closes_while_healthy_recipient_continues() {
    let mut harness = NativeHarness::start().await;
    let observer = open_observer(harness.router_addr).await;
    let mut healthy = harness.connect().await;
    let mut slow = timeout(Duration::from_secs(5), async {
        let tcp = tokio::net::TcpSocket::new_v4().unwrap();
        tcp.set_recv_buffer_size(16 * 1024).unwrap();
        let stream = tcp.connect(harness.addr).await.unwrap();
        let mut request = format!("ws://{}/", harness.addr)
            .into_client_request()
            .unwrap();
        request
            .headers_mut()
            .insert("Sec-WebSocket-Protocol", "owp".parse().unwrap());
        tokio_tungstenite::client_async(request, tokio_tungstenite::MaybeTlsStream::Plain(stream))
            .await
            .unwrap()
            .0
    })
    .await
    .expect("bounded small-window slow socket setup");
    timeout(Duration::from_secs(5), async {
        for ws in [&mut healthy, &mut slow] {
            negotiate(ws).await;
            ws.send(Message::text("SUB status ServiceStatus ServiceStatus"))
                .await
                .unwrap();
            assert!(matches!(read_operation(ws).await, ServerOp::Ok));
        }
    })
    .await
    .unwrap();

    let fixture = include_str!("../../../fixtures/service-status.json");
    // Accepted whitespace fills the real TCP socket without introducing new
    // schema data or a test-only server hook. The healthy socket is drained
    // after every publication, so pressure is confined to the unread peer.
    let padded = format!("{fixture}{}", " ".repeat(128 * 1024));
    let key = "la-cal/uci/002.5.0/ServiceStatus";
    let published = timeout(Duration::from_secs(10), async {
        for count in 1..=256 {
            observer
                .put(key, envelope_bytes(padded.as_bytes()))
                .congestion_control(zenoh::qos::CongestionControl::Block)
                .await
                .unwrap();
            assert!(matches!(read_operation(&mut healthy).await, ServerOp::Msg { sid, payload }
                if sid == "status" && *payload == padded));
            if std::fs::read_to_string(&harness.log_path)
                .unwrap()
                .contains("outbound queue overflow")
            {
                return count;
            }
        }
        panic!("unread socket did not reach observable outbound overflow within the bounded traffic cap");
    })
    .await
    .expect("bounded pressure on the real slow recipient");
    timeout(Duration::from_secs(5), async {
        loop {
            match slow.next().await {
                Some(Ok(Message::Text(text))) => assert!(matches!(
                    parse_server(&text).unwrap(),
                    ServerOp::Msg { sid, .. } if sid == "status"
                )),
                Some(Ok(Message::Close(_)) | Err(_)) | None => break,
                other => panic!("unexpected slow recipient frame: {other:?}"),
            }
        }
        let _ = slow.flush().await;
        observer
            .put(key, envelope_bytes(fixture.as_bytes()))
            .await
            .unwrap();
        assert!(
            matches!(read_operation(&mut healthy).await, ServerOp::Msg { sid, payload }
            if sid == "status" && &*payload == fixture)
        );
        healthy.close(None).await.unwrap();
        observer.close().await.unwrap();
    })
    .await
    .expect("slow recipient must close while shared ingress still serves the healthy peer");
    assert!(harness.server.0.try_wait().unwrap().is_none());
    println!(
        "native slow socket: unread small receive window / reported overflow after {published} publications / failed socket closes / healthy recipient continues on real Zenoh ingress"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn deployed_router_loss_closes_clients_and_latches_unsuccessful_run() {
    let mut harness = NativeHarness::start().await;
    let observer = open_observer(harness.router_addr).await;
    let mut first = harness.connect().await;
    let mut second = harness.connect().await;
    let fixture = include_str!("../../../fixtures/service-status.json");
    timeout(Duration::from_secs(5), async {
        for ws in [&mut first, &mut second] {
            negotiate(ws).await;
            ws.send(Message::text("SUB status ServiceStatus ServiceStatus"))
                .await
                .unwrap();
            assert!(matches!(read_operation(ws).await, ServerOp::Ok));
        }
        // Prove both connections actually receive independent real-Zenoh
        // ingress before testing their response to infrastructure loss.
        observer
            .put(
                "la-cal/uci/002.5.0/ServiceStatus",
                envelope_bytes(fixture.as_bytes()),
            )
            .await
            .unwrap();
        for ws in [&mut first, &mut second] {
            assert!(
                matches!(read_operation(ws).await, ServerOp::Msg { sid, payload }
                if sid == "status" && &*payload == fixture)
            );
        }
        observer.close().await.unwrap();
    })
    .await
    .expect("both clients ready before router loss");

    harness
        .router
        .0
        .kill()
        .expect("kill the real deployed router");
    let status = harness.router.0.wait().expect("reap killed router");
    assert!(!status.success());

    // No PUB is in flight at loss. First closure is external evidence that
    // the server detected loss; an immediate post-kill PUB would race transport
    // detection and the 100 ms router poll, neither of which is instantaneous.
    timeout(Duration::from_secs(5), require_failure_close(&mut first))
        .await
        .expect("detected router loss must close idle OWP clients");

    // Restore the same endpoint as soon as loss is externally observed, before
    // probing the other client. Reconnection must not permit a new successful
    // publication or revive OWP admission during this run's cleanup.
    harness.router = start_router(&harness.router_path, harness.router_addr).await;
    timeout(Duration::from_secs(5), async {
        // A write failure or an Internal-Error/connection failure is acceptable.
        // A successful acknowledgment after observed loss is never acceptable.
        let _ = second
            .send(Message::text(format!("PUB ServiceStatus {fixture}")))
            .await;
        require_failure_close(&mut second).await;
    })
    .await
    .expect("remaining client must fail without a post-loss +OK");

    timeout(Duration::from_secs(5), async {
        loop {
            if let Some(status) = harness.server.0.try_wait().unwrap() {
                assert!(
                    !status.success(),
                    "detected router loss must make the process exit unsuccessful"
                );
                break;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("bounded unsuccessful shutdown despite a restored router endpoint");
    assert!(
        timeout(
            Duration::from_secs(5),
            tokio::net::TcpStream::connect(harness.addr)
        )
        .await
        .expect("bounded admission probe after failed run")
        .is_err(),
        "failed run cannot resume OWP admission"
    );
    println!(
        "native router loss: real ingress delivered to both clients / real zenohd killed and reaped / observed client closure / router restored / no post-detection +OK / bounded unsuccessful server exit"
    );
}

async fn require_failure_close(ws: &mut Socket) {
    loop {
        match ws.next().await {
            Some(Ok(Message::Close(_)) | Err(_)) | None => {
                // Reply to a close when possible; this does not wait for the
                // peer's transport and helps bounded server-side cleanup.
                let _ = ws.flush().await;
                break;
            }
            Some(Ok(Message::Text(text))) => assert!(
                matches!(
                    parse_server(&text).expect("valid failure response"),
                    ServerOp::Err {
                        error: OwpError::InternalError,
                        ..
                    }
                ),
                "after detected router loss only infrastructure failure is allowed, received {text}"
            ),
            Some(Ok(Message::Ping(_))) => ws.flush().await.unwrap(),
            other => panic!("unexpected frame after router loss: {other:?}"),
        }
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

async fn read_operation(ws: &mut Socket) -> ServerOp {
    match ws.next().await {
        Some(Ok(Message::Text(text))) => parse_server(&text).expect("valid OWP server frame"),
        other => panic!("expected OWP text, received {other:?}"),
    }
}
