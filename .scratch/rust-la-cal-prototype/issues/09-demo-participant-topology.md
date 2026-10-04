# Re-evaluate self-RESTART versus an orchestrator and an orchestrated Service

Parent: ../map.md
Labels: wayfinder:grilling
Type: grilling
Status: resolved
Blocked by: 01, 03, 05, 07

## Question

**Historical decision:** [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer) supplies the current application/demo contract. START/process-launch, the example-service name, and the runtime below are superseded; the shared-server decision remains.

Should the first demonstration retain a single Service commanding its own RESTART, or start directly with an orchestrator commanding a separate Service? Choose the smallest topology that makes command delivery, lifecycle execution, and correlated status understandable without accumulating self-sender/self-receiver special cases.

The user requested this reassessment because combining sender and receiver roles appears to create too many special cases. This is a decision to revisit the first-draft topology, not confirmation that the two-Service option has already been selected.

Compare the alternatives using concrete command/status sequences and implementation responsibilities:

- Identify the Commander, Commandee/processor, Effecting Service, and Affected Service. For the two-Service option, distinguish an orchestrator that sends commands to a Service which executes them from a manager that itself effects another Service's lifecycle; do not assume those are the same design.
- Trace first startup, readiness, RESTART execution, periodic/transition ServiceStatus, CommandID correlation, and observable completion. Determine who publishes the command, who replies, and who observes each Service's state.
- Compare actual complexity: Service identities and configuration, subscription fan-out and foreign/self-message handling, startup coordination, process ownership, launch order, cleanup, and test attribution. Compare whole-demo complexity rather than minimizing participant count alone.
- Decide whether two OWP clients can share one LA-CAL Server, or whether one LA-CAL Server per Service connected through the router is the simpler or more useful first demonstration. Do not equate two Services with two required CAL Servers.
- Revisit physical-router evidence and SDK-local delivery in light of the selected topology. The previous first/second-phase boundary is provisional for this decision, not a reason to exclude the two-Service alternative.
- Identify exactly which lifecycle, deployment, routing, and acceptance assumptions are superseded. Preserve the selected standards profile, JSON-in-Protobuf encoding, profile-scoped startup wildcard listener, message-type-only topics, and simple failure bounds wherever the selected topology permits.

Start from the existing RESTART demonstration and one-machine loopback deployment. Manager-driven START, process provisioning, additional lifecycle actions, and recovery are not prerequisites merely because a participant is called an orchestrator; identify any necessary scope change explicitly before expanding the demonstration.

## Resolution requirements

