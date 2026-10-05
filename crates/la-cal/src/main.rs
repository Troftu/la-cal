//! Project-owned LA-CAL adoption baseline.
//!
//! Modified 2026-10-04 from SLEET e3c2d798a229766571ed7be1f5b28d2dcdb51868:
//! project executable name; baseline transport behavior retained. See ../../../NOTICE.
//! Modified 2026-10-05: real ASB startup/delivery, await server cleanup on admin
//! failure, and finish bounded runtime shutdown before returning a failure exit.
//!
//! Loads configuration and an OMS XSD schema at startup, then listens for
//! WebSocket connections using the OWP subprotocol. Each connection performs
//! an INIT handshake and can then PUB/SUB/UNSUB OMS JSON messages with
//! structural validation against the loaded schema.

mod admin;
mod asb;
mod config;
mod facets;
mod router;
mod schema;
mod server;
mod telemetry;
mod validator;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::time::Instant;

use futures_util::future::{Either, select};
use futures_util::pin_mut;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// Default path used when no explicit configuration path is passed.
const DEFAULT_CONFIG_PATH: &str = "config/server.toml";

/// CLI action resolved from command-line arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
enum CliAction {
    /// Run the server using the provided configuration path.
    Run { config_path: String },
    /// Execute a simple HTTP health check against the local admin listener.
    HealthCheck { port: u16 },
    /// Print usage and exit successfully.
    Help,
    /// Print version and exit successfully.
    Version,
}

/// Parse command-line arguments into a high-level action.
fn parse_cli_args<I, E>(args: I, env_var: E) -> Result<CliAction, String>
where
    I: IntoIterator<Item = String>,
    E: Fn(&str) -> Result<String, std::env::VarError>,
{
    let mut iter = args.into_iter();
    let _program = iter.next();

    let Some(first) = iter.next() else {
        return Ok(CliAction::Run {
            config_path: DEFAULT_CONFIG_PATH.to_owned(),
        });
    };

    if first == "--health-check" {
        if let Some(extra) = iter.next() {
            return Err(format!("unexpected argument: {extra}"));
        }
        match env_var("SLEET_METRICS_PORT") {
            Ok(port_str) => {
                let port: u16 = port_str
                    .parse()
                    .map_err(|_| format!("invalid port in SLEET_METRICS_PORT: {port_str}"))?;
                return Ok(CliAction::HealthCheck { port });
            }
            Err(_) => {
                return Err(
                    "SLEET_METRICS_PORT environment variable is required for health check"
                        .to_owned(),
                );
            }
        }
    }
    if first == "--help" || first == "-h" {
        return Ok(CliAction::Help);
    }
    if first == "--version" || first == "-V" {
        return Ok(CliAction::Version);
    }

    if first.starts_with('-') {
        return Err(format!("unknown option: {first}"));
    }

    if let Some(extra) = iter.next() {
        return Err(format!("unexpected argument: {extra}"));
    }

    Ok(CliAction::Run { config_path: first })
}

/// Print command usage information.
fn print_usage() {
    println!(
        "Usage: la-cal [OPTIONS] [CONFIG_PATH]\n\
         \n\
         Arguments:\n\
           CONFIG_PATH     Path to server TOML config (default: {DEFAULT_CONFIG_PATH})\n\
         \n\
         Options:\n\
           --health-check         Run a fast local TCP health check (requires SLEET_METRICS_PORT)\n\
           -h, --help             Print help\n\
           -V, --version          Print version"
    );
}

/// Load and validate the server configuration.
fn load_config(config_path: &str) -> Result<Arc<config::Config>, String> {
    config::load(std::path::Path::new(config_path))
        .map(Arc::new)
        .map_err(|e| format!("configuration error: {e}"))
}

/// Install the final tracing subscriber using loaded configuration.
fn init_configured_tracing(config: &config::Config) -> Result<(), String> {
    let log_format = if config.server.log_format == "json" {
        telemetry::LogFormat::Json
    } else {
        telemetry::LogFormat::Text
    };

    telemetry::init_tracing(
        log_format,
        config.server.log_level.as_deref(),
        config.server.otlp_endpoint.as_deref(),
    )
    .map_err(|e| format!("tracing initialization error: {e}"))
}

/// Load and parse the configured UCI schema.
fn load_schema(config: &config::Config) -> Result<Arc<schema::model::Schema>, String> {
    let schema = schema::parser::parse(&config.server.schema_path)
        .map_err(|e| format!("schema parse error: {e}"))?;
    validate_schema_version(&config.server.schema_version, &schema)
        .map_err(|e| format!("schema configuration error: {e}"))?;
    info!(
        schema_version = schema.version.as_deref().unwrap_or(""),
        elements = schema.global_elements.len(),
        types = schema.complex_types.len(),
        facets = schema.facet_count(),
        "schema loaded"
    );
    Ok(Arc::new(schema))
}

