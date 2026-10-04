# Define the Service lifecycle command-and-status round trip

Parent: ../map.md
Labels: wayfinder:grilling
Type: grilling
Status: resolved
Blocked by: 01

## Topology update

The resolved [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer) supplies the current command/reporting contract. The START and self-RESTART answers below are historical; their still-applicable standards grounding does not restore their old demo requirements.

## Question

What precisely happens when the example Service connects, listens, publishes the initial `ServiceLifecycleCommand`, receives it through the ASB path, and replies with `ServiceLifecycleCommandStatus`?

Resolve whether the initial command is `START`, who acts as commander and commandee, how a running example Service handles it, how self-delivery works, and who observes the correlated status. Specify the necessary subscriptions, Service identity, `CommandID` correlation, initialization versus connection readiness, and the observable end of the demonstration. Keep OWP publication acknowledgment, command delivery, command processing status, and Service state distinct.

Choose which actual lifecycle behaviors, states, and additional status messages the prototype demonstrates, and which behavior remains outside its conformance claim. Ground the result in the selected profile and its requirement identifiers.

## Comments

- 2026-10-02, lifecycle discussion Q1: User chose an actual lifecycle transition: the running example Service self-publishes `RESTART`, receives it through the ASB path, and reinitializes. Manager-driven `START` is the future direction: “B would be next, as down the road this service will be a ‘service manager’/‘orchestrator’”. Remaining restart behavior, status evidence, and demonstration completion decisions are still open; this ticket remains claimed.
- 2026-10-02, Q2–Q3: User confirmed application reinitialization while preserving the OWP connection and subscriptions, with an observable initialization counter and a single correlated terminal `ACCEPTED`/`SUCCESS` reply after successful reinitialization. User chose periodic `ServiceStatus` plus a publication on every state transition, and explicitly excluded status-data-request handling from this demonstration. This is scoped lifecycle integration evidence, not full OMS Service compliance.
- 2026-10-02, Q4–Q6: User chose a periodic interval read from an environment variable, defaulting to one second, and all four success observations (ASB-path delivery, NORMAL → INITIALIZING → NORMAL reports, exactly one initialization-counter increment, and a correlated ACCEPTED/SUCCESS reply). User asked whether repeated CommandID handling is necessary; duplicate policy and the remaining Q5 command-handling policy are not yet confirmed.
- 2026-10-02, Q5 follow-up: User explicitly chose to skip the receiver duplicate-command cache. The healthy-path demonstration sends one command with a fresh CommandID and no automatic retries; duplicate execution is outside its evidence scope. User additionally required INITIALIZING on first startup, not only during RESTART: first startup follows INITIALIZING → NORMAL, then the demonstration follows NORMAL → INITIALIZING → NORMAL. Periodic reporting begins once publication is possible; publish the initial INITIALIZING state as soon as the communications path permits, before completing application initialization. These startup details will be included in the final contract confirmation.
- 2026-10-02, final confirmation: User confirmed the summarized lifecycle contract (“looking good”) and asked how it will be tested. The lifecycle-specific test intent below informs the still-open acceptance-evidence decision; no tests or prototype were implemented in this planning session.
- 2026-10-03: User confirmed a native START-only flow with a driver commanding an orchestrator to launch example-service, local RECEIVED followed by one terminal ACCEPTED/SUCCESS after OS process creation, repeated-START rejection, and KISS cleanup. The reusable lifecycle runtime model was also confirmed; WASIp3 work is deferred to a separate spike. The current answer supersedes the historical RESTART contract.

## Answer

Use [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer) for the current sequence, identities, two command results, final reporting/exit, and direct application ownership. Resolution details live there rather than being duplicated here.

## Historical answer — superseded START baseline

### START roles and command result

