//! Public process / scripted OWP peer checks for final publication completion.
//! This seam does not supply Zenoh/Protobuf evidence; deployed driver checks do.
#![allow(clippy::result_large_err)] // Tungstenite's handshake response type.

use std::{process::Stdio, time::Duration};

use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpListener, process::Command, time::timeout};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

type Peer = WebSocketStream<tokio::net::TcpStream>;
const COMMAND_ID: &str = "123e4567-e89b-12d3-a456-426614174030";

async fn text(peer: &mut Peer) -> String {
    timeout(Duration::from_secs(3), peer.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap()
        .to_string()
}

async fn ready() -> (tokio::process::Child, Peer, String) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_commandee-service"));
    for (key, _) in
        std::env::vars_os().filter(|(key, _)| key.to_string_lossy().starts_with("DEMO_"))
    {
        command.env_remove(key);
    }
    let child = command
        .env(
            "DEMO_ENDPOINT",
            format!("ws://{}/", listener.local_addr().unwrap()),
        )
        .env("DEMO_WAIT_MS", "1000")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let (tcp, _) = timeout(Duration::from_secs(3), listener.accept())
        .await
        .unwrap()
        .unwrap();
    let mut peer = tokio_tungstenite::accept_hdr_async(
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
    assert!(text(&mut peer).await.starts_with("INIT "));
    peer.send(Message::text("+OK")).await.unwrap();
    peer.send(Message::text(r#"INFO {"version":"1.0","server_id":"test-peer","uuids":{"system":"123e4567-e89b-12d3-a456-426614174000","service":"123e4567-e89b-12d3-a456-426614174001"},"system_label":"test"}"#)).await.unwrap();
    assert!(
        text(&mut peer)
            .await
            .contains("\"ServiceState\":\"INITIALIZING\"")
    );
    peer.send(Message::text("+OK")).await.unwrap();
    let sub = text(&mut peer).await;
    assert!(
        sub.starts_with("SUB ") && sub.ends_with(" ServiceLifecycleCommand"),
        "{sub}"
    );
    let sid = sub.split_whitespace().nth(1).unwrap().to_owned();
    peer.send(Message::text("+OK")).await.unwrap();
    assert!(
        text(&mut peer)
            .await
            .contains("\"ServiceState\":\"NORMAL\"")
    );
    peer.send(Message::text("+OK")).await.unwrap();
    (child, peer, sid)
}

async fn stop(peer: &mut Peer, sid: &str) {
    let command = serde_json::json!({ "ServiceLifecycleCommand": {
        "SecurityInformation": { "Classification": "U", "OwnerProducer": [{ "GovernmentIdentifier": "USA" }] },
        "MessageHeader": { "SystemID": { "UUID": "123e4567-e89b-12d3-a456-426614174000" },
            "ServiceID": { "UUID": "123e4567-e89b-12d3-a456-426614174010" },
            "MissionID": { "UUID": "123e4567-e89b-12d3-a456-426614174020" },
            "Timestamp": "2026-10-05T00:00:00Z", "SchemaVersion": "002.5.0", "Mode": "LIVE" },
        "MessageData": { "CommandID": { "UUID": COMMAND_ID }, "CommandState": "NEW",
            "MissionID": { "UUID": "123e4567-e89b-12d3-a456-426614174020" },
            "EffectingServiceID": { "UUID": "123e4567-e89b-12d3-a456-426614174012" },
            "AffectedServiceID": [{ "UUID": "123e4567-e89b-12d3-a456-426614174012" }], "LifecycleState": "STOP" }
    }});
    peer.send(Message::text(format!("MSG {sid} {command}")))
        .await
        .unwrap();
}

async fn finish(peer: &mut Peer) {
    // Dropping the application command stream requests best-effort UNSUB.
    loop {
        match timeout(Duration::from_secs(3), peer.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap()
        {
            Message::Close(_) => {
                peer.flush().await.unwrap();
                return;
            }
            Message::Text(text) if text.starts_with("UNSUB ") => {
                peer.send(Message::text("+OK")).await.unwrap();
            }
            other => panic!("unexpected publication/retry during closure: {other:?}"),
        }
    }
}

#[tokio::test]
async fn commandee_does_not_close_or_exit_before_both_final_publications_complete() {
    let (mut child, mut peer, sid) = ready().await;
    stop(&mut peer, &sid).await;
    let publication = text(&mut peer).await;
    assert!(publication.starts_with("PUB ServiceLifecycleCommandStatus "));
    let json: serde_json::Value =
        serde_json::from_str(publication.splitn(3, ' ').nth(2).unwrap()).unwrap();
    let data = &json["ServiceLifecycleCommandStatus"]["MessageData"];
    assert_eq!(data["CommandID"]["UUID"], COMMAND_ID);
    assert_eq!(
        data["ServiceID"]["UUID"],
        "123e4567-e89b-12d3-a456-426614174012"
    );
    assert_eq!(data["CommandProcessingState"], "ACCEPTED");
    assert_eq!(data["LifecycleCommandStatus"], "SUCCESS");
    assert!(child.try_wait().unwrap().is_none());
    assert!(
        timeout(Duration::from_millis(40), peer.next())
            .await
            .is_err(),
        "no second publication/close before result acknowledgment"
    );
    peer.send(Message::text("+OK")).await.unwrap();
    let publication = text(&mut peer).await;
    assert!(publication.starts_with("PUB ServiceStatus "));
    let json: serde_json::Value =
        serde_json::from_str(publication.splitn(3, ' ').nth(2).unwrap()).unwrap();
    assert_eq!(
        json["ServiceStatus"]["MessageData"]["ServiceState"],
        "INOPERABLE"
    );
    assert!(
        json["ServiceStatus"]["MessageData"]
            .get("CommandID")
            .is_none()
    );
    assert!(child.try_wait().unwrap().is_none());
    assert!(
        timeout(Duration::from_millis(40), peer.next())
            .await
            .is_err(),
        "no close before final state acknowledgment"
    );
    peer.send(Message::text("+OK")).await.unwrap();
    finish(&mut peer).await;
    assert!(
        timeout(Duration::from_secs(3), child.wait())
            .await
            .unwrap()
            .unwrap()
            .success()
    );
}

#[tokio::test]
async fn failed_final_publication_is_visible_and_exits_unsuccessfully_without_retry() {
    for fail_state in [false, true] {
        let (child, mut peer, sid) = ready().await;
        stop(&mut peer, &sid).await;
        assert!(
            text(&mut peer)
                .await
                .starts_with("PUB ServiceLifecycleCommandStatus ")
        );
        if fail_state {
            peer.send(Message::text("+OK")).await.unwrap();
            assert!(
                text(&mut peer)
                    .await
                    .contains("\"ServiceState\":\"INOPERABLE\"")
            );
        }
        peer.send(Message::text(
            "-ERR Internal-Error final-publication-rejected",
        ))
        .await
        .unwrap();
        finish(&mut peer).await;
        let output = timeout(Duration::from_secs(3), child.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains(if fail_state {
                "final INOPERABLE publication failed"
            } else {
                "terminal result publication failed"
            }),
            "{stderr}"
        );
        assert!(stderr.contains("final-publication-rejected"), "{stderr}");
    }
}
