//! Real Commandee process and typed public messages through native Zenoh/Protobuf.

use std::{
    net::{SocketAddr, TcpListener},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::Duration,
};

use demo_driver::DemoConfig;
use futures_util::StreamExt;
use la_cal_client::{
    Client,
    messages::uci_002_5::{
        CommandProcessingStateEnum, CommandStateEnum, ServiceLifecycleCommand,
        ServiceLifecycleCommandStatus, ServiceLifecycleCommandStatusEnum,
        ServiceLifecycleStateEnum, ServiceStateEnum, ServiceStatus,
    },
};
use tokio::time::{sleep, timeout};

struct Owned(Child);
impl Drop for Owned {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

struct Deployment {
    children: Vec<Owned>,
    directory: tempfile::TempDir,
    root: PathBuf,
}

impl Deployment {
    fn launch(&mut self, executable: &Path, args: &[&Path], config: &DemoConfig) {
        let mut command = Command::new(executable);
        for (key, _) in
            std::env::vars_os().filter(|(key, _)| key.to_string_lossy().starts_with("DEMO_"))
        {
            command.env_remove(key);
        }
        self.children.push(Owned(
            command
                .args(args)
                .current_dir(&self.root)
                .env("DEMO_ENDPOINT", &config.endpoint)
                .env("RUST_LOG", "error")
                .stdin(Stdio::null())
                .spawn()
                .expect("launch deployed process"),
        ));
    }
}

fn address() -> SocketAddr {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
}

async fn listener_ready(addr: SocketAddr) {
    timeout(Duration::from_secs(5), async {
        while tokio::net::TcpStream::connect(addr).await.is_err() {
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("native listener readiness");
}

async fn start() -> (Deployment, DemoConfig, Client) {
    // Fixed literal identities keep these assertions independent of ambient overrides.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut deployment = Deployment {
        children: vec![],
        directory: tempfile::tempdir().unwrap(),
        root,
    };
    let router = address();
    let server = address();
    let config = DemoConfig {
        driver: demo_driver::Identity {
            participant: "demo-driver".into(),
            uuid: "123e4567-e89b-12d3-a456-426614174010".into(),
        },
        orchestrator: demo_driver::Identity {
            participant: "orchestrator".into(),
            uuid: "123e4567-e89b-12d3-a456-426614174011".into(),
        },
        commandee: demo_driver::Identity {
            participant: "commandee-service".into(),
            uuid: "123e4567-e89b-12d3-a456-426614174012".into(),
        },
        system_uuid: "123e4567-e89b-12d3-a456-426614174000".into(),
        mission_uuid: "123e4567-e89b-12d3-a456-426614174020".into(),
        status_period: Duration::from_secs(1),
        wait: Duration::from_secs(5),
        endpoint: format!("ws://{server}/"),
    };
    let path = deployment.directory.path();
    std::fs::create_dir(path.join("services")).unwrap();
    for identity in [&config.driver, &config.commandee] {
        std::fs::write(path.join("services").join(format!("{}.toml", identity.participant)), format!(
            "service_id = {:?}\nservice_uuid = {:?}\nallowed_topics = [\"ServiceLifecycleCommand\", \"ServiceLifecycleCommandStatus\", \"ServiceStatus\"]\n",
            identity.participant, identity.uuid)).unwrap();
    }
    let server_config = std::fs::read_to_string(deployment.root.join("config/server.toml"))
        .unwrap()
        .replace("127.0.0.1:9000", &server.to_string())
        .replace("tcp/127.0.0.1:7447", &format!("tcp/{router}"))
        .replace("config/services.d", path.join("services").to_str().unwrap());
    let server_path = path.join("server.toml");
    std::fs::write(&server_path, server_config).unwrap();
    let router_path = path.join("router.json5");
    std::fs::write(
        &router_path,
        std::fs::read_to_string(deployment.root.join("config/zenoh-router.json5"))
            .unwrap()
            .replace("127.0.0.1:7447", &router.to_string()),
    )
    .unwrap();
    let router_bin = PathBuf::from(std::env::var_os("ZENOHD").unwrap_or_else(|| "zenohd".into()));
    deployment.launch(&router_bin, &[Path::new("-c"), &router_path], &config);
    listener_ready(router).await;
    let binaries = deployment.root.join("target/debug");
    deployment.launch(&binaries.join("la-cal"), &[&server_path], &config);
    listener_ready(server).await;
    let client = Client::connect(config.client_config(&config.driver))
        .await
        .unwrap();
    (deployment, config, client)
}

#[tokio::test]
async fn commandee_ignores_other_processors_rejects_ineligible_requests_and_then_stops() {
    let (mut deployment, config, client) = start().await;
    let mut statuses = client.subscribe::<ServiceStatus>().await.unwrap();
    let mut results = client
        .subscribe::<ServiceLifecycleCommandStatus>()
        .await
        .unwrap();
    deployment.launch(
        &deployment.root.join("target/debug/commandee-service"),
        &[],
        &config,
    );
    timeout(config.wait, async {
        loop {
            let report = statuses.next().await.unwrap().unwrap();
            if report.message.message_data.service_state == ServiceStateEnum::Normal {
                break;
            }
        }
    })
    .await
    .unwrap();

    let other = config
        .stop_command(&config.driver, &config.orchestrator, &config.commandee)
        .unwrap();
    client.send(&other).await.unwrap();
    for case in [
        "restart",
        "update",
        "data-mission",
        "header-mission",
        "other-affected",
        "no-effecting",
        "system",
        "multiple-affected",
    ] {
        let command = ineligible_command(&config, case);
        client.send(&command).await.unwrap();
        let result = timeout(config.wait, results.next())
            .await
            .expect("targeted rejection deadline")
            .unwrap()
            .unwrap();
        assert_rejected(&result, &command);
    }
    // A real later NORMAL report, not merely absence of process exit, establishes
    // rejection did not stop the Service or its reporting.
    let report = timeout(config.wait, statuses.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(
        report.message.message_data.service_state,
        ServiceStateEnum::Normal
    );
    let command = config
        .stop_command(&config.driver, &config.commandee, &config.commandee)
        .unwrap();
    client.send(&command).await.unwrap();
    let result = timeout(config.wait, results.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(
        result.message.message_data.command_id,
        command.message.message_data.command_id
    );
    assert_eq!(
        result.message.message_data.command_processing_state,
        CommandProcessingStateEnum::Accepted
    );
    assert_eq!(
        result.message.message_data.lifecycle_command_status,
        ServiceLifecycleCommandStatusEnum::Success
    );
    timeout(config.wait, async {
        loop {
            let report = statuses.next().await.unwrap().unwrap();
            if report.message.message_data.service_state == ServiceStateEnum::Inoperable {
                break;
            }
        }
    })
    .await
    .expect("independent final state");
    let child = &mut deployment.children.last_mut().unwrap().0;
    timeout(config.wait, async {
        loop {
            if let Some(status) = child.try_wait().unwrap() {
                assert!(status.success());
                break;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("independent successful process exit");
    client.close().await.unwrap();
}

fn ineligible_command(config: &DemoConfig, case: &str) -> ServiceLifecycleCommand {
    let mut command = config
        .stop_command(&config.driver, &config.commandee, &config.commandee)
        .unwrap();
    let header = &mut command.message.message_header;
    let data = &mut command.message.message_data;
    match case {
        "restart" => data.lifecycle_state = ServiceLifecycleStateEnum::Restart,
        "update" => data.command_state = CommandStateEnum::Update,
        "data-mission" => data.mission_id.uuid.clone_from(&config.driver.uuid),
        "header-mission" => header
            .mission_id
            .as_mut()
            .unwrap()
            .uuid
            .clone_from(&config.driver.uuid),
        "other-affected" => data.affected_service_id = vec![config.orchestrator.service_id()],
        "no-effecting" => data.effecting_service_id = None,
        "system" => header.system_id.uuid.clone_from(&config.driver.uuid),
        "multiple-affected" => data
            .affected_service_id
            .push(config.orchestrator.service_id()),
        _ => panic!("unknown ineligible test case {case}"),
    }
    command
}

fn assert_rejected(result: &ServiceLifecycleCommandStatus, command: &ServiceLifecycleCommand) {
    let data = &result.message.message_data;
    assert_eq!(data.command_id, command.message.message_data.command_id);
    assert_eq!(data.service_id.uuid, "123e4567-e89b-12d3-a456-426614174012");
    assert_eq!(
        data.command_processing_state,
        CommandProcessingStateEnum::Rejected
    );
    assert_eq!(
        data.lifecycle_command_status,
        ServiceLifecycleCommandStatusEnum::Fail
    );
    let json = serde_json::to_value(result).unwrap();
    let reason =
        &json["ServiceLifecycleCommandStatus"]["MessageData"]["CommandProcessingStateReason"];
    assert_eq!(reason["Reason"], "INVALID_INPUT_PARAMETER");
    assert!(
        reason["Description"]
            .as_str()
            .is_some_and(|text| !text.is_empty())
    );
    assert!(reason.get("AssociatedID").is_none());
}
