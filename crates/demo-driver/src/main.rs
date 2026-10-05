//! Process-owning native readiness and direct STOP demonstration.

mod processes;

use std::{
    net::{SocketAddr, TcpListener},
    path::{Path, PathBuf},
    process::ExitCode,
    time::Duration,
};

use demo_driver::{DemoConfig, Identity, shutdown_signal};
use futures_util::StreamExt;
use la_cal_client::{
    Client, Subscription,
    messages::uci_002_5::{
        CommandProcessingStateEnum, ServiceLifecycleCommandStatus,
        ServiceLifecycleCommandStatusEnum, ServiceStateEnum, ServiceStatus,
    },
};
use tokio::{
    process::Command,
    time::{Instant, sleep, timeout},
};

use processes::{ProcessExit, Processes};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("FAIL readiness/direct STOP: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let direct = match args.as_slice() {
        [arg] if arg == "--readiness" => false,
        [arg] if arg == "--direct-stop" => true,
        _ => {
            return Err(
                "usage: demo-driver --readiness | --direct-stop (relay follows in ticket 22)"
                    .into(),
            );
        }
    };
    let mut config = DemoConfig::from_env()?;
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let binaries = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("driver executable has no parent directory")?
        .to_path_buf();
    let directory = tempfile::tempdir().map_err(|e| e.to_string())?;
    let router_addr = unused_address()?;
    let server_addr = unused_address()?;
    config.endpoint = format!("ws://{server_addr}/");
    write_configuration(&config, &root, directory.path(), router_addr, server_addr)?;
    println!(
        "configuration: endpoint={} router=tcp/{router_addr} mission={} period={}s wait={}ms",
        config.endpoint,
        config.mission_uuid,
        config.status_period.as_secs(),
        config.wait.as_millis()
    );
    let (mut processes, mut exits) = Processes::new(config.wait);
    let mut connection = None;
    let outcome = tokio::select! {
        result = async {
            let (mut statuses, mut results) = tokio::select! {
                result = readiness(&config, &binaries, directory.path(), router_addr, server_addr, &mut processes, &mut connection) => result?,
                exit = exits.recv() => return Err(exit.map_or_else(|| "process observation ended".into(), |exit| exit.unexpected())),
            };
            if direct {
                let client = connection.as_ref().ok_or("missing driver client")?;
                direct_stop(&config, client, &mut statuses, &mut results, &mut exits).await
            } else {
                Ok(())
            }
        } => result,
        signal = shutdown_signal() => Err(signal.err().unwrap_or_else(|| "driver interrupted".into())),
    };
    // Always close the independent observer and reap every successfully launched
    // child, even if setup was canceled between operations or process launches.
    let close = if let Some(client) = connection {
        client
            .close()
            .await
            .map_err(|e| format!("driver connection cleanup: {e}"))
    } else {
        Ok(())
    };
    // Recheck an exit already queued at the readiness/cleanup boundary.
    let outcome = outcome.and(close).and_then(|()| {
        exits
            .try_recv()
            .map_or(Ok(()), |exit| Err(exit.unexpected()))
    });
    let cleanup = processes.cleanup(outcome.is_err()).await;
    outcome.and(cleanup)?;
    if direct {
        println!(
            "PASS direct STOP: terminal result, INOPERABLE and process exit independently observed; Orchestrator remains NORMAL; all owned processes reaped"
        );
    } else {
        println!(
            "PASS readiness: both distinct Services NORMAL and periodically reporting; all owned processes reaped"
        );
    }
    Ok(())
}

async fn readiness(
    config: &DemoConfig,
    binaries: &Path,
    directory: &Path,
    router_addr: SocketAddr,
    server_addr: SocketAddr,
    processes: &mut Processes,
    connection: &mut Option<Client>,
) -> Result<
    (
        Subscription<ServiceStatus>,
        Subscription<ServiceLifecycleCommandStatus>,
    ),
    String,
