//! Approved deployed driver seam: independent result, final state and process exit.
//! Requires workspace binaries and native zenohd (PATH or ZENOHD).

use std::{
    process::{Command, Output},
    time::Duration,
};

#[tokio::test]
async fn direct_stop_observes_terminal_result_final_state_and_successful_exit() {
    let output = run_driver(&[]).await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(output.status.success(), "{stdout}\n{stderr}");
    for milestone in [
        "observations installed before application launch",
        "observed commandee-service: periodic NORMAL",
        "direct STOP publication acknowledged command=",
        "observed direct result: ACCEPTED/SUCCESS",
        "observed commandee-service: INOPERABLE",
        "observed commandee-service: successful process exit",
        "observed orchestrator: still NORMAL after direct STOP",
        "PASS direct STOP",
    ] {
        assert!(stdout.contains(milestone), "missing {milestone}: {stdout}");
    }
    assert_eq!(
        stdout.matches("observed direct result:").count(),
        1,
        "{stdout}"
    );
    assert_reaped(&stdout);
}

async fn run_driver(overrides: &[(&str, &str)]) -> Output {
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_demo-driver"));
    command
        .arg("--direct-stop")
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
        .expect("direct STOP driver deadline")
        .expect("launch driver")
}

fn assert_reaped(stdout: &str) {
    for name in ["router", "la-cal", "orchestrator", "commandee-service"] {
        let line = stdout
            .lines()
            .find(|line| line.starts_with(&format!("reaped {name} pid=")))
            .unwrap_or_else(|| panic!("missing reaping of {name}: {stdout}"));
        let pid = line
            .split("pid=")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap();
        assert!(
            !Command::new("kill")
                .args(["-0", pid])
                .output()
                .unwrap()
                .status
                .success(),
            "{name} PID {pid} remains alive"
        );
    }
}

#[tokio::test]
async fn exit_without_final_state_cannot_substitute_for_inoperable_observation() {
    assert_missing_milestone(
        "missing-state",
        [
            "observed direct result: ACCEPTED/SUCCESS",
            "observed commandee-service: successful process exit",
        ],
        "INOPERABLE",
    )
    .await;
}

#[tokio::test]
async fn final_state_and_exit_cannot_substitute_for_terminal_command_result() {
    assert_missing_milestone(
        "missing-result",
        [
            "observed commandee-service: INOPERABLE",
            "observed commandee-service: successful process exit",
        ],
        "terminal result",
    )
    .await;
}

#[tokio::test]
async fn successful_messages_cannot_substitute_for_process_exit() {
    assert_missing_milestone(
        "missing-exit",
        [
            "observed direct result: ACCEPTED/SUCCESS",
            "observed commandee-service: INOPERABLE",
        ],
        "process exit",
    )
    .await;
}

async fn assert_missing_milestone(mode: &str, observations: [&str; 2], missing: &str) {
    let peer = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/stop-test-peer"
    );
    let output = run_driver(&[
        ("DEMO_COMMANDEE_SERVICE_BIN", peer),
        ("STOP_TEST_MODE", mode),
        ("DEMO_WAIT_MS", "1500"),
    ])
    .await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success(), "{stdout}\n{stderr}");
    for observation in observations {
        assert!(
            stdout.contains(observation),
            "missing {observation}: {stdout}\n{stderr}"
        );
    }
    assert!(
        stderr.contains(&format!("direct STOP deadline waiting for {missing}")),
        "{stdout}\n{stderr}"
    );
    assert!(!stdout.contains("PASS direct STOP"), "{stdout}");
    assert_reaped(&stdout);
}

#[tokio::test]
async fn final_state_before_command_result_is_accepted_without_an_arrival_order_assumption() {
    let peer = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/stop-test-peer"
    );
    let output = run_driver(&[
        ("DEMO_COMMANDEE_SERVICE_BIN", peer),
        ("STOP_TEST_MODE", "state-first"),
    ])
    .await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(output.status.success(), "{stdout}\n{stderr}");
    for milestone in [
        "observed direct result: ACCEPTED/SUCCESS",
        "observed commandee-service: INOPERABLE",
        "observed commandee-service: successful process exit",
        "PASS direct STOP",
    ] {
        assert!(stdout.contains(milestone), "{stdout}");
    }
    assert_reaped(&stdout);
}

#[tokio::test]
async fn unsuccessful_process_exit_fails_even_when_both_success_messages_arrive() {
    let peer = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/stop-test-peer"
    );
    let output = run_driver(&[
        ("DEMO_COMMANDEE_SERVICE_BIN", peer),
        ("STOP_TEST_MODE", "failed-exit"),
    ])
    .await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success(), "{stdout}\n{stderr}");
    assert!(
        stderr.contains("unexpected commandee-service exit"),
        "{stdout}\n{stderr}"
    );
    assert!(!stdout.contains("PASS direct STOP"), "{stdout}");
    assert_reaped(&stdout);
}

#[tokio::test]
async fn stale_orchestrator_reports_cannot_establish_post_stop_availability() {
    let peer = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/debug/examples/stop-test-peer"
    );
    let output = run_driver(&[
        ("DEMO_ORCHESTRATOR_BIN", peer),
        ("STOP_TEST_MODE", "stale-orchestrator"),
        ("DEMO_WAIT_MS", "1500"),
    ])
    .await;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success(), "{stdout}\n{stderr}");
    assert!(
        stderr.contains("Orchestrator post-STOP reporting deadline"),
        "{stdout}\n{stderr}"
    );
    assert!(!stdout.contains("PASS direct STOP"), "{stdout}");
    assert_reaped(&stdout);
}
