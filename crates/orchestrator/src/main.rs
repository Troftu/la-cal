//! Application-owned orchestrator Service.

use std::{process::ExitCode, time::Instant};

use demo_driver::{DemoConfig, shutdown_signal};
use futures_util::StreamExt;
use la_cal_client::{
    Client,
    messages::uci_002_5::{
        ServiceLifecycleCommand, ServiceLifecycleCommandStatus, ServiceStateEnum, ServiceStatus,
    },
};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("orchestrator: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    let config = DemoConfig::from_env()?;
    let client = Client::connect(config.client_config(&config.orchestrator))
        .await
        .map_err(|e| e.to_string())?;
    let outcome = tokio::select! {
        result = serve(&client, &config) => result,
        result = shutdown_signal() => result,
    };
    let cleanup = client.close().await.map_err(|e| e.to_string());
    outcome.and(cleanup)
}

async fn serve(client: &Client, config: &DemoConfig) -> Result<(), String> {
    let started = Instant::now();
    client
        .send(&config.status(
            &config.orchestrator,
            ServiceStateEnum::Initializing,
            started.elapsed(),
        )?)
        .await
        .map_err(|e| e.to_string())?;
    let mut commands = client
        .subscribe::<ServiceLifecycleCommand>()
        .await
        .map_err(|e| e.to_string())?;
    let mut results = client
        .subscribe::<ServiceLifecycleCommandStatus>()
        .await
        .map_err(|e| e.to_string())?;
    let mut statuses = client
        .subscribe::<ServiceStatus>()
        .await
        .map_err(|e| e.to_string())?;
    // Application readiness requires all three subscriptions, not just OWP connect.
    client
        .send(&config.status(
            &config.orchestrator,
            ServiceStateEnum::Normal,
            started.elapsed(),
        )?)
        .await
        .map_err(|e| e.to_string())?;
    let reporting_client = client.clone();
    let mut reporting = tokio::time::interval_at(
        tokio::time::Instant::now() + config.status_period,
        config.status_period,
    );
    reporting.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            command = commands.next() => {
                command.ok_or("command stream ended unexpectedly")?.map_err(|e| e.to_string())?;
                return Err("command relay is not implemented yet (ticket 22)".into());
            }
            result = results.next() => { result.ok_or("result stream ended unexpectedly")?.map_err(|e| e.to_string())?; }
            status = statuses.next() => { status.ok_or("status stream ended unexpectedly")?.map_err(|e| e.to_string())?; }
            _ = reporting.tick() => {
                reporting_client.send(&config.status(&config.orchestrator, ServiceStateEnum::Normal, started.elapsed())?).await.map_err(|e| e.to_string())?;
            }
        }
    }
}