> {
    let mut router = Command::new(std::env::var_os("ZENOHD").unwrap_or_else(|| "zenohd".into()));
    router
        .arg("-c")
        .arg(directory.join("router.json5"))
        .env("RUST_LOG", "error");
    processes.launch("router", router, directory)?;
    listener_ready(router_addr, config.wait).await?;
    let mut server = application_command("la-cal", binaries);
    server
        .arg(directory.join("server.toml"))
        .env("RUST_LOG", "warn");
    processes.launch("la-cal", server, directory)?;
    listener_ready(server_addr, config.wait).await?;
    let client = Client::connect(config.client_config(&config.driver))
        .await
        .map_err(|e| e.to_string())?;
    *connection = Some(client.clone());
    let mut statuses = client
        .subscribe::<ServiceStatus>()
        .await
        .map_err(|e| e.to_string())?;
    let mut results = client
        .subscribe::<ServiceLifecycleCommandStatus>()
        .await
        .map_err(|e| e.to_string())?;
    println!("observations installed before application launch");
    for name in ["orchestrator", "commandee-service"] {
        let mut application = application_command(name, binaries);
        application.env("DEMO_ENDPOINT", &config.endpoint);
        processes.launch(name, application, directory)?;
    }
    let mut services = [
        Progress::new("orchestrator", &config.orchestrator, config.wait),
        Progress::new("commandee-service", &config.commandee, config.wait),
    ];
    observe_readiness(config, &mut statuses, &mut results, &mut services).await?;
    Ok((statuses, results))
}

fn application_command(name: &str, binaries: &Path) -> Command {
    let variable = format!("DEMO_{}_BIN", name.replace('-', "_").to_uppercase());
    Command::new(std::env::var_os(variable).map_or_else(|| binaries.join(name), PathBuf::from))
}

async fn direct_stop(
    config: &DemoConfig,
    client: &Client,
    statuses: &mut Subscription<ServiceStatus>,
    results: &mut Subscription<ServiceLifecycleCommandStatus>,
    exits: &mut tokio::sync::mpsc::UnboundedReceiver<ProcessExit>,
) -> Result<(), String> {
    if let Ok(exit) = exits.try_recv() {
        return Err(exit.unexpected());
    }
    let command = config.stop_command(&config.driver, &config.commandee, &config.commandee)?;
    let command_id = &command.message.message_data.command_id.uuid;
    client
        .send(&command)
        .await
        .map_err(|e| format!("direct STOP publication: {e}"))?;
    println!("direct STOP publication acknowledged command={command_id}");
    let mut result_seen = false;
    let mut state_seen = false;
    let mut exit_seen = false;
    let deadline = Instant::now() + config.wait;
    // Each required milestone has its own wait; unrelated reports never extend it.
    while !(result_seen && state_seen && exit_seen) {
        tokio::select! {
            () = tokio::time::sleep_until(deadline) => {
                let missing = [(result_seen, "terminal result"), (state_seen, "INOPERABLE"), (exit_seen, "process exit")]
                    .into_iter().filter_map(|(seen, name)| (!seen).then_some(name)).collect::<Vec<_>>().join(", ");
                return Err(format!("direct STOP deadline waiting for {missing}"));
            }
            result = results.next() => {
                let result = result.ok_or("driver result stream ended before direct STOP completed")?.map_err(|e| e.to_string())?;
                let header = &result.message.message_header;
                let data = &result.message.message_data;
                if data.command_id.uuid != *command_id || data.service_id.uuid != config.commandee.uuid {
                    return Err("unexpected direct command result identity/correlation".into());
                }
                check_header(header, config, &config.commandee)?;
                if result_seen { return Err("duplicate terminal direct command result".into()); }
                if data.command_processing_state != CommandProcessingStateEnum::Accepted || data.lifecycle_command_status != ServiceLifecycleCommandStatusEnum::Success {
                    return Err(format!("direct STOP failed: {:?}/{:?}", data.command_processing_state, data.lifecycle_command_status));
                }
                if header.timestamp == command.message.message_header.timestamp {
                    return Err("direct result reused command header timestamp".into());
                }
                println!("observed direct result: ACCEPTED/SUCCESS command={command_id} processor={}", data.service_id.uuid);
                result_seen = true;
            }
            report = statuses.next() => {
                let report = report.ok_or("driver status stream ended before direct STOP completed")?.map_err(|e| e.to_string())?;
                let data = &report.message.message_data;
                if data.service_id.uuid == config.commandee.uuid {
                    check_header(&report.message.message_header, config, &config.commandee)?;
                    match data.service_state {
                        ServiceStateEnum::Inoperable => {
                            if state_seen { return Err("duplicate final INOPERABLE report".into()); }
                            println!("observed commandee-service: INOPERABLE uuid={}", data.service_id.uuid);
                            state_seen = true;
                        }
                        ServiceStateEnum::Normal if !state_seen => (),
                        _ => return Err(format!("unexpected Commandee state {:?}", data.service_state)),
                    }
                } else if data.service_id.uuid == config.orchestrator.uuid {
                    check_header(&report.message.message_header, config, &config.orchestrator)?;
                    if data.service_state != ServiceStateEnum::Normal { return Err("Orchestrator did not remain NORMAL".into()); }
                } else { return Err("unexpected reporting Service during direct STOP".into()); }
            }
            exit = exits.recv() => {
                let exit = exit.ok_or("process observation ended")?;
                if exit.name != "commandee-service" || !exit.status.as_ref().is_ok_and(std::process::ExitStatus::success) {
                    return Err(exit.unexpected());
                }
                println!("observed commandee-service: successful process exit");
                exit_seen = true;
                // The process can exit while its already-delivered MSGs remain buffered.
                // Keep requiring independent result/state observations within their deadlines.
            }
        }
    }
    // Observe a later periodic Orchestrator report, rather than infer availability
    // from the lack of an exit event. Keep duplicate results visible during this wait.
    let fresh_after = time::OffsetDateTime::now_utc();
    timeout(config.wait, async {
        loop {
            tokio::select! {
                result = results.next() => {
                    result.ok_or("driver result stream ended")?.map_err(|e| e.to_string())?;
                    return Err("unexpected additional command result after direct STOP".into());
                }
                exit = exits.recv() => return Err(exit.map_or_else(|| "process observation ended".into(), |exit| exit.unexpected())),
                report = statuses.next() => {
                    let report = report.ok_or("driver status stream ended")?.map_err(|e| e.to_string())?;
                    if report.message.message_data.service_id.uuid != config.orchestrator.uuid {
                        return Err("unexpected Commandee report after final INOPERABLE".into());
                    }
                    check_header(&report.message.message_header, config, &config.orchestrator)?;
                    if report.message.message_data.service_state != ServiceStateEnum::Normal {
                        return Err("Orchestrator did not remain NORMAL".into());
                    }
                    let timestamp = time::OffsetDateTime::parse(&report.message.message_header.timestamp,
                        &time::format_description::well_known::Rfc3339).map_err(|e| format!("Orchestrator report timestamp: {e}"))?;
                    if timestamp <= fresh_after { continue; }
                    println!("observed orchestrator: still NORMAL after direct STOP");
                    return Ok(());
                }
            }
        }
    }).await.map_err(|_| "Orchestrator post-STOP reporting deadline".to_string())?
}