/// Verify the configured schema version matches the loaded XSD declaration.
fn validate_schema_version(
    configured_schema_version: &str,
    schema: &schema::model::Schema,
) -> Result<(), String> {
    let loaded_schema_version = schema
        .version
        .as_deref()
        .ok_or_else(|| "loaded schema does not declare a version attribute".to_owned())?;

    if loaded_schema_version == configured_schema_version {
        Ok(())
    } else {
        Err(format!(
            "configured schema_version '{configured_schema_version}' does not match loaded schema version '{loaded_schema_version}'"
        ))
    }
}

/// Install the Prometheus recorder when the admin listener is configured.
fn install_metrics(config: &config::Config) -> Result<Option<PrometheusHandle>, String> {
    if config.server.metrics_bind_addr.is_none() {
        return Ok(None);
    }

    let process_collector = metrics_process::Collector::default();
    process_collector.describe();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            process_collector.collect();
        }
    });

    PrometheusBuilder::new()
        .install_recorder()
        .map(|handle| {
            telemetry::describe_metrics();
            Some(handle)
        })
        .map_err(|e| format!("metrics recorder initialization error: {e}"))
}

/// Build the shared admin endpoint state.
fn make_admin_state(
    config: &config::Config,
    active_connections: Arc<AtomicU64>,
    ready: Arc<AtomicBool>,
) -> Arc<admin::AdminState> {
    Arc::new(admin::AdminState {
        start_time: Instant::now(),
        schema_version: config.server.schema_version.clone(),
        active_connections,
        ready,
    })
}

/// Start the admin listener when both its bind address and metrics handle exist.
fn spawn_admin_listener(
    config: &config::Config,
    admin_state: Arc<admin::AdminState>,
    metrics_handle: Option<PrometheusHandle>,
    shutdown: &CancellationToken,
) -> Option<tokio::task::JoinHandle<Result<(), admin::AdminError>>> {
    let bind_addr = config.server.metrics_bind_addr?;
    let handle = metrics_handle?;
    let admin_shutdown = shutdown.clone();

    Some(tokio::spawn(admin::run(
        bind_addr,
        admin_state,
        handle,
        admin_shutdown,
    )))
}

fn flatten_server_result(
    result: Result<Result<(), server::ServerError>, tokio::task::JoinError>,
) -> Result<(), String> {
    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(format!("server error: {e}")),
        Err(e) => Err(format!("server task failed: {e}")),
    }
}

fn flatten_admin_result(
    result: Result<Result<(), admin::AdminError>, tokio::task::JoinError>,
) -> Result<(), String> {
    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(format!("admin listener error: {e}")),
        Err(e) => Err(format!("admin listener task failed: {e}")),
    }
}

/// Execute a fast, standalone HTTP GET against the local `/health` endpoint.
///
/// This avoids spinning up the Tokio runtime or initializing complex
/// dependencies just to check container health. Returns `Ok` if a 200 is
/// received, otherwise returns an error description.
fn run_health_check(port: u16) -> Result<(), String> {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::time::Duration;

    let mut stream = TcpStream::connect(format!("127.0.0.1:{port}"))
        .map_err(|e| format!("connection failed: {e}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| format!("failed to set read timeout: {e}"))?;
    stream
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| format!("failed to set write timeout: {e}"))?;

    stream
        .write_all(b"GET /health HTTP/1.0\r\nHost: localhost\r\n\r\n")
        .map_err(|e| format!("write failed: {e}"))?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|e| format!("read failed: {e}"))?;

    if response.starts_with("HTTP/1.0 200") || response.starts_with("HTTP/1.1 200") {
        Ok(())
    } else {
        Err(format!(
            "unhealthy response: {}",
            response.lines().next().unwrap_or("")
        ))
    }
}

