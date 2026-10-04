//! Ticket-14 acceptance at the real process/WebSocket boundary.

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use sleet_types::owp::{ServerOp, parse_server};
use tokio::time::{sleep, timeout};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

struct OwnedServer(Child);

impl Drop for OwnedServer {
    fn drop(&mut self) {
        // Always reap the process, including assertion/panic paths.
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[tokio::test]
async fn native_process_negotiates_validates_and_delivers_service_status() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixture = std::fs::read_to_string(root.join("fixtures/service-status.json"))
        .expect("read independently specified UCI 002.5.0 fixture");
    let temp = tempfile::tempdir().expect("temporary launch configuration");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("choose loopback port");
    let addr = listener.local_addr().expect("local address");
    drop(listener);
    // Reuse the checked-in launch configuration, changing only the test port.
    let config = std::fs::read_to_string(root.join("config/server.toml"))
        .expect("read native configuration")
        .replace("127.0.0.1:9000", &addr.to_string());
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
        ws.send(Message::text("SUB status ServiceStatus baseline"))
            .await
            .unwrap();
        assert!(matches!(read_operation(&mut ws).await, ServerOp::Ok));
        ws.send(Message::text(format!("PUB baseline {fixture}")))
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
        ws.send(Message::text("UNSUB status")).await.unwrap();
        assert!(matches!(read_operation(&mut ws).await, ServerOp::Ok));
        ws.close(None).await.unwrap();
        println!("native baseline: owp / INIT+INFO 1.0 / UCI 002.5.0 / SUB +OK / PUB +OK / original-byte MSG / UNSUB +OK");
    })
    .await
    .expect("bounded OWP exchange");
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