fn check_header(
    header: &la_cal_client::messages::uci_002_5::HeaderType,
    config: &DemoConfig,
    sender: &Identity,
) -> Result<(), String> {
    if header.service_id.as_ref().map(|id| &id.uuid) != Some(&sender.uuid)
        || header.system_id.uuid != config.system_uuid
        || header.mission_id.as_ref().map(|id| &id.uuid) != Some(&config.mission_uuid)
    {
        Err(format!(
            "{} report has inconsistent sender/System/Mission",
            sender.participant
        ))
    } else {
        Ok(())
    }
}

fn unused_address() -> Result<SocketAddr, String> {
    TcpListener::bind("127.0.0.1:0")
        .and_then(|listener| listener.local_addr())
        .map_err(|e| e.to_string())
}

async fn listener_ready(addr: SocketAddr, wait: Duration) -> Result<(), String> {
    timeout(wait, async {
        loop {
            match tokio::net::TcpStream::connect(addr).await {
                Ok(_) => return Ok(()),
                Err(error) if error.kind() == std::io::ErrorKind::ConnectionRefused => {
                    // Poll the readiness condition, never assume startup after a fixed delay.
                    sleep(Duration::from_millis(20)).await;
                }
                Err(error) => return Err(format!("listener {addr}: {error}")),
            }
        }
    })
    .await
    .map_err(|_| format!("listener {addr} readiness deadline"))?
}

