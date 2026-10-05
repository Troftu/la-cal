# Ticket 20: native Service readiness and process ownership

Executed 2026-10-05 on native macOS **26.6.2 (25G83), arm64**, continuing the
claimed work from `0f06f6d`. This slice establishes readiness, not STOP acceptance.

## Behavior and sources

- The driver launches/owns native zenohd and `la-cal`, checks listener readiness,
  negotiates its own typed Client and installs persistent Service Status and
  Lifecycle Command Status observations before launching both applications.
  The Orchestrator does not launch or own the Commandee.
- Applications report INITIALIZING before subscription installation completes,
  then NORMAL after their required persistent streams are installed. Commandee
  installs commands; Orchestrator installs commands, results and status. Each
  owns one cloneable Client and directly emits transition plus periodic status.
  No generic lifecycle runtime, handlers or automatic replies were added.
- The driver checks distinct configured Service UUIDs, header sender/System/Mission,
  INITIALIZING then NORMAL and a fresh subsequent NORMAL report for each Service.
  Communications readiness alone never produces a readiness verdict.
- Three predefined participant/Service identities, System and Mission UUIDs have
  validated environment overrides. Generated server authorization, OWP INIT and
  typed headers/ServiceIDs use the same resolved values. Reporting defaults to
  one second and allows whole seconds 1–600; per-wait defaults are five seconds.
- TCP readiness polls the actual condition (20 ms); typed operations and message
  waits use their own deadlines. Child wait events interrupt startup. Required
  errors, missing reports and stream failure fail visibly. SIGINT/SIGTERM enters
  cleanup. The driver requests termination of every owned process, bounds waits,
  escalates to forceful termination if needed and reaps children, including
  failed/partial startup. Failure output includes per-child diagnostics.

Grounding read directly from the official OMS 2.5
`02_OMSC-STD-001_RevM_OMS_Standard_DandD_v2_5.docx`: §6.1.1 / Table 6.1-1
(initialization finishes only once OMS messaging is usable; NORMAL follows
completed initialization); §7.2.1 RQMT STD-000319–000321 and -000872/-000873
(Service reporting, configurable whole-second 1–600 range, NORMAL/transitions).
This is **selected behavior evidence**, not full verification of those
requirements: no delivered Service Contract, DataRequest response or broader
Minimum Message Set is claimed. Payloads use the pinned **UCI 002.5.0** schema
and generated roots; provenance remains in `spec/002.5.0/PROVENANCE.md`.
Profile remains **OMS 2.5 / OMSC-SPC-013 Rev B / OWP 1.0**, not UCI 2.6.
Process ownership, fresh readiness observations, deadlines and the Zenoh/Protobuf
ASB binding are approved project spec/ticket decisions, not standardized UCI ASB
semantics. The server's existing only-ASB-ingress dispatch path is unchanged.

## Versions and launch configuration

- Rust `1.99.0 (b940084d7 2026-09-28)`; Cargo
  `1.99.0 (5f94df478 2026-08-27)`; upstream `libprotoc 36.2` on PATH.
- Native `zenohd v1.10.1`, built with Rust 1.97.1. The existing downloaded release
  is `/private/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd`.
- Existing locked dependencies retained: Zenoh 1.10.1, prost/prost-build 0.14.4,
  Tokio 1.53.2, tokio-tungstenite 0.30.0. Cargo.lock adds only the three workspace
  packages and their already-resolved dependencies. No generator/schema changes.
- The first compile attempt could not find the SDK through system xcode-select.
  Subsequent checks use the installed Xcode through per-command
  `DEVELOPER_DIR=/Applications/Xcode-26.6.0.app/Contents/Developer`; no system
  tool selection was changed.
- Default run: temporary `ws://127.0.0.1:62132/`, router `tcp/127.0.0.1:62131`,
  Mission `123e4567-e89b-12d3-a456-426614174020`, period 1 second, wait 5000 ms.
  Driver/Orchestrator/Commandee UUIDs end `4010`/`4011`/`4012`, respectively.
  Ports vary each run. Router discovery and plugins remain disabled, with all
  listeners loopback-only and plain WebSockets. Configuration is temporary and
  removed after cleanup; launch from the repository root for the pinned schema.

