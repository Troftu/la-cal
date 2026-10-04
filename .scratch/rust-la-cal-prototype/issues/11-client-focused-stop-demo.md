# Choose a client-focused relayed STOP demonstration

Parent: ../map.md
Labels: wayfinder:grilling
Type: grilling
Status: resolved
Blocked by: 01, 04, 05, 07, 09

## Question

What concrete demo should exercise an easy-to-use, specification-aligned `la-cal-client` now that the user has dropped the reusable Service lifecycle runtime and deferred START? Settle the two applications' roles, the STOP relay and its command-status responses, and what completion means without introducing a new lifecycle abstraction.

## Comments

- 2026-10-04: Claimed to record the live component/demo reassessment. The user has selected already-running applications, a driver-triggered STOP through the orchestrator, and commandee INOPERABLE publication followed by process exit. The previous START/runtime design is being superseded.
- 2026-10-04: User confirmed the two command interactions and the orchestrator's own upstream ServiceLifecycleCommandStatus after seeing the concrete CommandID/processor fields: “ah that makes sense. we will do it this way.” No client receive API or revised TLS scope was selected.

## Answer

### Components and scope

Prioritize an easy-to-use, specification-aligned `la-cal-client`, exercised by `orchestrator` and `commandee-service` as separate applications using one shared `la-cal` server. `demo-driver` is the thin trigger/observer harness. Preserve OMS 2.5 / UCI 002.5.0, OWP 1.0, typed OMS JSON messaging, the three generated message roots, and the Zenoh/Protobuf adapter contract.

Drop `service-lifecycle` from this effort. Each application owns its lifecycle state, initialization, periodic/transition status reports, message processing, cleanup, and client connection directly. The client provides communications rather than an application supervisor, execution hooks, health coordination, or automatic lifecycle command replies. Preserve one OWP connection per Service and shared messaging handles within an application as needed; concrete ownership/types remain for the interface decision.

Use `commandee-service` in place of `example-service`. START, orchestrator-driven process launch, process-spawn success, and repeated-START occupancy rules are no longer first-demo requirements. The old runtime asset and START answers are historical records.

### Confirmed healthy-path sequence

1. Both applications are already running for the command portion of the demo. Establish communications/subscriptions and report startup/periodic state directly from application code. The driver waits for both distinct Service identities to report NORMAL before sending STOP. Harness launch order and process-exit observation remain interface/acceptance details.
2. Install status observation and matching command-response reception before publications that can produce them. Reception, reporting, and OWP operation acknowledgments remain live while awaiting downstream results.
3. The driver publishes NEW/STOP with fresh CommandID A, EffectingServiceID=orchestrator, and AffectedServiceID=commandee-service, using the configured MissionID.
4. The orchestrator processes A and issues a separate NEW/STOP with fresh CommandID B, EffectingServiceID=commandee-service, and AffectedServiceID=commandee-service. Retain A → B → affected-Service context. This is a new command interaction, not transparent retransmission.
5. Commandee stops its workload, publishes its terminal successful ServiceLifecycleCommandStatus for B and final ServiceStatus(INOPERABLE), completes those publication operations within bounded deadlines, then closes its connection and exits. Accept either arrival order of command status and ServiceStatus.
6. After receiving successful result B and observing commandee INOPERABLE, the orchestrator publishes its own terminal successful ServiceLifecycleCommandStatus for A. It remains NORMAL. Its upstream success does not claim to have observed process exit.
7. The driver verifies the upstream result, commandee's final state report, and process exit for the overall demo verdict. Publication acknowledgments, both command results, state report, and process exit are separate observations.

### Response fields and correlation

Both replies are ordinary `ServiceLifecycleCommandStatus` messages, with required SecurityInformation and MessageHeader. Use fresh message identities and the actual sending Service in each header.

| MessageData field | Downstream result | Upstream result |
| --- | --- | --- |
| CommandID | B | A |
| ServiceID (processor) | commandee-service | orchestrator |
| CommandProcessingState | ACCEPTED | ACCEPTED |
| LifecycleCommandStatus | SUCCESS | SUCCESS |

The orchestrator answers its own request using retained upstream context; do not simply rewrite the child's response. ServiceStatus has no CommandID: attribute it by Service identity within this single-STOP demonstration. Do not send an early terminal ACCEPTED followed by a second completion response for the same command.

### Remaining interface and evidence decisions

Client stream/listener delivery, response matching, concurrent operation handling, subscription ownership, queue bounds, deadlines, connection error/closure behavior, and concrete types remain open. Specify downstream rejection, wait/publication failure, and shutdown-failure handling without fabricating success or adding retries/recovery. Define harness setup, exit observation, and cleanup separately; the orchestrator need not own the commandee process.

The previous loopback/TLS exception remains until explicitly revised. Raising TLS capability was a recommendation, not a selected change. This is planning, not implementation or proof of compliance.

### Normative grounding and project choices

- Official OMS 2.5 `02_OMSC-STD-001_RevM_OMS_Standard_DandD_v2_5.docx`, §6.1.1/Table 6.1-1 and §7.2.1/STD-000873: NORMAL → INOPERABLE on STOP; INOPERABLE means no longer capable of mission functions. Ability to send a final INOPERABLE report is Service-contract-dependent. Sending it and subsequently exiting are selected demo behavior, not a universal process-exit requirement.
- Pinned UCI 002.5.0 XSD at the commit recorded in [Choose the OMS/UCI profile with a UCI 2.6 preference](01-standards-profile.md#answer): ServiceLifecycleCommandMDT, ServiceLifecycleCommandStatusMDT, CommandStatusBaseType, ServiceStatusMDT, and ServiceLifecycleStateEnum define identities, correlation, STOP, and required SUCCESS/FAIL. ServiceStatus contains no CommandID.
- Pinned UCI Normalized Interface Specification Rev A §4.6.1/Figure 4.6-1/CERT UNIS-000105 and §4.6.2: Command-2 has terminal ACCEPTED/REJECTED/CANCELED responses; later responses after terminal completion are ignored. §4 makes no message-arrival-order or guaranteed-delivery assumption.
- Fresh downstream CommandID, the A/B mapping, waiting for downstream success and INOPERABLE, bounded final publications, and driver verification of exit are project decisions, not prescribed intermediary routing or acknowledgment mechanisms.