Record the chosen participant and CAL topology, one concrete command/status sequence, the reason it is simpler or more representative, and the evidence boundary. Update the topology-dependent parts of the [lifecycle contract](03-lifecycle-round-trip.md#answer), [ASB routing contract](05-zenoh-asb-contract.md#answer), and [operating boundary](07-prototype-operating-boundaries.md#answer), plus the map and downstream ticket context. State what remains in a later phase. Resolve this before choosing component interfaces or final acceptance evidence.

## Answer

Choose two Services, one shared LA-CAL Server, and an external driver. The driver commands START; the native orchestrator acts as Commandee and Effecting Service, launching `example-service` as Affected Service. This is now manager-driven START, not the originally considered self-RESTART or a downstream RESTART exchange. The user confirmed the demo flow and the reusable lifecycle runtime design.

One shared server deliberately exercises multiple OWP clients. Separate applications remove the original self-commanding workload's combined sender/receiver responsibilities. Preserve the existing one-session Zenoh adapter, profile-scoped startup wildcard listener, message-type-only keys, and actual encode/publish/subscribe/decode evidence. SDK-local delivery remains acceptable; physical router traversal and multiple CAL Servers are later work.

### Confirmed sequence

1. Start the loopback Zenoh router and CAL Server; connect the demo driver and install status/command-status observation before launching the orchestrator.
2. The orchestrator establishes communications, installs its command/status-observation subscriptions, reports INITIALIZING, completes initialization, and reports NORMAL. Both Services use periodic and transition reporting, with the existing configurable one-second default.
3. The driver sends NEW/START with fresh CommandID S, EffectingServiceID=orchestrator, and AffectedServiceID=commandee. The already-running orchestrator receives this as the Commandee of the START interaction; its own lifecycle remains NORMAL while managing the child.
4. The orchestrator records a local, CommandID-correlated RECEIVED event and launches the commandee with its configured identity and CAL endpoint. There is no interim lifecycle command-status frame.
5. After successful OS process creation, publish one terminal ACCEPTED/SUCCESS for S, identifying the orchestrator as processor. This does not wait for or establish child OWP readiness or NORMAL.
6. Independently, the commandee establishes communications and required subscriptions, reports INITIALIZING, initializes its workload, and reports NORMAL. The driver receives the START result and verifies both Services' INITIALIZING → NORMAL startup reports, periodic reporting, process launch and actual initialization, processor/affected identities, and actual adapter traversal. Command success does not alone establish overall demo success; do not assume child NORMAL arrives before command status. Required waits retain the existing configurable five-second default.

OWP publication acknowledgments remain distinct from command success. ServiceStatus has no CommandID; attribute it using Service identity and local transition evidence, without assuming a total arrival order across message types. All OMS command/status traffic uses the actual OWP/Zenoh/Protobuf adapter path.

### Application policy and cleanup

- Reject another START while the managed child is starting or running with REJECTED/FAIL; do not create another child. This occupancy policy is not a duplicate-CommandID cache.
- Process creation failure produces REJECTED/FAIL. Child failure after successful creation fails the demo without revising the completed START reply.
- The orchestrator owns its child and terminates/reaps it during teardown. The driver initiates teardown and cleans up the orchestrator/infrastructure it launched. Keep this ordinary process cleanup; no OMS STOP exchange is required.
- No RESTART, workload reset, or second initialization-counter increment belongs to first-demo evidence. Recovery, retries, and replay remain deferred.

### Crates and runtime asset

Use the confirmed names: `orchestrator`, `example-service`, `la-cal`, `la-cal-client`, `service-lifecycle`, and `demo-driver`. Both Service applications use a reusable lifecycle runtime with a serialized asynchronous supervisor, application health signals, initialization/action hooks, and periodic/transition reporting. Application hooks must make reported state true while communications/reporting remain active.

The detailed runtime idea lives in [Reusable Service lifecycle runtime](../lifecycle-runtime.md); exact Rust interfaces/dependencies remain for [Choose the Rust component interfaces and dependencies](06-rust-component-boundaries.md). `la-cal` and `orchestrator` are native. The desired Wasm capability of other Services and the shared crates is deferred to [Spike WASIp3 support for Service lifecycle and LA-CAL clients](../../wasip3-service-portability/issues/01-wasip3-service-runtime-spike.md), outside this native first-demo map.

### Superseded contracts and grounding

The current [lifecycle contract](03-lifecycle-round-trip.md#answer), [ASB contract](05-zenoh-asb-contract.md#answer), and [operating boundary](07-prototype-operating-boundaries.md#answer) incorporate the topology change. Their historical comments retain the original decisions. The selected standards/encoding, routing namespace/listener, simple failure bounds, and TLS/status-request exceptions remain.

OMS 2.5 Rev M §6.1.1/Table 6.1-1 governs initialization; pinned UCI 002.5.0 START guidance and effecting/affected fields support the manager pattern. Process-spawn success is the selected project milestone. UCI Normalized Interface Specification Rev A §4.6.1/Figure 4.6-1 (CERT UNIS-000105) and §4.6.2 make ACCEPTED terminal. ServiceLifecycleCommandStatusMDT requires both CommandProcessingState and SUCCESS/FAIL LifecycleCommandStatus, so RECEIVED is local here and the wire reply combines ACCEPTED/SUCCESS. Further source detail is recorded in the runtime asset and lifecycle contract.

## Comments

- 2026-10-03: User requested re-evaluating the single-Service approach against starting directly with an orchestrator and an orchestrated Service, citing too many special cases when sender and receiver are combined. No topology choice was made in this map update.
- 2026-10-03: Claimed for the resumed Wayfinder conversation about the prototype's participant topology and command/status flow.
- 2026-10-03, Q1: User selected two Services: an orchestrator that commands and a separate commandee Service. Both publish ServiceStatus periodically and on state transitions. User prefers separate orchestrator, commandee, and reusable LA-CAL client crates. The revised demo starts LA-CAL and the orchestrator initially, then sends START to the orchestrator; the user requested a derived full demo flow. This introduces START into scope. Its sender, effecting/affected identities, process-launch responsibility, CAL count, completion milestone, and subsequent RESTART trigger remain to be confirmed. The ticket remains claimed until the complete topology/flow is confirmed.
- 2026-10-03, START source check: Official OMS 2.5 Rev M §6.1.1/Table 6.1-1 makes INITIALIZING → NORMAL automatic after initialization; START is not that transition's trigger. The pinned UCI 002.5.0 schema's ServiceLifecycleStateEnum/START describes starting a lifecycle in INITIALIZING, likely through a resource manager. A proposed START addressed to the orchestrator as EffectingServiceID with the commandee as AffectedServiceID is therefore distinct from starting or restarting the orchestrator itself. Process spawning and waiting for the child's NORMAL before terminal START success are proposed project choices, not yet user-confirmed.
- 2026-10-03, Q2–Q3 follow-up: User confirmed that START makes the orchestrator spawn the commandee executable, and said RESTART is not needed for the first demo. The proposed flow is narrowed to START and initial startup evidence. The external START sender, terminal START success milestone, CAL topology (Q4 unanswered), duplicate/unsupported START policy, and child-process cleanup remain open; do not resolve the full ticket yet.
- 2026-10-03, Q4–Q6: User selected one shared CAL Server specifically to exercise an n-client/server setup, and confirmed the external demo driver sends START and observes the result. User asked whether the proposed START success milestone means demo success or the command-status response. Clarification: it means the orchestrator's terminal ServiceLifecycleCommandStatus for START, distinct from OWP publication acknowledgment, the child's ServiceStatus, and the driver's overall demo verdict. Waiting for child NORMAL before terminal command success remains unconfirmed.
- 2026-10-03, Q6 follow-up: User proposed accepting the work immediately and reporting success when the process starts. Preserve process startup as the selected success milestone, rather than waiting for NORMAL. Normative check of pinned UCI 002.5.0 Normalized Interface Specification Rev A §4.6.1/Figure 4.6-1 (CERT UNIS-000105) and §4.6.2: ACCEPTED/REJECTED/CANCELED are terminal; the Commander ignores later responses after a terminal result. The XSD CommandProcessingStateEnum permits RECEIVED/ACCEPTED/REJECTED/CANCELED, but ServiceLifecycleCommandStatusMDT requires LifecycleCommandStatus with only SUCCESS/FAIL, so there is no faithful outcome-pending lifecycle response. Proposed simplest resolution: a visible local work-taken event/log, followed by one standard terminal ACCEPTED/SUCCESS when process startup succeeds. Await user confirmation; no protocol extension or early misleading success/failure is selected.
- 2026-10-03, Q7–Q10 and final flow confirmation: User confirmed local RECEIVED, OS process creation as START success, rejection of repeated START while starting/running, KISS cleanup, and then explicitly said the demo flow is good. User added a reusable lifecycle crate and confirmed the supervisor/health-signal/async-hook model. Exact crate names were supplied by the user.
- 2026-10-03, persistence: User requested persisting the lifecycle runtime idea and deferring WASIp3 work to a spike. Recorded the runtime asset, resolved this topology ticket, reconciled dependent contracts, and kept interface/dependency selection and acceptance evidence open. The spike is a separate deferred follow-up, not a blocker of the native demo.
