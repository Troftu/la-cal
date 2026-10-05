//! Deployed test fixture, not a Service implementation or compliance oracle.
//! Supplies controlled public messages/exit behavior for driver milestone checks.

use std::{process::ExitCode, time::Instant};

use demo_driver::{DemoConfig, shutdown_signal};
use futures_util::StreamExt;
use la_cal_client::{
    Client,
    messages::uci_002_5::{
        CommandProcessingStateEnum, ServiceLifecycleCommand, ServiceLifecycleCommandStatusEnum,
        ServiceStateEnum,
    },
};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("stop-test-peer: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    let config = DemoConfig::from_env()?;
    let mode = std::env::var("STOP_TEST_MODE").map_err(|e| e.to_string())?;
    if mode == "stale-orchestrator" {
        return stale_orchestrator(&config).await;
    }
    let client = Client::connect(config.client_config(&config.commandee))
        .await
        .map_err(|e| e.to_string())?;
    let started = Instant::now();
    client
        .send(&config.status(
            &config.commandee,
            ServiceStateEnum::Initializing,
            started.elapsed(),
        )?)
        .await
        .map_err(|e| e.to_string())?;
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
    let mut reporting = tokio::time::interval_at(
        tokio::time::Instant::now() + config.status_period,
        config.status_period,
    );
    let command = loop {
        tokio::select! {
            command = commands.next() => break command.ok_or("fixture command stream ended")?.map_err(|e| e.to_string())?,
            _ = reporting.tick() => client.send(&config.status(&config.commandee, ServiceStateEnum::Normal, started.elapsed())?).await.map_err(|e| e.to_string())?,
        }
    };
    if mode != "missing-state" {
        client
            .send(&config.status(
                &config.commandee,
                ServiceStateEnum::Inoperable,
                started.elapsed(),
            )?)
            .await
            .map_err(|e| e.to_string())?;
    }
    if mode != "missing-result" {
        client
            .send(&config.command_result(
                &config.commandee,
                &command.message.message_data.command_id,
                CommandProcessingStateEnum::Accepted,
                ServiceLifecycleCommandStatusEnum::Success,
                None,
            )?)
            .await
            .map_err(|e| e.to_string())?;
    }
    if mode == "missing-exit" {
        shutdown_signal().await?;
    }
    client.close().await.map_err(|e| e.to_string())?;
    if mode == "failed-exit" {
        return Err("fixture finalization failure after reporting".into());
    }
    Ok(())
}

async fn stale_orchestrator(config: &DemoConfig) -> Result<(), String> {
    let identity = &config.orchestrator;
    let client = Client::connect(config.client_config(identity))
        .await
        .map_err(|e| e.to_string())?;
    client
        .send(&config.status(
            identity,
            ServiceStateEnum::Initializing,
            std::time::Duration::ZERO,
        )?)
        .await
        .map_err(|e| e.to_string())?;
    let mut commands = client
        .subscribe::<ServiceLifecycleCommand>()
        .await
        .map_err(|e| e.to_string())?;
    client
        .send(&config.status(
            identity,
            ServiceStateEnum::Normal,
            std::time::Duration::ZERO,
        )?)
        .await
        .map_err(|e| e.to_string())?;
    let mut reporting = tokio::time::interval_at(
        tokio::time::Instant::now() + config.status_period,
        config.status_period,
    );
    let mut stale = false;
    let shutdown = shutdown_signal();
    tokio::pin!(shutdown);
    loop {
        tokio::select! {
            result = &mut shutdown => { result?; break; }
            command = commands.next() => {
                command.ok_or("fixture command stream ended")?.map_err(|e| e.to_string())?;
                stale = true;
            }
            _ = reporting.tick() => {
                let mut report = config.status(identity, ServiceStateEnum::Normal, std::time::Duration::from_secs(1))?;
                if stale { report.message.message_header.timestamp = "2026-10-05T00:00:00Z".into(); }
                client.send(&report).await.map_err(|e| e.to_string())?;
            }
        }
    }
    client.close().await.map_err(|e| e.to_string())
}
