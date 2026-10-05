//! Configuration and payload helpers for the native demo (no lifecycle runtime).

use std::{env, time::Duration};

use la_cal_client::{
    ClientConfig,
    messages::uci_002_5::{
        CannotComplyTypeValue, CommandIdType, CommandProcessingStateEnum, CommandStateEnum,
        HeaderType, MessageModeEnum, MissionIdType, ServiceIdType, ServiceLifecycleCommand,
        ServiceLifecycleCommandMdt, ServiceLifecycleCommandMt, ServiceLifecycleCommandStatus,
        ServiceLifecycleCommandStatusEnum, ServiceLifecycleCommandStatusMdt,
        ServiceLifecycleCommandStatusMt, ServiceLifecycleStateEnum, ServiceStateEnum,
        ServiceStatus, ServiceStatusMdt, ServiceStatusMt, SystemIdType,
    },
};
use uuid::Uuid;

/// A demo participant's consistent OWP and payload identity.
#[derive(Clone, Debug)]
pub struct Identity {
    /// Authorized OWP INIT participant name.
    pub participant: String,
    /// UCI Service UUID.
    pub uuid: String,
}

impl Identity {
    fn from_env(prefix: &str, participant: &str, uuid: &str) -> Result<Self, String> {
        let participant = setting(&format!("{prefix}_PARTICIPANT"), participant)?;
        if !owp_types::owp::validate_identifier(&participant) {
            return Err(format!("invalid {prefix}_PARTICIPANT: {participant}"));
        }
        Ok(Self {
            participant,
            uuid: uuid_setting(&format!("{prefix}_UUID"), uuid)?,
        })
    }

    /// Construct the payload `ServiceID` with the resolved participant label.
    pub fn service_id(&self) -> ServiceIdType {
        ServiceIdType {
            uuid: self.uuid.clone(),
            descriptive_label: Some(self.participant.clone()),
            service_version: None,
        }
    }
}

/// Shared demo configuration, not a Service state machine.
#[derive(Clone, Debug)]
pub struct DemoConfig {
    /// Independent driver/Commander identity.
    pub driver: Identity,
    /// Orchestrator identity.
    pub orchestrator: Identity,
    /// Commandee identity.
    pub commandee: Identity,
    /// System UUID used by authorization INFO and payload headers.
    pub system_uuid: String,
    /// Mission UUID used by application headers and targeting.
    pub mission_uuid: String,
    /// Whole-second periodic reporting interval (1–600).
    pub status_period: Duration,
    /// Bound for each required startup, operation, event and cleanup wait.
    pub wait: Duration,
    /// Plain loopback WebSocket endpoint; the driver supplies an isolated port.
    pub endpoint: String,
}

impl DemoConfig {
    /// Resolve and validate environment overrides before launching any processes.
    pub fn from_env() -> Result<Self, String> {
        let driver = Identity::from_env(
            "DEMO_DRIVER",
            "demo-driver",
            "123e4567-e89b-12d3-a456-426614174010",
        )?;
        let orchestrator = Identity::from_env(
            "DEMO_ORCHESTRATOR",
            "orchestrator",
            "123e4567-e89b-12d3-a456-426614174011",
        )?;
        let commandee = Identity::from_env(
            "DEMO_COMMANDEE",
            "commandee-service",
            "123e4567-e89b-12d3-a456-426614174012",
        )?;
        let identities = [&driver, &orchestrator, &commandee];
        for (index, identity) in identities.iter().enumerate() {
            if identities[..index].iter().any(|other| {
                other.participant == identity.participant || other.uuid == identity.uuid
            }) {
                return Err("demo participant names and Service UUIDs must be distinct".into());
            }
        }
        let endpoint = setting("DEMO_ENDPOINT", "ws://127.0.0.1:9000/")?;
        let address = endpoint
            .strip_prefix("ws://")
            .and_then(|s| s.strip_suffix('/'))
            .and_then(|s| s.parse::<std::net::SocketAddr>().ok())
            .filter(|addr| addr.ip().is_loopback() && addr.port() != 0);
        if address.is_none() {
            return Err("DEMO_ENDPOINT must be ws://<loopback IP>:<nonzero port>/".into());
        }
        Ok(Self {
            driver,
            orchestrator,
            commandee,
            system_uuid: uuid_setting("DEMO_SYSTEM_UUID", "123e4567-e89b-12d3-a456-426614174000")?,
            mission_uuid: uuid_setting(
                "DEMO_MISSION_UUID",
                "123e4567-e89b-12d3-a456-426614174020",
            )?,
            status_period: Duration::from_secs(number("DEMO_STATUS_SECONDS", "1", 600)?),
            wait: Duration::from_millis(number("DEMO_WAIT_MS", "5000", 3_600_000)?),
            endpoint,
        })
    }

    /// Use the same bounded client configuration for each participant.
    pub fn client_config(&self, identity: &Identity) -> ClientConfig {
        let mut config = ClientConfig::new(&self.endpoint, &identity.participant);
        config.connection_timeout = self.wait;
        config.operation_timeout = self.wait;
        config.close_timeout = self.wait;
        config
    }

