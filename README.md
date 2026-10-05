# Native Rust LA-CAL prototype

The project-owned server, typed Rust client and readiness applications (tickets 14–20) run on native macOS:
an authorized OWP Client negotiates `owp` / OWP 1.0, subscribes, publishes a
validated UCI 002.5.0 `ServiceStatus`, observes `+OK` and receives the original
JSON in `MSG` through the real Zenoh/Protobuf ASB subscriber. Only ASB ingress
dispatches to the adopted local subscription/group router; PUB has no local echo.

The supported topics are `ServiceLifecycleCommand`,
`ServiceLifecycleCommandStatus` and `ServiceStatus`, each accepting only its
identically named UCI 002.5.0 root. Target Services are identified in payloads.

## Build and run

Run these commands from the repository root. Rustup selects recorded Rust 1.99.0.
Cargo builds the checked-in bytes envelope using upstream **protoc on PATH**
(verified with 36.2); no vendored compiler is used. Ordinary builds do not run
the Python UCI generator. Install native **zenohd 1.10.1** from its
[release assets](https://github.com/eclipse-zenoh/zenoh/releases/tag/1.10.1),
choosing `aarch64-apple-darwin-standalone.zip` for Apple Silicon or
`x86_64-apple-darwin-standalone.zip` for Intel, and put the extracted executable
on PATH. Tests also accept its absolute path through `ZENOHD`.

```sh
cargo build --locked --workspace
# Terminal 1:
zenohd -c config/zenoh-router.json5
# Terminal 2, once the router is running:
target/debug/la-cal config/server.toml
```

The router binds only `127.0.0.1:7447`; external discovery and plugins are disabled.
The server admits clients on `127.0.0.1:9000` only after observing the same nonempty
router-ZID set twice (100 ms apart) and declaring its one startup subscriber.
Stop the server with Ctrl-C, then stop the router with Ctrl-C. The configuration's
schema and participant-directory paths are relative to the working directory.
`config/services.d/baseline.toml` authorizes `baseline-tester` for the
three supported topics. Optional explicit `topic_bindings` must match the fixed
topic/root policy; omitting them does not permit other roots. Unsupported topics
or schema profiles fail configuration loading. No credentials or TLS are configured.
`zenoh_router` must be an explicit loopback TCP endpoint. The configurable
`asb_operation_timeout_ms`, `asb_startup_timeout_ms` and `asb_cleanup_timeout_ms`
default to 5000, with 1–60000 allowed. Each startup/operation/cleanup wait is bounded
separately, including synchronous Zenoh work and resource destruction off async
threads. A publication deadline reports an uncertain outcome because started
blocking work cannot be canceled.

Router-ZID observation continues after startup, every 100 ms by default
(`asb_router_poll_interval_ms`, 1–60000 allowed). Loss or replacement of the startup
router set permanently fails that run, stops admission, closes clients and produces
an unsuccessful server exit. Restoring the router endpoint does not resume the run.
Outages entirely between observations can be missed; polling does not bound the
underlying transport's loss-detection latency.

Each server connection has a 64-message outbound queue with nonblocking enqueue.
Overflow is reported and closes that connection, disposing all its registrations
while healthy recipients continue. Unsubscribe invalidates queued delivery for its
registration; reusing a subscription ID cannot inherit an old queued message.
Connection cleanup also runs on task abort and retains the shared ASB listener.

For a repeatable native exchange with automatic router/server launch and cleanup:

```sh
cargo test --locked -p la-cal --test native_baseline -- --nocapture
```

The native tests launch the actual `la-cal` binary and native `zenohd` on isolated
ports. The exchange checks the negotiated WebSocket subprotocol and INFO identities, subscribes
to all three roots, rejects cross-topic publications without closing the connection,
awaits SUB, PUB and UNSUB acknowledgments, and independently checks original-byte
`ServiceStatus` MSG delivery and an independently decoded envelope observed by
another Zenoh session. Twelve invalid ASB samples are visibly discarded, each
followed by a valid original-byte delivery. Unknown envelope fields do not bypass
payload checks; UNSUB, resubscription and client closure retain the shared listener.
A missing valid router fails startup before OWP admission. Separate deployed checks
verify slow-socket overflow isolation and terminal router loss despite endpoint
restoration. A successful exchange verifies graceful server/session cleanup.
Readiness and exchanges are bounded; owned processes are killed and
reaped on success or failure. The test uses
[`fixtures/service-status.json`](fixtures/service-status.json), constructed
from the pinned schema rather than a generated serialization expectation.

## Service readiness demo

From the repository root, one driver owns native router/server and both Services:

```sh
cargo build --locked --workspace
target/debug/demo-driver --readiness
# Repeatable deployed readiness and cleanup checks:
cargo test --locked -p demo-driver --test readiness
```

Put `zenohd` on PATH or set `ZENOHD` to its absolute path. The driver selects
isolated loopback ports and generates temporary router/server authorization from
the resolved identities. It installs independent typed status/result observations
**before** launching either application, then requires INITIALIZING, NORMAL and
a later periodic NORMAL report from each distinct Service. Every launch, report
wait and cleanup is bounded; failures print child diagnostics. Cleanup requests
SIGTERM, escalates to SIGKILL if necessary and waits for every owned child.
Ctrl-C also takes the cleanup path. Launch only from the repository root so the
pinned schema is found. Applications live beside the driver binary.

Each Service owns its client, required subscriptions and reporting loop directly.
Configuration/payload helpers are shared, not a lifecycle runtime. The readiness
driver sends no commands. The direct STOP slice below executes the Commandee's
shutdown; the Orchestrator relay and final acceptance remain tickets 22–24.

| Environment override | Default / range |
| --- | --- |
| `DEMO_DRIVER_PARTICIPANT`, `DEMO_ORCHESTRATOR_PARTICIPANT`, `DEMO_COMMANDEE_PARTICIPANT` | `demo-driver`, `orchestrator`, `commandee-service` |
| `DEMO_DRIVER_UUID`, `DEMO_ORCHESTRATOR_UUID`, `DEMO_COMMANDEE_UUID` | `123e4567-e89b-12d3-a456-426614174010`, `…4011`, `…4012` (full defaults in `DemoConfig`) |
| `DEMO_SYSTEM_UUID` | `123e4567-e89b-12d3-a456-426614174000` |
| `DEMO_MISSION_UUID` | `123e4567-e89b-12d3-a456-426614174020` |
| `DEMO_STATUS_SECONDS` | `1`; whole seconds 1–600 |
| `DEMO_WAIT_MS` | `5000`; 1–3600000 per required wait (server ASB waits cap at 60000) |
| `DEMO_ENDPOINT` | Standalone Services: `ws://127.0.0.1:9000/`; driver supplies its isolated loopback endpoint |
| `DEMO_LA_CAL_BIN`, `DEMO_ORCHESTRATOR_BIN`, `DEMO_COMMANDEE_SERVICE_BIN` | Sibling application binaries; optional executable path overrides |

Participant names and Service UUIDs must each be distinct. Invalid names, UUIDs,
endpoints and numeric settings fail before driver launch. Increase the event wait
above the reporting period when using longer periods, for example:

```sh
DEMO_STATUS_SECONDS=2 DEMO_WAIT_MS=6000 target/debug/demo-driver --readiness
```

See [`docs/service-readiness-evidence.md`](docs/service-readiness-evidence.md)
for the native run, configuration, source grounding and coverage limits.

## Direct Commandee STOP

```sh
cargo build --locked --workspace
target/debug/demo-driver --direct-stop
# Build the controlled public-message fixtures before the focused checks:
cargo build --locked --workspace --examples
cargo test --locked -p demo-driver --test direct_stop --test commandee_stop
cargo test --locked -p commandee-service --test final_publications
```

As with readiness, put `zenohd` on PATH or set `ZENOHD`. The driver installs
observations before launching both Services and observes their periodic NORMAL
reports. It sends one NEW/STOP directly to the Commandee as both Effecting and
Affected Service, then independently requires its correlated ACCEPTED/SUCCESS,
final INOPERABLE and successful process exit. Neither message arrival order is
assumed. The Orchestrator remains running and must report NORMAL again before
the driver cleans up all remaining owned processes.

The Commandee's minimal workload is an application-owned periodic unit counter;
STOP ends that loop before final messaging. Only explicitly addressed NEW/STOP
in the configured System/Mission with the Commandee as sole affected Service is
eligible. Commands for other processors are ignored; targeted unsupported or
ineligible requests receive one REJECTED/FAIL with a truthful diagnostic and leave
the Service running. Result and state publications are separately awaited before
bounded shared-client closure. Publication or closure failure exits unsuccessfully;
there is no retry, recovery or duplicate suppression.

See [`docs/direct-stop-evidence.md`](docs/direct-stop-evidence.md) for executed
native milestones and test limits. This is not the relayed STOP acceptance.

## Checks and regeneration

```sh
cargo build --locked --workspace --examples
cargo check --locked --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets
cargo test --locked -p la-cal-client --test owp_peer
cargo test --locked -p la-cal-client --doc
python3 -B scripts/generate_uci_types.py --self-test
SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
```

Set `SLEET_SCHEMA_PATH` for the full-schema parser check; the adopted upstream
test otherwise prints a skip and returns successfully. This legacy variable
name is retained for the baseline. Ordinary builds use checked-in bindings.
Explicit regeneration and separate UCI terms are documented in
[`spec/002.5.0/PROVENANCE.md`](spec/002.5.0/PROVENANCE.md).

## Typed Rust client

Depend on `la-cal-client` and use the generated global roots in
`la_cal_client::messages::uci_002_5`. `ServiceLifecycleCommand`,
`ServiceLifecycleCommandStatus` and `ServiceStatus` each wrap the complete body
in a public `message` field. Their generated Serde implementations and `Message`
metadata supply the OMS JSON root, topic and fixed UCI 002.5.0 profile. Body and
dependency types are available for construction but are not publishable roots.

```rust,no_run
use futures_util::StreamExt;
use la_cal_client::{Client, ClientConfig, messages::uci_002_5::ServiceStatus};

async fn exchange(report: ServiceStatus) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::connect(ClientConfig::new(
        "ws://127.0.0.1:9000/", "baseline-tester",
    )).await?;
    let mut reports = client.subscribe::<ServiceStatus>().await?;
    client.clone().send(&report).await?;
    if let Some(item) = reports.next().await {
        println!("{:?}", item?.message.message_data.service_state);
    }
    reports.close().await?;
    client.close().await?;
    Ok(())
}
```

`ClientConfig` supplies the plain loopback endpoint, authorized OWP participant
and connection/operation/close deadlines, each defaulting to five seconds.
Connect awaits `owp` negotiation, verbose INIT acknowledgment and OWP 1.0 INFO.
INFO has no schema member in Rev B: successful INIT accepts the requested
002.5.0 profile. Communications readiness is separate from application NORMAL.

Clones share one connection. Concurrent calls queue PUB/SUB/UNSUB with one
transmitted pending acknowledgment, while MSG and WebSocket controls remain live.
The private reader and writer make independent progress even under socket-write
backpressure; receiving Ping does not block subsequent MSG dispatch on Pong flushing.
Send success means OWP publication completion; server errors retain their exact
code/details. Connection failures distinguish protocol, transport and deadlines,
and record whether transmission was attempted (a sent publication can be uncertain).

Canceling a sent operation's wait leaves background I/O responsible for its response
or operation deadline. Cancellation does not close the connection, undo publication
or retry it; reception continues. A missing acknowledgment terminally fails the
shared connection: queued operations are never sent and late responses cannot be
assigned to them. Pending operations report `transmitted: true` (uncertain PUB
outcome), whereas queued operations and later clone calls report the same cause
with `transmitted: false`. Attributable `-ERR` rejections retain server details and
leave a valid connection usable; malformed controls are terminal protocol failures.

Every subscribe owns its own OWP registration and persistent
`Stream<Item = Result<T, ReceiveError>>`, including repeated subscriptions to the
same type. Typed decoding errors are nonterminal; malformed protocol and unexpected
loss produce a terminal connection error before stream end. Applications own
processing and response matching. Delivery uses simple private unbounded channels;
application polling cannot block acknowledgment handling, and no receive-loss or
configurable overflow contract is established.

Subscription `close(self).await` consumes its stream and awaits only its UNSUB.
Drop stops local reception and requests best-effort cleanup. Ordinary client-clone
drop leaves other owners usable; explicit `close().await` through any clone closes
the shared connection. Close atomically stops new admissions, drains admitted
operations within `close_timeout`, then gives WebSocket closure its own
`close_timeout` (five seconds by default for each wait). Drain expiry still attempts
bounded cleanup; socket backpressure cannot postpone that drain deadline. Close
returns a drain rejection or terminal drain/closure failure, and concurrent or later
close waiters see the same result. Canceling a close waiter does not cancel shutdown.
Live streams receive terminal connection errors, including closure failure, before
ending. Await final publications before close; last-owner/subscription Drop remains
best effort rather than acknowledged teardown.

The native process test also exchanges all three typed roots and a polymorphic
diagnostic fixture, independently observes the corresponding Zenoh envelopes,
and verifies repeated registrations and explicit cleanup. See
[`docs/typed-client-evidence.md`](docs/typed-client-evidence.md) for executed checks.
Ticket 18's cancellation/failure/closure results and independent review are in
[`docs/client-closure-evidence.md`](docs/client-closure-evidence.md).
Ticket 19's server validation, overflow, router-loss and bounded-cleanup checks
and review results are in
[`docs/server-failure-evidence.md`](docs/server-failure-evidence.md).

## External CAL harness

The read-only reference at `standards-and-references/la-cal-harness` is now
available for practical protocol-test reuse. See
[`docs/la-cal-harness-assessment.md`](docs/la-cal-harness-assessment.md) for its
pinned revision, native retargeting route and adaptation/coverage limits.
This is a source assessment only: no external harness preflight or suite has
been run against the project server, and the acceptance scope is unchanged.

## Workspace and milestone

| Package | Purpose at this milestone |
| --- | --- |
| `la-cal` | Adopted OWP frontend, fixed three-root topic policy, authorization, Zenoh/Protobuf ASB, local subscription/groups, direct Rust validator and admin/telemetry |
| `owp-types` | Adopted OWP grammar/types and baseline UCI bindings/tests |
| `baseline-client` | Unmodified upstream-style client used by copied socket tests; temporary adoption support |
| `la-cal-client` | Cloneable async typed client, independent subscription streams and checked-in UCI 002.5.0 roots/dependencies |
| `demo-driver` | Native process ownership, independent typed readiness/direct STOP observations, bounded cleanup and shared configuration/payload helpers |
| `orchestrator` | Application-owned initialization, persistent command/result/status streams and reporting |
| `commandee-service` | Application-owned initialization, persistent command stream, targeting, direct STOP/final messaging and reporting |

Tickets 18–19 complete client cancellation/deadline/drain verification, detected
router-loss failure and the 64-message server queue/close-on-overflow contract.
Ticket 20 supplies readiness/process ownership; ticket 21 supplies direct STOP,
final messaging and independent exit observation. Relay remains ticket 22.
It is source-adoption evidence, not final prototype acceptance or a conformance
claim. Plain `ws` retains the explicit RFC 6455 §10.6 TLS-capability exception.

See [`docs/baseline-evidence.md`](docs/baseline-evidence.md) for executed checks,
versions and known limitations. [`NOTICE`](NOTICE) records the selected SLEET
revision and actual modifications; [`LICENSE`](LICENSE) preserves its Apache-2.0
terms. Reference submodules are read-only.

Ticket 15's topic-policy checks and coverage limits are recorded in
[`docs/topic-routing-evidence.md`](docs/topic-routing-evidence.md).
Ticket 16's executed ASB checks, tool/router versions and coverage limits are in
[`docs/asb-delivery-evidence.md`](docs/asb-delivery-evidence.md).
