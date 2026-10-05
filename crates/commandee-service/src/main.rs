//! Application-owned commandee Service.

use std::{process::ExitCode, time::Instant};

use demo_driver::{DemoConfig, shutdown_signal};
use futures_util::StreamExt;
use la_cal_client::{
    Client,
    messages::uci_002_5::{
        CannotComplyEnum, CannotComplyType, CannotComplyTypeValue, CommandProcessingStateEnum,
        CommandStateEnum, ServiceLifecycleCommand, ServiceLifecycleCommandStatusEnum,
        ServiceLifecycleStateEnum, ServiceStateEnum,
    },
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
    // The minimal demo workload is an application-owned periodic unit of work.
    // Leaving this loop stops it synchronously before any final publication.
    let mut workload = tokio::time::interval(config.status_period);
    workload.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let mut units = 0_u64;
    let state = ServiceStateEnum::Normal;
    loop {
        tokio::select! {
            command = commands.next() => {
                let command = command.ok_or("command stream ended unexpectedly")?.map_err(|e| e.to_string())?;
                let data = &command.message.message_data;
                if data.effecting_service_id.as_ref().is_some_and(|id| id.uuid != config.commandee.uuid)
                    || (data.effecting_service_id.is_none() && !data.affected_service_id.iter().any(|id| id.uuid == config.commandee.uuid)) {
                    continue;
                }
                if let Some(description) = rejection(&command, config, &state) {
                    client.send(&config.command_result(
                        &config.commandee, &data.command_id,
                        CommandProcessingStateEnum::Rejected,
                        ServiceLifecycleCommandStatusEnum::Fail,
                        Some(CannotComplyTypeValue::CannotComplyType(CannotComplyType {
                            reason: CannotComplyEnum::InvalidInputParameter,
                            description: Some(description.into()), associated_id: None,
                        })),
                    )?).await.map_err(|e| e.to_string())?;
                    continue;
                }
                drop(workload);
                println!("workload stopped after {units} units; command={}", data.command_id.uuid);
                // Failure here is visible and never retried or replaced with a fabricated result.
                client.send(&config.command_result(
                    &config.commandee, &data.command_id,
                    CommandProcessingStateEnum::Accepted,
                    ServiceLifecycleCommandStatusEnum::Success, None,
                )?).await.map_err(|e| format!("terminal result publication failed: {e}"))?;
                client.send(&config.status(&config.commandee, ServiceStateEnum::Inoperable, started.elapsed())?)
                    .await.map_err(|e| format!("final INOPERABLE publication failed: {e}"))?;
                println!("final result and INOPERABLE publications acknowledged; closing");
                return Ok(());
            }
            _ = reporting.tick() => {
                reporting_client.send(&config.status(&config.commandee, ServiceStateEnum::Normal, started.elapsed())?).await.map_err(|e| e.to_string())?;
            }
            _ = workload.tick() => { units += 1; }
        }
    }
}

fn rejection(
    command: &ServiceLifecycleCommand,
    config: &DemoConfig,
    state: &ServiceStateEnum,
) -> Option<&'static str> {
    let header = &command.message.message_header;
    let data = &command.message.message_data;
    if header.system_id.uuid != config.system_uuid
        || header.mission_id.as_ref().map(|id| &id.uuid) != Some(&config.mission_uuid)
        || data.mission_id.uuid != config.mission_uuid
    {
        Some("System or Mission context does not match this Service")
    } else if data.effecting_service_id.as_ref().map(|id| &id.uuid) != Some(&config.commandee.uuid)
        || data.affected_service_id.len() != 1
        || data.affected_service_id[0].uuid != config.commandee.uuid
    {
        Some("direct STOP requires this Service as sole effecting and affected Service")
    } else if data.command_state != CommandStateEnum::New
        || data.lifecycle_state != ServiceLifecycleStateEnum::Stop
    {
        Some("only NEW/STOP is supported")
    } else if *state != ServiceStateEnum::Normal {
        Some("direct STOP requires NORMAL")
    } else {
        None
    }
}