Use the selected OMS 2.5 / UCI 002.5.0 profile and the [confirmed topology/sequence](09-demo-participant-topology.md#answer): `demo-driver` is the START Commander; `orchestrator` is Commandee, processor, and Effecting Service; `example-service` is Affected Service. One shared `la-cal` server handles their independent OWP connections.

The driver sends one NEW/START with a fresh CommandID, required configured MissionID, EffectingServiceID=orchestrator, and AffectedServiceID=example-service. Naming the affected child does not make it the command processor or require it to receive/execute START; the orchestrator creates its process.

The orchestrator records a local CommandID-correlated RECEIVED event, creates the child process, and sends one terminal ServiceLifecycleCommandStatus after successful OS process creation: CommandProcessingState=ACCEPTED, LifecycleCommandStatus=SUCCESS, original CommandID, and ServiceID=orchestrator. It does not wait for child OWP connection or NORMAL. The orchestrator remains NORMAL while launching the child.

Do not send RECEIVED as an outcome-pending lifecycle status. The schema requires SUCCESS/FAIL LifecycleCommandStatus; ACCEPTED is terminal and later responses are ignored under UCI UNIS §4.6.2. OWP publication acknowledgment is a separate operation result.

### START reporting and command policy

- Both Services enter INITIALIZING on first start, establish communications/subscriptions, publish INITIALIZING before application initialization completes, then automatically enter/report NORMAL. The driver installs observation before the orchestrator starts; the orchestrator installs child-status observation before launching the child.
- Both publish periodic ServiceStatus and on every transition, with the existing environment-configurable one-second default and 1–600-second whole-second range. Variable names and invalid-value handling belong to the interface/configuration decision.
- Reports retain each Service's identity, ServiceState, and running-lifetime TimeUp. There is no CommandID in ServiceStatus and no assumed total delivery order across message types.
- Another START while the managed child is starting or running is REJECTED/FAIL. Process creation failure is REJECTED/FAIL. Unsupported commands designated for this processor are rejected without executing another action; commands designated for a different processor are ignored. Keep the effecting and affected roles separate in target checks.
- No duplicate-CommandID cache, command retry/replay, UPDATE/CANCEL support claim, or lifecycle recovery is added. One-child occupancy rejection is a separate application rule.
- ServiceStatusDataRequest/ServiceStatusDataRequestStatus remain outside this scoped demo and are still an explicit exception to full Service compliance.
- The orchestrator owns child termination/reaping at teardown; the driver cleans up the orchestrator and infrastructure it launched. Teardown is ordinary process cleanup, not an OMS STOP exchange.

### START lifecycle evidence

Verify actual adapter delivery of START and its correlated terminal reply, both Services' INITIALIZING → NORMAL startup reports, actual child process creation/application initialization, and periodic/transition reporting. Command success is not overall demo success: child failure after spawning fails the demo without revising the completed command reply.

Exercise actual WebSocket endpoints and the real Protobuf encode/Zenoh publish/subscriber/decode path. SDK-local delivery remains acceptable with one shared CAL server; physical router traversal is not required. Disable adapter publication/receive forwarding to expose shortcuts. Use bounded event waits with the configured five-second per-wait default, not fixed sleeps. Preserve JSON/identities at the adapter boundary.

Supporting checks include rejection of repeated START without another process, process-creation failure without a fabricated success, foreign/unsupported command handling, reporting configuration, connection-owned subscription cleanup, and child cleanup. RESTART, workload reset, a second initialization-counter increment, and restart connection-preservation checks are deferred.

The [runtime design](../lifecycle-runtime.md) records the shared supervisor, health signals, and application hooks. The [acceptance decision](08-conformance-acceptance-evidence.md) still must produce the full requirement-linked plan. Source grounding in the historical section below remains applicable to initialization/reporting and message definitions; process-spawn success is a project milestone, not a prescribed standard mechanism.

## Historical answer — superseded self-RESTART baseline

### Scope and roles

Under the [selected OMS 2.5 / UCI 002.5.0 profile](01-standards-profile.md), demonstrate a real **self-directed RESTART**, not self-START or an initialization-release command. The example Service acts as both commander and commandee; the LA-CAL Server provides communications and is not the commandee.

Use one stable example Service identity throughout the connection, header sender identity, command processor identity, and affected-Service identity. The command uses:

- `MessageData.CommandID.UUID`: a fresh UUID for the logical command.
- `CommandState=NEW` and `LifecycleState=RESTART`.
- Required `MissionID`: a configured demonstration mission identity.
- `EffectingServiceID`: the example Service, explicitly identifying the processor.
- `AffectedServiceID`: exactly the same example Service for this demonstration.

The reply's `MessageData.ServiceID` identifies the command processor; its `CommandID` repeats the original command identity. Preserve the distinction between effecting and affected identities in the interfaces so a future manager can act on another Service. Exact configuration names and topic/key naming are settled by the component, operating-boundary, and ASB decisions, not inferred from identity fields.

### Startup and restart sequence

1. **First startup enters INITIALIZING.** Establish the OWP connection through INIT/INFO, prepare ordinary (non-group) subscriptions, and initialize application state. OWP connection readiness is not completion of Service initialization.
2. Subscribe the commandee to `ServiceLifecycleCommand`, the commander to `ServiceLifecycleCommandStatus`, and the state observer to `ServiceStatus`. Establish subscription and ASB readiness before the command publication. The test observer must be ready before the Service publishes its first state report.
3. Publish the initial INITIALIZING state once communications permit; complete application initialization and enter NORMAL automatically. Publish NORMAL. Record an initialization counter baseline (normally 1 after first startup).
4. Once NORMAL and the route is ready, the example Service publishes one fresh self-targeted NEW/RESTART. It must reach the commandee through OWP → LA-CAL → Zenoh/Protobuf → LA-CAL → OWP, without a local lifecycle shortcut.
5. Execute only a supported command targeting this Service while NORMAL. Enter INITIALIZING and publish that transition; reset and reinitialize observable application state. Keep the OWP connection and subscriptions alive. The initialization counter is evidence bookkeeping retained across application resets, not reset along with the example workload state.
6. After successful reinitialization, increment that counter exactly once and enter NORMAL automatically; publish the transition.
7. Publish one terminal `ServiceLifecycleCommandStatus` with the original CommandID, `CommandProcessingState=ACCEPTED`, and `LifecycleCommandStatus=SUCCESS`. The Service's command-status subscription observes the correlated reply through the ASB path. There is no early ACCEPTED followed by a second completion reply.

This is application reinitialization, not an operating-system process restart. The healthy path is the demonstration; initialization failures, outages, timeouts and recovery bounds must be addressed explicitly in [operating boundaries](07-prototype-operating-boundaries.md) and the [ASB contract](05-zenoh-asb-contract.md), without inventing additional lifecycle transitions here.

### Reporting and command policy

- Publish `ServiceStatus` periodically **and on every state transition**. Read the periodic interval from an environment variable, defaulting to **one second**; the requirement-aligned configurable range is 1–600 seconds in whole-second increments. The exact variable name and invalid-value behavior belong to the configuration decision.
- State reports use the Service identity, current `ServiceState`, and required `TimeUp`. TimeUp describes the running Service lifetime; application reinitialization does not imply that the process has just started. Reports do not contain CommandID and are not substitutes for a correlated command reply.
- Execute only self-targeted NEW/RESTART while NORMAL. Ignore commands targeting other Services. For this example, unsupported commands targeting this Service produce a terminal REJECTED/FAIL reply, without executing another lifecycle action. This restricted support policy is project-defined and does not establish support for every UCI lifecycle command.
- **No receiver duplicate-command cache**, retries, or duplicate-execution guarantee. Send one fresh command with one exchange in flight. The commander recognizes its terminal reply and ignores later replies for the completed exchange (UCI UNIS §4.6.2). No full UPDATE/CANCEL or retransmission claim is made.
- **No ServiceStatusDataRequest / ServiceStatusDataRequestStatus handling** in this prototype. This is an explicit exception to a full OMS Service compliance case, not a claim that the standard makes requests optional.

### Observable completion and lifecycle test intent

The user selected all four success conditions:

1. Evidence that the command was carried by the actual Zenoh/Protobuf path and delivered to the commandee.
2. INITIALIZING → NORMAL on first startup, then NORMAL → INITIALIZING → NORMAL for the restart, with corresponding state publications.
3. Exactly one additional initialization-counter increment for the command, plus observable reset/reinitialization of application state.
4. A received ACCEPTED/SUCCESS reply correlated by the original CommandID and carrying the processor identity.

The acceptance plan should exercise the actual WebSocket endpoints and Zenoh adapter with a test driver, not only direct state-machine calls. Install observer subscriptions before starting the Service; wait for explicit readiness/events with bounded deadlines rather than fixed sleeps. Retain decoded ASB boundary observations sufficient to check payload/identity preservation, along with local lifecycle events and OWP-received reports. Test that the same connection/subscriptions survive the restart.

Because ServiceStatus has no CommandID and UCI makes no transport-order guarantee, establish lifecycle ordering from the Service's transition events and verify that the corresponding reports arrive; do not assume a total arrival order across status and command-status topics. One command in flight makes the demonstration attributable without extending the UCI message schema.

The subsequent [ASB routing decision](05-zenoh-asb-contract.md#answer) clarifies that this evidence must execute the actual Zenoh publication/subscriber and Protobuf encode/decode path. Zenoh session-local delivery is acceptable; physical router traversal is deferred to a second-phase two-Service/two-LA-CAL-Server demonstration. The no-shortcut negative check disables the adapter path, rather than assuming router removal must prevent Zenoh-local delivery.

Meaningful supporting checks include: unsupported self-targeted commands do not change lifecycle state and return a correlated rejection; foreign-target commands do not change this Service; periodic reporting works with the default and environment override; transitions publish independently of the periodic timer; and disabling the ASB path cannot still produce a successful self-restart through a hidden local shortcut. Mapping-fidelity, protocol-negative and failure checks remain owned by their corresponding decisions and the [acceptance-evidence decision](08-conformance-acceptance-evidence.md).

### Source grounding and claim limits

Use official artifacts and pinned UCI provenance listed in the profile decision:

- **OMS Standard 2.5 §6.1.1, Table 6.1-1**: INITIALIZING → NORMAL is automatic after initialization; NORMAL → INITIALIZING is triggered by RESTART. Initialization cannot complete before the Service can send and receive OMS messages. **STD-000872/STD-000873** provide the NORMAL-state and applicable-transition requirements.
- **OMS §7.2.1, STD-000319/320/321** and Service Contract template Rev M §3.1.2: periodic reporting, configurability, interval range, and reporting once communications permit. Immediate reports on every transition are an explicit project behavior, not an assumed universal reporting deadline.
- **UCI 002.5.0 XSD**: `ServiceLifecycleCommandMDT`, `ServiceLifecycleCommandStatusMDT`, their inherited Command/Header definitions, and `ServiceStatusMDT` define fields, identities and enumerations. START guidance describes beginning a lifecycle in INITIALIZING, normally through a resource manager; it does not define START as INITIALIZING → NORMAL. Schema annotations are guidance per the UCI Standard's compliance section.
- **UCI Normalized Interface Specification Rev A §4.6.1, Figure 4.6-1, CERT UNIS-000105**, and §4.6.2: commander/commandee roles, terminal ACCEPTED/REJECTED/CANCELED processing states, and ignoring subsequent replies after a terminal response. UCI Standard **RQMT USTD-000629** links interactions to those CERTs. No inspected source explicitly mandates a duplicate-NEW cache.
- **LA-CAL Rev B §5.1.1.1.1.2 and §§5.1.1.1.3.2–.3**: PUB/SUB after INFO and topic/message matching; **LACAL-000008/000009/000012/000013/000014** cover PUB/SUB/MSG/+OK/-ERR. `+OK` is a protocol-operation acknowledgment, not delivery, lifecycle execution, or completion evidence.

These decisions support a scoped lifecycle integration demonstration and later requirement-linked component evidence. They do not themselves establish full OMS Service/Platform compliance or complete Command-2 interaction conformance. Manager-driven START and other lifecycle actions are future work beyond this demonstration.