## Executed checks

```sh
DEVELOPER_DIR=/Applications/Xcode-26.6.0.app/Contents/Developer cargo build --locked --workspace
ZENOHD=/private/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd target/debug/demo-driver --readiness
DEVELOPER_DIR=/Applications/Xcode-26.6.0.app/Contents/Developer ZENOHD=/private/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd cargo test --locked -p demo-driver --test readiness -- --nocapture
DEVELOPER_DIR=/Applications/Xcode-26.6.0.app/Contents/Developer cargo check --locked --workspace --all-targets
DEVELOPER_DIR=/Applications/Xcode-26.6.0.app/Contents/Developer cargo clippy --locked --workspace --all-targets
```

All checks above passed. The initial stub test failed with “readiness driver not
implemented” before implementation. Seven deployed process-boundary tests passed:
default transitions/periodic readiness and independently nonexistent child PIDs;
all identity/System/Mission overrides with a two-second report (uptime at least
two seconds); partial launch failure with reaping and successful fresh run;
unexpected early application exit with reaping; invalid configuration before any
launch; and accepted 600-second reporting configuration with a deliberately shorter
1500-ms periodic deadline, producing visible failure and reaping ready Services;
and a nonreporting application which ignores SIGTERM, forcing SIGKILL and reaping.
The tests have an independent 30-second process deadline. No fixed startup sleep
or private test-only interface is used.

The standalone default run observed both INITIALIZING and NORMAL at
`2026-10-05T16:51:48Z`, then both periodic NORMAL reports at `16:51:49Z`.
Owned PIDs 21768 (router), 21770 (server), 21786 (Orchestrator), 21787 (Commandee)
were reaped. Verdict: **PASS readiness**. The partial-startup test also proves a
repeat run succeeds after failure.

Final checks also passed:

```sh
cargo fmt --all -- --check
DEVELOPER_DIR=/Applications/Xcode-26.6.0.app/Contents/Developer ZENOHD=/private/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
python3 -B scripts/generate_uci_types.py --self-test
git diff --check
git diff --cached --check
```

**326 Rust tests passed, zero failed or ignored**: 7 readiness process tests,
236 server checks (including enabled full-schema parsing), 4 native ASB/server
checks, 16 public-client/OWP-peer checks, 6 baseline-client checks, 55 OWP/binding
checks and 2 client doctests. Focused counts overlap this suite, not additional
unique passes. The missing-router startup-error output is an expected negative
test result.
Generator self-tests also passed **15/15**; no generated artifact was changed.

Independent `/code-review` compared staged work with baseline `0f06f6d` before
commit, since no implementation commit yet existed. Standards: 0 hard violations,
1 suggested possible Primitive Obsession in numeric readiness progress. Replaced
those numbers with a small driver-local `ReadinessStage` enum; both reviewers
rechecked. **Final Standards 0, Spec 0 actionable findings.** Reviewers inspected
source without rerunning tests; the full suite ran once after the review repair.

## Coverage limits

- STOP execution/relay, command correlation, final INOPERABLE and independent
  Commandee exit acceptance remain tickets 21–23; readiness sends no commands.
- No external CAL harness execution, Linux execution, complete Service/Platform
  or OWP/WebSocket conformance claim. Plain `ws` retains the RFC 6455 §10.6
  TLS-capability exception. Existing validator/generator defects remain recorded
  in prior evidence; this slice neither repairs nor expands validation coverage.
- Configurable periods 1, 2 and 600 are exercised; waiting 600 seconds for a
  periodic report is not claimed. No broad failure-injection campaign or physical
  router-hop evidence is claimed. Existing real Zenoh/Protobuf adapter (including
  possible SDK-local delivery) carries the observed status messages.
- Temporary free-port selection has a bind race; a collision fails visibly.
  Uncatchable driver termination (SIGKILL/host loss) cannot promise cleanup.
