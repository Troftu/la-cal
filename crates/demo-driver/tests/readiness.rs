//! Approved driver/process boundary: readiness reports and independently reaped children.
//! Requires `cargo build --locked --workspace` and native zenohd (PATH or ZENOHD).

use std::{
    process::{Command, Output},
    time::Duration,
};

async fn run_driver(overrides: &[(&str, &str)]) -> Output {
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_demo-driver"));
    command
        .arg("--readiness")
        .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
        .kill_on_drop(true);
    for (key, _) in
        std::env::vars_os().filter(|(key, _)| key.to_string_lossy().starts_with("DEMO_"))
    {
        command.env_remove(key);
    }
    command.envs(overrides.iter().copied());
    tokio::time::timeout(Duration::from_secs(30), command.output())
        .await
        .expect("driver must finish within an independent test deadline")
        .expect("launch native readiness driver")
}

#[tokio::test]
async fn driver_observes_both_startup_transitions_and_periodic_reports_then_reaps_children() {
    let output = run_driver(&[]).await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(output.status.success(), "{stdout}\n{stderr}");
    for service in ["orchestrator", "commandee-service"] {
        for milestone in ["INITIALIZING", "NORMAL", "periodic NORMAL"] {
            assert!(
                stdout.contains(&format!("observed {service}: {milestone}")),
                "{stdout}"
            );
        }
    }
    assert!(stdout.contains("PASS readiness"), "{stdout}");
    assert_reaped(
        &stdout,
        &["router", "la-cal", "orchestrator", "commandee-service"],
    );
}

fn assert_reaped(stdout: &str, names: &[&str]) {
    for name in names {
        let line = stdout
            .lines()
            .find(|line| line.starts_with(&format!("reaped {name} pid=")))
            .unwrap_or_else(|| panic!("missing cleanup for {name}: {stdout}"));
        let pid = line
            .split("pid=")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap();
        let output = Command::new("kill").args(["-0", pid]).output().unwrap();
        assert!(
            !output.status.success(),
            "owned {name} process {pid} still exists"
        );
    }
}

#[tokio::test]
async fn configured_identities_mission_and_two_second_period_reach_readiness() {
    let output = run_driver(&[
        ("DEMO_DRIVER_PARTICIPANT", "commander-test"),
        ("DEMO_ORCHESTRATOR_PARTICIPANT", "relay-test"),
        ("DEMO_COMMANDEE_PARTICIPANT", "worker-test"),
        ("DEMO_DRIVER_UUID", "123e4567-e89b-12d3-a456-426614174110"),
        (
            "DEMO_ORCHESTRATOR_UUID",
            "123e4567-e89b-12d3-a456-426614174111",
        ),
        (
            "DEMO_COMMANDEE_UUID",
            "123e4567-e89b-12d3-a456-426614174112",
        ),
        ("DEMO_SYSTEM_UUID", "123e4567-e89b-12d3-a456-426614174100"),
        ("DEMO_MISSION_UUID", "123e4567-e89b-12d3-a456-426614174120"),
        ("DEMO_STATUS_SECONDS", "2"),
    ])
    .await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(output.status.success(), "{stdout}\n{stderr}");
    for (service, uuid) in [
        ("orchestrator", "123e4567-e89b-12d3-a456-426614174111"),
        ("commandee-service", "123e4567-e89b-12d3-a456-426614174112"),
    ] {
        assert!(
            stdout.contains(&format!("observed {service}: periodic NORMAL uuid={uuid}")),
            "{stdout}"
        );
        let report = stdout
            .lines()
            .find(|line| line.starts_with(&format!("observed {service}: periodic NORMAL")))
            .unwrap();
        // The independent uptime oracle rejects an immediate duplicate NORMAL
        // masquerading as a configured two-second periodic report.
        let uptime: u64 = report
            .split("uptime=PT")
            .nth(1)
            .unwrap()
            .trim_end_matches('S')
            .parse()
            .unwrap();
        assert!(
            uptime >= 2,
            "periodic report arrived before two seconds: {report}"
        );
    }
    assert_reaped(
        &stdout,
        &["router", "la-cal", "orchestrator", "commandee-service"],
    );
}