fn write_configuration(
    config: &DemoConfig,
    root: &Path,
    directory: &Path,
    router_addr: SocketAddr,
    server_addr: SocketAddr,
) -> Result<(), String> {
    let services_dir = directory.join("services");
    std::fs::create_dir(&services_dir).map_err(|e| e.to_string())?;
    for (index, identity) in [&config.driver, &config.orchestrator, &config.commandee]
        .iter()
        .enumerate()
    {
        let participant = toml::toml! {
            service_id = (identity.participant.clone())
            service_uuid = (identity.uuid.clone())
            allowed_topics = ["ServiceLifecycleCommand", "ServiceLifecycleCommandStatus", "ServiceStatus"]
        };
        std::fs::write(
            services_dir.join(format!("{index}.toml")),
            toml::to_string(&participant).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    let mut server: toml::Table =
        toml::from_str(include_str!("../../../config/server.toml")).map_err(|e| e.to_string())?;
    for (key, value) in [
        ("bind_addr", server_addr.to_string()),
        ("zenoh_router", format!("tcp/{router_addr}")),
        ("system_uuid", config.system_uuid.clone()),
        ("services_dir", services_dir.to_string_lossy().into_owned()),
        (
            "schema_path",
            root.join("spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd")
                .to_string_lossy()
                .into_owned(),
        ),
    ] {
        server.insert(key.into(), toml::Value::String(value));
    }
    // Server ASB settings have a narrower public range than demo event waits.
    let server_wait =
        i64::try_from(config.wait.as_millis().min(60_000)).map_err(|e| e.to_string())?;
    for key in [
        "asb_operation_timeout_ms",
        "asb_startup_timeout_ms",
        "asb_cleanup_timeout_ms",
    ] {
        server.insert(key.into(), toml::Value::Integer(server_wait));
    }
    std::fs::write(
        directory.join("server.toml"),
        toml::to_string(&server).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let router = include_str!("../../../config/zenoh-router.json5")
        .replace("127.0.0.1:7447", &router_addr.to_string());
    std::fs::write(directory.join("router.json5"), router).map_err(|e| e.to_string())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReadinessStage {
    Initializing,
    Normal,
    PeriodicNormal,
    Ready,
}

impl ReadinessStage {
    const fn label(self) -> &'static str {
        match self {
            Self::Initializing => "INITIALIZING",
            Self::Normal => "NORMAL",
            Self::PeriodicNormal => "periodic NORMAL",
            Self::Ready => "ready",
        }
    }

    const fn next(self) -> Self {
        match self {
            Self::Initializing => Self::Normal,
            Self::Normal => Self::PeriodicNormal,
            Self::PeriodicNormal | Self::Ready => Self::Ready,
        }
    }
}

struct Progress<'a> {
    name: &'static str,
    identity: &'a Identity,
    stage: ReadinessStage,
    deadline: Instant,
    last_timestamp: Option<String>,
}

impl<'a> Progress<'a> {
    fn new(name: &'static str, identity: &'a Identity, wait: Duration) -> Self {
        Self {
            name,
            identity,
            stage: ReadinessStage::Initializing,
            deadline: Instant::now() + wait,
            last_timestamp: None,
        }
    }
}

async fn observe_readiness(
    config: &DemoConfig,
    statuses: &mut Subscription<ServiceStatus>,
    results: &mut Subscription<ServiceLifecycleCommandStatus>,
    services: &mut [Progress<'_>; 2],
) -> Result<(), String> {
    while services
        .iter()
        .any(|service| service.stage != ReadinessStage::Ready)
    {
        let deadline = services
            .iter()
            .filter(|service| service.stage != ReadinessStage::Ready)
            .map(|service| service.deadline)
            .min()
            .ok_or("missing readiness deadline")?;
        tokio::select! {
            () = tokio::time::sleep_until(deadline) => {
                let missing = services.iter().filter(|s| s.stage != ReadinessStage::Ready && s.deadline <= Instant::now())
                    .map(|s| format!("{} {}", s.name, s.stage.label())).collect::<Vec<_>>().join(", ");
                return Err(format!("readiness deadline waiting for {missing}"));
            }
            result = results.next() => {
                result.ok_or("driver result stream ended unexpectedly")?.map_err(|e| e.to_string())?;
                return Err("unexpected command result before command phase".into());
            }
            report = statuses.next() => {
                let report = report.ok_or("driver status stream ended unexpectedly")?.map_err(|e| e.to_string())?;
                let body = report.message;
                let header = body.message_header;
                let data = body.message_data;
                let Some(service) = services.iter_mut().find(|s| s.identity.uuid == data.service_id.uuid) else {
                    return Err(format!("unexpected reporting Service {}", data.service_id.uuid));
                };
                if header.service_id.as_ref().map(|id| &id.uuid) != Some(&service.identity.uuid)
                    || header.system_id.uuid != config.system_uuid
                    || header.mission_id.as_ref().map(|id| &id.uuid) != Some(&config.mission_uuid) {
                    return Err(format!("{} report has inconsistent sender/System/Mission", service.name));
                }
                if service.last_timestamp.as_ref() == Some(&header.timestamp) {
                    return Err(format!("{} duplicate report timestamp", service.name));
                }
                let expected = if service.stage == ReadinessStage::Initializing { ServiceStateEnum::Initializing } else { ServiceStateEnum::Normal };
                if data.service_state != expected {
                    return Err(format!("{} expected {expected:?}, received {:?}", service.name, data.service_state));
                }
                if service.stage != ReadinessStage::Ready {
                    println!("observed {}: {} uuid={} timestamp={} uptime={}", service.name, service.stage.label(), service.identity.uuid, header.timestamp, data.time_up);
                    service.stage = service.stage.next();
                    service.deadline = Instant::now() + config.wait;
                }
                service.last_timestamp = Some(header.timestamp);
            }
        }
    }
    Ok(())
}