async fn run_until_shutdown(
    config: Arc<config::Config>,
    schema: Arc<schema::model::Schema>,
    router: Arc<router::Router>,
    shutdown: CancellationToken,
    active_connections: Arc<AtomicU64>,
    ready: Arc<AtomicBool>,
    admin_task: Option<tokio::task::JoinHandle<Result<(), admin::AdminError>>>,
) -> Result<(), String> {
    let cleanup_deadline = std::time::Duration::from_millis(config.server.asb_cleanup_timeout_ms);
    let mut server_task = tokio::spawn(server::run(
        config,
        schema,
        router,
        shutdown.clone(),
        active_connections,
        ready,
    ));

    let Some(mut admin_task) = admin_task else {
        return flatten_server_result(server_task.await);
    };

    tokio::select! {
        server_result = &mut server_task => {
            let server_outcome = flatten_server_result(server_result);
            shutdown.cancel();
            let admin_outcome = if let Ok(result) = tokio::time::timeout(cleanup_deadline, &mut admin_task).await {
                flatten_admin_result(result)
            } else {
                admin_task.abort();
                let _ = tokio::time::timeout(cleanup_deadline, admin_task).await;
                Err("admin listener cleanup deadline".to_owned())
            };
            server_outcome.and(admin_outcome)
        }
        admin_result = &mut admin_task => {
            let mut admin_outcome = flatten_admin_result(admin_result);
            if admin_outcome.is_ok() && !shutdown.is_cancelled() {
                admin_outcome = Err("admin listener exited unexpectedly".to_owned());
            }
            shutdown.cancel();
            // The server owns bounded connection and adapter cleanup. Await it
            // even when the admin listener was the first task to fail.
            let server_outcome = flatten_server_result(server_task.await);
            admin_outcome.and(server_outcome)
        }
    }
}

/// Wait for process shutdown signals and cancel the shared token.
async fn shutdown_signal(token: CancellationToken) {
    #[cfg(unix)]
    {
        let mut sigterm =
            match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
                Ok(stream) => stream,
                Err(e) => {
                    error!("failed to register SIGTERM handler: {e}");
                    token.cancel();
                    return;
                }
            };

        let sigint = tokio::signal::ctrl_c();
        let sigterm_recv = sigterm.recv();
        pin_mut!(sigint, sigterm_recv);

        match select(sigint, sigterm_recv).await {
            Either::Left((_ok, _remaining)) => {
                info!("received SIGINT");
                token.cancel();
            }
            Either::Right((_term, _remaining)) => {
                info!("received SIGTERM");
                token.cancel();
            }
        }
    }

    #[cfg(not(unix))]
    {
        if let Err(e) = tokio::signal::ctrl_c().await {
            error!("failed to register SIGINT handler: {e}");
        } else {
            info!("received SIGINT");
        }
        token.cancel();
    }
}

fn main() -> std::process::ExitCode {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("create native server runtime");
    let mut cleanup_deadline = std::time::Duration::from_secs(5);
    let outcome = runtime.block_on(async_main(&mut cleanup_deadline));
    // Started blocking work is not cancellable. Never wait for it indefinitely
    // during process shutdown; the process owner bounds any remaining lifetime.
    runtime.shutdown_timeout(cleanup_deadline);
    match outcome {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            std::process::ExitCode::FAILURE
        }
    }
}