    /// Build a fresh typed Service Status; the application chooses its state.
    pub fn status(
        &self,
        identity: &Identity,
        state: ServiceStateEnum,
        uptime: Duration,
    ) -> Result<ServiceStatus, String> {
        let timestamp = time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(|e| e.to_string())?;
        Ok(ServiceStatus {
            message: ServiceStatusMt {
                // The security type has many optional fields; this constant supplies
                // only the selected unclassified demo markings, not a wire root.
                security_information: serde_json::from_str(
                    r#"{"Classification":"U","OwnerProducer":[{"GovernmentIdentifier":"USA"}]}"#,
                )
                .map_err(|e| e.to_string())?,
                message_header: HeaderType {
                    system_id: SystemIdType {
                        uuid: self.system_uuid.clone(),
                        descriptive_label: None,
                    },
                    timestamp,
                    schema_version: "002.5.0".into(),
                    mode: MessageModeEnum::Live,
                    service_id: Some(identity.service_id()),
                    mission_id: Some(MissionIdType {
                        uuid: self.mission_uuid.clone(),
                        descriptive_label: None,
                        version: None,
                    }),
                },
                message_data: ServiceStatusMdt {
                    service_id: identity.service_id(),
                    time_up: format!("PT{}S", uptime.as_secs()),
                    service_state: state,
                    service_state_reason: vec![],
                    predicted_service_state: vec![],
                    enabled_settings: vec![],
                    supported_settings: vec![],
                },
            },
        })
    }

    /// Construct a NEW/STOP request with a fresh `CommandID` and actual sender.
    pub fn stop_command(
        &self,
        sender: &Identity,
        effecting: &Identity,
        affected: &Identity,
    ) -> Result<ServiceLifecycleCommand, String> {
        let common = self
            .status(sender, ServiceStateEnum::Normal, Duration::ZERO)?
            .message;
        Ok(ServiceLifecycleCommand {
            message: ServiceLifecycleCommandMt {
                security_information: common.security_information,
                message_header: common.message_header.clone(),
                message_data: ServiceLifecycleCommandMdt {
                    command_id: CommandIdType {
                        uuid: Uuid::new_v4().to_string(),
                        descriptive_label: None,
                    },
                    command_state: CommandStateEnum::New,
                    mission_id: common
                        .message_header
                        .mission_id
                        .ok_or("missing MissionID")?,
                    effecting_service_id: Some(effecting.service_id()),
                    affected_service_id: vec![affected.service_id()],
                    lifecycle_state: ServiceLifecycleStateEnum::Stop,
                },
            },
        })
    }

    /// Format one terminal result; classification and matching belong to the application.
    pub fn command_result(
        &self,
        processor: &Identity,
        command_id: &CommandIdType,
        processing: CommandProcessingStateEnum,
        status: ServiceLifecycleCommandStatusEnum,
        reason: Option<CannotComplyTypeValue>,
    ) -> Result<ServiceLifecycleCommandStatus, String> {
        let common = self
            .status(processor, ServiceStateEnum::Normal, Duration::ZERO)?
            .message;
        Ok(ServiceLifecycleCommandStatus {
            message: ServiceLifecycleCommandStatusMt {
                security_information: common.security_information,
                message_header: common.message_header,
                message_data: ServiceLifecycleCommandStatusMdt {
                    command_id: command_id.clone(),
                    command_processing_state: processing,
                    command_processing_state_reason: reason,
                    lifecycle_command_status: status,
                    service_id: processor.service_id(),
                },
            },
        })
    }
}

fn setting(name: &str, default: &str) -> Result<String, String> {
    match env::var(name) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Ok(default.into()),
        Err(error) => Err(format!("{name}: {error}")),
    }
}

fn uuid_setting(name: &str, default: &str) -> Result<String, String> {
    let value = setting(name, default)?;
    let uuid = Uuid::parse_str(&value).map_err(|e| format!("{name}: {e}"))?;
    if uuid.is_nil() || uuid.get_version().is_none() || uuid.get_variant() != uuid::Variant::RFC4122
    {
        return Err(format!("{name}: expected a nonnil RFC 4122 UUID"));
    }
    Ok(uuid.to_string())
}

fn number(name: &str, default: &str, maximum: u64) -> Result<u64, String> {
    setting(name, default)?
        .parse::<u64>()
        .ok()
        .filter(|n| (1..=maximum).contains(n))
        .ok_or_else(|| format!("{name}: expected a whole number from 1 to {maximum}"))
}

/// Await either native termination signal, usable during startup as well as reporting.
pub async fn shutdown_signal() -> Result<(), String> {
    let mut term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .map_err(|e| e.to_string())?;
    tokio::select! {
        result = tokio::signal::ctrl_c() => result.map_err(|e| e.to_string()),
        _ = term.recv() => Ok(()),
    }
}
