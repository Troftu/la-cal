//! Application-owned commandee Service.

use std::{process::ExitCode, time::Instant};

use demo_driver::{DemoConfig, shutdown_signal};
use futures_util::StreamExt;
use la_cal_client::{
    Client,
    messages::uci_002_5::{ServiceLifecycleCommand, ServiceStateEnum},
};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("commandee-service: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    let config = DemoConfig::from_env()?;
    let client = Client::connect(config.client_config(&config.commandee))
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
            &config.commandee,
            ServiceStateEnum::Initializing,
            started.elapsed(),
        )?)
        .await
        .map_err(|e| e.to_string())?;
    // Initialization is not complete until the persistent command subscription is installed.
    let mut commands = client
        .subscribe::<ServiceLifecycleCommand>()
        .await
        .map_err(|e| e.to_string())?;
    client
        .send(&config.status(
            &config.commandee,
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
                return Err("command execution is not implemented yet (ticket 21)".into());
            }
            _ = reporting.tick() => {
                reporting_client.send(&config.status(&config.commandee, ServiceStateEnum::Normal, started.elapsed())?).await.map_err(|e| e.to_string())?;
            }
        }
    }
}