async fn async_main(cleanup_deadline: &mut std::time::Duration) -> Result<(), String> {
    let cli_action = match parse_cli_args(std::env::args(), |k| std::env::var(k)) {
        Ok(action) => action,
        Err(message) => {
            print_usage();
            return Err(message);
        }
    };

    let CliAction::Run { config_path } = cli_action else {
        match cli_action {
            CliAction::HealthCheck { port } => {
                return run_health_check(port)
                    .map_err(|message| format!("health check failed: {message}"));
            }
            CliAction::Help => print_usage(),
            CliAction::Version => println!("{}", env!("CARGO_PKG_VERSION")),
            CliAction::Run { .. } => unreachable!("run action handled above"),
        }
        return Ok(());
    };

    let bootstrap_guard = telemetry::init_bootstrap();
    let config = load_config(&config_path)?;
    *cleanup_deadline = std::time::Duration::from_millis(config.server.asb_cleanup_timeout_ms);
    init_configured_tracing(&config)?;
    drop(bootstrap_guard);

    info!(
        system_uuid = %config.server.system_uuid,
        subsystem_uuid = %config.server.subsystem_uuid,
        "resolved OMS identity UUIDs"
    );

    let schema = load_schema(&config)?;
    let active_connections = Arc::new(AtomicU64::new(0));
    let ready = Arc::new(AtomicBool::new(false));
    let admin_state =
        make_admin_state(&config, Arc::clone(&active_connections), Arc::clone(&ready));
    let metrics_handle = install_metrics(&config)?;
    let router = Arc::new(router::Router::new());
    let shutdown = CancellationToken::new();

    tokio::spawn(shutdown_signal(shutdown.clone()));
    let admin_task = spawn_admin_listener(&config, admin_state, metrics_handle, &shutdown);

    run_until_shutdown(
        config,
        schema,
        router,
        shutdown,
        active_connections,
        ready,
        admin_task,
    )
    .await?;

    info!("shutdown complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{CliAction, parse_cli_args, validate_schema_version};
    use crate::schema::model::Schema;

    #[test]
    fn parse_help_flag() {
        let action = parse_cli_args(["sleet".to_owned(), "--help".to_owned()], |k| {
            std::env::var(k)
        })
        .expect("help flag should parse");
        assert_eq!(action, CliAction::Help);
    }

    #[test]
    fn parse_version_flag() {
        let action = parse_cli_args(["sleet".to_owned(), "--version".to_owned()], |k| {
            std::env::var(k)
        })
        .expect("version flag should parse");
        assert_eq!(action, CliAction::Version);
    }

    #[test]
    fn parse_default_config_path() {
        let action = parse_cli_args(["sleet".to_owned()], |k| std::env::var(k))
            .expect("empty args should parse");
        assert_eq!(
            action,
            CliAction::Run {
                config_path: "config/server.toml".to_owned(),
            }
        );
    }

    #[test]
    fn parse_custom_config_path() {
        let action = parse_cli_args(["sleet".to_owned(), "./custom.toml".to_owned()], |k| {
            std::env::var(k)
        })
        .expect("custom path should parse");
        assert_eq!(
            action,
            CliAction::Run {
                config_path: "./custom.toml".to_owned(),
            }
        );
    }

    #[test]
    fn parse_health_check_with_env() {
        let action = parse_cli_args(["sleet".to_owned(), "--health-check".to_owned()], |key| {
            if key == "SLEET_METRICS_PORT" {
                Ok("21312".to_owned())
            } else {
                Err(std::env::VarError::NotPresent)
            }
        })
        .expect("--health-check should parse with env var");
        assert_eq!(action, CliAction::HealthCheck { port: 21312 });
    }

    #[test]
    fn parse_health_check_missing_env() {
        let error = parse_cli_args(["sleet".to_owned(), "--health-check".to_owned()], |_| {
            Err(std::env::VarError::NotPresent)
        })
        .expect_err("--health-check without env var should fail");
        assert!(error.contains("SLEET_METRICS_PORT environment variable is required"));
    }

    #[test]
    fn parse_health_check_invalid_env() {
        let error = parse_cli_args(["sleet".to_owned(), "--health-check".to_owned()], |key| {
            if key == "SLEET_METRICS_PORT" {
                Ok("foo".to_owned())
            } else {
                Err(std::env::VarError::NotPresent)
            }
        })
        .expect_err("--health-check with invalid env var should fail");
        assert!(error.contains("invalid port in SLEET_METRICS_PORT: foo"));
    }

    #[test]
    fn parse_health_check_unexpected_argument() {
        let error = parse_cli_args(
            [
                "sleet".to_owned(),
                "--health-check".to_owned(),
                "config/server.toml".to_owned(),
            ],
            |key| {
                if key == "SLEET_METRICS_PORT" {
                    Ok("21312".to_owned())
                } else {
                    Err(std::env::VarError::NotPresent)
                }
            },
        )
        .expect_err("--health-check with argument should fail");
        assert!(error.contains("unexpected argument: config/server.toml"));
    }

    #[test]
    fn parse_unknown_option_fails() {
        let error = parse_cli_args(["sleet".to_owned(), "--bogus".to_owned()], |k| {
            std::env::var(k)
        })
        .expect_err("unknown options should fail");
        assert!(error.contains("unknown option"));
    }

    #[test]
    fn schema_version_validation_accepts_matching_loaded_version() {
        let mut schema = Schema::new();
        schema.version = Some("002.5.0".to_owned());

        validate_schema_version("002.5.0", &schema)
            .expect("matching schema version should be accepted");
    }

    #[test]
    fn schema_version_validation_rejects_mismatch() {
        let mut schema = Schema::new();
        schema.version = Some("003.0.0".to_owned());

        let error = validate_schema_version("002.5.0", &schema)
            .expect_err("mismatched schema version should be rejected");

        assert!(error.contains("configured schema_version '002.5.0'"));
        assert!(error.contains("loaded schema version '003.0.0'"));
    }

    #[test]
    fn schema_version_validation_rejects_missing_loaded_version() {
        let schema = Schema::new();

        let error = validate_schema_version("002.5.0", &schema)
            .expect_err("missing schema version should be rejected");

        assert!(error.contains("does not declare a version attribute"));
    }

    #[test]
    fn run_health_check_connection_refused() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener); // Ensure the port is closed

        let err = super::run_health_check(port).expect_err("connection should fail");
        assert!(err.contains("connection failed"));
    }

    #[test]
    fn run_health_check_read_timeout() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        std::thread::spawn(move || {
            if let Ok((_stream, _)) = listener.accept() {
                // Keep the stream alive but don't send any data to cause a read timeout
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
        });

        let err = super::run_health_check(port).expect_err("read should timeout");
        assert!(err.contains("read failed"));
    }
}
