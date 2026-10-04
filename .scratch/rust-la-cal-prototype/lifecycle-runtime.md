# Reusable Service lifecycle runtime

Status: historical; reusable runtime dropped from the current effort

Decision: [Re-evaluate self-RESTART versus an orchestrator and an orchestrated Service](issues/09-demo-participant-topology.md#answer)

Superseded by [Choose a client-focused relayed STOP demonstration](issues/11-client-focused-stop-demo.md#answer). Applications own lifecycle behavior and client connections directly. The design below is retained for historical context, not as an implementation requirement.

## Purpose

Put reusable Service lifecycle behavior in `service-lifecycle`. The runtime owns the state machine and reporting; the application owns the work that makes each state true. Both `orchestrator` and `example-service` use the runtime and `la-cal-client`.

The user confirmed this approach, the START-only demo flow, and the crate names below. This document records the design, not implemented Rust interfaces.

## Crates and responsibilities

| Crate | Responsibility |
| --- | --- |
| `la-cal` | Native OWP server, OMS JSON validation, and Zenoh/Protobuf ASB adapter. |
| `la-cal-client` | Reusable OWP negotiation, connection, publication, subscription, acknowledgment, and message reception. |
| `service-lifecycle` | Lifecycle state, transition validation, initialization sequencing, health signals, periodic/transition ServiceStatus, supervisor/connection ownership, and lifecycle command coordination/correlation/reply helpers. |
| `orchestrator` | Native Service application: process-management initialization, START action, one-child ownership, repeated-START rejection, and cleanup. |
| `example-service` | Service application: example workload initialization and application behavior. |
| `demo-driver` | Initial START Commander and observer; launch coordination, evidence collection, demo verdict, and teardown initiation. |

Conceptual dependency direction:

```text
orchestrator ────┐
                ├── service-lifecycle ── la-cal-client
example-service ┘

demo-driver ─────────────────────────── la-cal-client
```

Application hooks make the runtime reusable without placing process management or workload logic inside it. The application can also use the client for its own messaging. The user selected one OWP connection per Service, shared through client handles. The Service lifecycle runtime explicitly owns that connection and its supervisor, supplies shared client/health handles to the application, and coordinates application cleanup and connection shutdown. The demo driver owns its client connection directly. Exact public types and cleanup mechanics belong to [Choose the Rust component interfaces and dependencies](issues/06-rust-component-boundaries.md).

`service-lifecycle` and `la-cal-client` are libraries running inside each Service application's process. The supervisor, connection I/O, and application futures are in-process async work, not separate lifecycle/application processes. The orchestrator and the child example-service are separate Service processes as already selected.

The user selected a typed client `send(message)` API: callers provide Rust message data structures, and the client encodes OMS JSON and performs OWP publication. Reuse a project-owned adaptation of SLEET's generator for ServiceLifecycleCommand, ServiceLifecycleCommandStatus, and ServiceStatus under UCI 002.5.0, including required type dependencies and root wrappers. Exact generated-type placement, metadata, signatures, and selected-subset generator verification remain part of the interface decision. This is Rust/OMS-JSON binding generation, not schema-derived UCI Protobuf generation or client runtime schema validation.

The user selected a project-owned adaptation of SLEET's OWP server frontend for `la-cal`, retaining local subscription/group routing with the publication/receive seams and required repairs identified in [SLEET server reuse with the selected Zenoh/Protobuf ASB](research/sleet-server-reuse-feasibility.md). Record actual adapted-source and generator modifications in the root [NOTICE](../../NOTICE), preserving upstream license/attribution and modified-file notices.

Generated message bindings live in a versioned messages module within `la-cal-client` (illustrative path `messages::uci_002_5`) and are shared by `service-lifecycle`, applications, and the driver. Start with SLEET's direct OMS JSON validator on the server/adapter path, repairing and independently verifying the selected message dependency closure; retain the validation seam for replacement if that approach proves infeasible. This choice does not establish complete validation coverage.

## Runtime ownership and event loop

Use one asynchronous supervisor per Service as the single owner of lifecycle state. Inputs are application health signals, incoming lifecycle commands, application action completions, and reporting timer events.

The supervisor owns:

- Current state and the previous state needed for RESUME from PAUSED.
- Service identity, running lifetime/TimeUp, and state reasons.
- The configured periodic-reporting schedule.
- Target and supported-action checks and command coordination/correlation; the application handler owns command semantics and sends protocol replies through the shared typed client.
- At most one pending application lifecycle action.

State changes are serialized through this supervisor. Applications receive a small lifecycle handle for reporting health and observing committed state; they do not mutate state independently.

The event loop must keep polling reporting and communications while an application hook is pending. An implementation can select over the pending action future, inbound events, and the reporting timer. Simply awaiting a long-running hook in the receive loop must not stop required reporting or control reception. Arbitration of competing commands and health events during a pending action remains an interface-level decision.

The user selected Tokio with tokio-tungstenite for the native baseline, with application hooks expressed as ordinary Rust futures. Exact versions, features, channel/timer/spawning mechanics, and public Rust types remain to be settled. WASIp3-driven changes to these choices belong to the [deferred portability spike](../wasip3-service-portability/issues/01-wasip3-service-runtime-spike.md), not the first native demo's acceptance requirements.

## Application-facing interface intent

The selected application hooks are **init, execute, and shutdown only**, all async with unit return:

```rust
async fn init(&self, client: ClientHandle);
async fn execute(&self, message: Message, client: ClientHandle);
async fn shutdown(&self, client: ClientHandle);
```

`Message` and `ClientHandle` are illustrative public type names; exact generated-message representation remains to be settled. The runtime-control handle is retained on the application. `execute` handles lifecycle commands and other messages, including downstream messaging and explicit protocol responses. Returning means that invocation finished; it does not cause an automatic command-status response or prove an achieved state transition. `init` completes when initialization is ready; if initialization cannot succeed, the application requests shutdown, and the supervisor must honor that request rather than automatically report NORMAL.

The following entries describe capabilities, not final Rust signatures:

| Interface operation | Meaning |
| --- | --- |
| `report_health(Degraded { reason })` | Submit an application condition that can justify a permitted transition to DEGRADED. |
| `report_health(Healthy)` | Submit recovery to full capability, allowing DEGRADED → NORMAL. |
| `observe_state()` | Read or subscribe to committed lifecycle state; a notification of state is not an action-completion acknowledgment. |
| Runtime-control `request_shutdown()` | Non-blocking, idempotent request for coordinated local process teardown; the application retains this shared handle on itself. |
| Application `init` hook | Initialize application-specific configuration/workload after the communications path is ready. |
| Application `execute` hook | Handle an incoming full typed message using the shared client; perform supported lifecycle/application actions and send appropriate protocol messages explicitly. |
| Application `shutdown` hook | Clean up owned work/resources during coordinated local process teardown, before the runtime closes its client connection. |

STOP, PAUSE, RESUME and other lifecycle commands enter `execute`; they do not select dedicated application hooks. `shutdown` is not a STOP-message callback. A STOP aimed at a managed child is handled by the orchestrator's `execute` while the orchestrator remains running. If a self-STOP is implemented by exiting, it can request coordinated local teardown, which subsequently invokes `shutdown`. Pinned UCI STOP specifies INOPERABLE; process exit is a Service-contract choice.

The application coordinates achieved self-state transitions with the supervisor explicitly; the exact acknowledgment interface remains open. Application control of another affected Service must not automatically change the Effecting Service's own state.

A health signal describes operational condition rather than an unrestricted `set_state`. The runtime validates state-specific transitions. Healthy/Degraded signals cannot silently resume a PAUSED Service.

## Initialization and reporting

1. Begin in INITIALIZING.
2. Establish OWP through INIT/INFO and install required subscriptions. Connection readiness is distinct from initialization completion.
3. Publish the initial INITIALIZING report once communications permit, before completing application initialization. Begin periodic reporting when publication is possible.
4. Run the application `init` hook.
5. On successful initialization for the first demo, transition automatically to NORMAL and publish its report.

Each Service reports periodically and on every state transition. Use the already-selected environment-configurable interval, default one second, supporting 1–600 seconds in whole-second increments. The environment variable name and invalid-value behavior remain to be settled with configuration.

Each report describes that Service's identity, state, and TimeUp. ServiceStatus is not correlated by CommandID. Immediate transition reporting is a project behavior in addition to periodic reporting.

## Health-driven transitions

For example, an application that detects loss of a primary sensor reports a Degraded condition with its reason. In NORMAL, the supervisor commits NORMAL → DEGRADED and publishes ServiceStatus. The reporting timer continues using DEGRADED. When the application reports restored full capability, DEGRADED → NORMAL is committed and reported.

The application defines what counts as degraded or recovered in its Service contract; the runtime does not infer application health solely from OWP or ASB connectivity. Unsupported or disallowed transitions must not be committed.

## PAUSE and RESUME

For a Service that supports PAUSE of its own workload:

1. Receive a lifecycle PAUSE command designated for this processor.
2. Validate its support and current state; the relevant commanded transitions are NORMAL/DEGRADED → PAUSED.
3. Dispatch the full command to application `execute` with its shared client.
4. The application stops admitting operational work, suspends or drains its workers according to its contract, and explicitly confirms achieved suspension to the supervisor; unit return alone is not that confirmation.
5. Remember the pre-pause state, commit PAUSED, and publish its state report.
6. The application handler publishes one truthful terminal ACCEPTED/SUCCESS command result, correlated to the original command, after the transition is confirmed.

Reporting PAUSED while the ordinary operational work continues would not make the reported state true. A state-observation channel alone is therefore insufficient; the application must explicitly acknowledge achieved suspension.

While PAUSED, communications, periodic status, and supported control operations needed to leave PAUSED remain available. Operational work is suspended, not the entire supervisor. For RESUME, dispatch through `execute`, coordinate restoration using the remembered state, wait for explicit confirmation of the achieved action, then restore and report that state: NORMAL → PAUSED → NORMAL, or DEGRADED → PAUSED → DEGRADED. The OMS table also permits PAUSED → INITIALIZING when the previous state was INITIALIZING.

The standard does not select callbacks, channels, suspension/draining mechanics, or exact action deadlines. These are project/Service-contract choices. Hook failure must not fabricate PAUSED or SUCCESS; partial-action failure handling remains to be defined if those actions are included in implementation scope.

The first demo exercises START and initial INITIALIZING → NORMAL, not a PAUSE/RESUME or RESTART exchange. This section records the agreed reusable runtime model for supported actions; it does not silently add every lifecycle action to first-demo acceptance.

## START and command outcomes in the first demo

The driver sends NEW/START with the orchestrator as EffectingServiceID and `example-service` as AffectedServiceID. The orchestrator is the processor/Commandee; the child is the affected Service. Target checks must preserve that distinction rather than requiring the processor to equal the affected identity.

The runtime records a local, CommandID-correlated RECEIVED event and dispatches the full START command to the orchestrator's `execute`. The application creates the child and constructs/sends the terminal result through the shared client, using the original CommandID and its own processor identity. The application action succeeds once OS process creation succeeds, not when the child connects or reaches NORMAL:

```text
CommandProcessingState = ACCEPTED
LifecycleCommandStatus = SUCCESS
CommandID              = original START CommandID
ServiceID              = orchestrator
```

RECEIVED is internal for this exchange, not an interim ServiceLifecycleCommandStatus. UCI ACCEPTED is terminal and subsequent responses are ignored. The lifecycle status payload requires SUCCESS or FAIL and has no outcome-pending value.

A START while the managed child is starting or running, or failure to create the child process, produces REJECTED/FAIL. One-child occupancy is application policy, distinct from a duplicate-CommandID cache; no such cache or application retry mechanism is introduced.

Starting the child leaves the orchestrator's own state NORMAL. The child's separate runtime instance connects and initializes independently. The driver verifies the child's startup/status in addition to the command result; child failure after successful spawning fails the demo without revising the completed START response.

## Ownership and cleanup

Use simple process ownership: the orchestrator owns the child it launches and terminates/reaps it during teardown. The driver initiates teardown and cleans up the orchestrator/infrastructure processes it started. This is demo cleanup, not an OMS STOP command exchange. Concrete shutdown/error mechanics remain an implementation-interface detail.

During coordinated local teardown, the runtime invokes application `shutdown` while the client is still available for final messaging. Once mission work has stopped and cleanup completes, the runtime commits INOPERABLE and publishes a final ServiceStatus, waits for publication within a bounded shutdown deadline, then closes the connection. This hook is process-exit cleanup, distinct from handling an incoming STOP in `execute`. `request_shutdown()` signals intent; it does not immediately report INOPERABLE while application work is still active. Final-report publication failure must be visible and must not prevent bounded teardown; publication acknowledgment does not guarantee observer receipt.

The user selected a cloneable runtime-control handle supplied at setup and retained by the application, exposing non-blocking/idempotent `request_shutdown()`. It notifies the supervisor rather than awaiting teardown or closing the client. The graceful sequence stops admitting new `execute` invocations, lets the active invocation finish while I/O/reporting remain available, invokes `shutdown` once, publishes the final INOPERABLE report as above, then tears down the connection/supervisor and returns to the process owner. An invocation must not await that whole sequence from inside `execute` or `shutdown`. The concrete handle type and bounded-drain/timeout behavior remain open.

## Platform direction and deferred work

`la-cal` and `orchestrator` are native. Other Service applications should ideally be Wasm-capable, so `service-lifecycle` and `la-cal-client` must be evaluated for that portability. The user deferred WASIp3 executor/clock/socket adapters, dependency compatibility, host/toolchain pinning, and a Wasm demo requirement to a [separate spike](../wasip3-service-portability/issues/01-wasip3-service-runtime-spike.md). The first-demo plan remains native; no Wasm backend or specific portability abstraction has been selected.

## Source grounding

Use the [selected OMS 2.5 / UCI 002.5.0 profile and pinned artifacts](issues/01-standards-profile.md#answer).

- **Official OMS Standard OMSC-STD-001 Rev M §6.1.1, Table 6.1-1:** state meanings and allowed transition triggers; INITIALIZING → NORMAL is automatic after initialization; NORMAL ↔ DEGRADED follows operational conditions; NORMAL/DEGRADED → PAUSED follows PAUSE; RESUME restores the specified previous state. PAUSED remains connected and able to send messages. Supported states/transitions must follow the table; support for every state/action is not assumed.
- **Same table, INOPERABLE row:** the Service is no longer capable of mission functions; whether a final INOPERABLE ServiceStatus can be sent before stopping operations is defined by the Service Contract. Our graceful-shutdown contract selects a runtime-owned final report after application cleanup and before connection closure.
- **OMS §7.2.1, STD-000319/000320/000321; Service Contract template Rev M §§3.1.2.1–.3:** status reporting and configurable period. **STD-000872/000873:** NORMAL and applicable allowed transitions.
- **Pinned UCI 002.5.0 XSD:** ServiceLifecycleCommandMDT, ServiceLifecycleCommandStatusMDT, CommandStatusBaseType, CommandProcessingStateEnum, ServiceLifecycleCommandStatusEnum, and ServiceStatusMDT. START describes beginning the affected lifecycle in INITIALIZING, likely through a resource manager. Process-spawn success as the command milestone is our contract, not a prescribed OS mechanism.
- **UCI Normalized Interface Specification Rev A §§4.6.1–4.6.2, Figure 4.6-1, CERT UNIS-000105:** terminal command processing and ignoring subsequent responses.
- **LA-CAL Rev B §§5.1.1.1.1.2–.3 and §§5.1.1.1.3.2–.3/.6–.8:** messaging setup, operations, and acknowledgments. OWP `+OK`, command result, Service state, and overall demo verdict remain separate facts.

Supervisor structure, health handles, hooks, and completion timing are project design choices. The existing status-data-request and TLS-support exceptions remain recorded in the lifecycle/operating contracts; this design is not a full Service/Platform compliance claim.