#[tokio::test]
async fn partial_startup_failure_reaps_launched_children_and_permits_a_fresh_run() {
    let output = run_driver(&[(
        "DEMO_COMMANDEE_SERVICE_BIN",
        "/nonexistent/la-cal-commandee",
    )])
    .await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success(), "{stdout}\n{stderr}");
    assert!(stderr.contains("launch commandee-service"), "{stderr}");
    assert!(!stdout.contains("PASS readiness"), "{stdout}");
    assert_reaped(&stdout, &["router", "la-cal", "orchestrator"]);
    let fresh = run_driver(&[]).await;
    assert!(
        fresh.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&fresh.stdout),
        String::from_utf8_lossy(&fresh.stderr)
    );
}

#[tokio::test]
async fn early_application_exit_is_visible_and_all_children_are_reaped() {
    let output = run_driver(&[("DEMO_COMMANDEE_SERVICE_BIN", "/usr/bin/false")]).await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success(), "{stdout}\n{stderr}");
    assert!(
        stderr.contains("unexpected commandee-service exit"),
        "{stderr}"
    );
    assert!(!stdout.contains("PASS readiness"), "{stdout}");
    assert_reaped(
        &stdout,
        &["router", "la-cal", "orchestrator", "commandee-service"],
    );
}

#[tokio::test]
async fn invalid_configuration_fails_before_any_process_is_launched() {
    for (key, value) in [
        ("DEMO_STATUS_SECONDS", "0"),
        ("DEMO_STATUS_SECONDS", "601"),
        ("DEMO_STATUS_SECONDS", "1.5"),
        ("DEMO_WAIT_MS", "0"),
        ("DEMO_MISSION_UUID", "invalid"),
        ("DEMO_ORCHESTRATOR_PARTICIPANT", "invalid identity"),
        (
            "DEMO_COMMANDEE_UUID",
            "123e4567-e89b-12d3-a456-426614174011",
        ),
    ] {
        let output = run_driver(&[(key, value)]).await;
        assert!(!output.status.success(), "{key}={value}");
        assert!(
            !String::from_utf8_lossy(&output.stdout).contains("launched"),
            "{key}={value}"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("FAIL readiness"),
            "{key}={value}"
        );
    }
}

#[tokio::test]
async fn missing_periodic_report_fails_its_own_deadline_and_reaps_ready_services() {
    let output = run_driver(&[("DEMO_STATUS_SECONDS", "600"), ("DEMO_WAIT_MS", "1500")]).await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success(), "{stdout}\n{stderr}");
    assert!(
        stderr.contains("readiness deadline waiting for") && stderr.contains("periodic NORMAL"),
        "{stdout}\n{stderr}"
    );
    assert!(stdout.contains("observed orchestrator: NORMAL"), "{stdout}");
    assert!(
        stdout.contains("observed commandee-service: NORMAL"),
        "{stdout}"
    );
    assert!(!stdout.contains("PASS readiness"), "{stdout}");
    assert_reaped(
        &stdout,
        &["router", "la-cal", "orchestrator", "commandee-service"],
    );
}

#[tokio::test]
async fn unresponsive_application_is_forcibly_terminated_and_reaped_after_readiness_failure() {
    use std::os::unix::fs::PermissionsExt;

    let directory = tempfile::tempdir().unwrap();
    let executable = directory.path().join("unresponsive-service");
    // Exec retains one owned PID. Ignoring TERM survives exec, so cleanup must
    // escalate rather than pass because a wrapper exited and leaked its child.
    std::fs::write(&executable, "#!/bin/sh\ntrap '' TERM\nexec /bin/sleep 60\n").unwrap();
    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
    let output = run_driver(&[
        ("DEMO_COMMANDEE_SERVICE_BIN", executable.to_str().unwrap()),
        ("DEMO_WAIT_MS", "1500"),
    ])
    .await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success(), "{stdout}\n{stderr}");
    assert!(
        stderr.contains("commandee-service INITIALIZING"),
        "{stdout}\n{stderr}"
    );
    assert!(
        stderr.contains("commandee-service: shutdown deadline; forcing termination"),
        "{stderr}"
    );
    assert_reaped(
        &stdout,
        &["router", "la-cal", "orchestrator", "commandee-service"],
    );
}
