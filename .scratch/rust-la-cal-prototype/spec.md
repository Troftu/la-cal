# Native Rust LA-CAL client and relayed STOP prototype

Status: ready-for-agent
Labels: ready-for-agent
Source: [Find the way to a working Rust LA-CAL prototype](map.md)

## Problem Statement

Developers need a usable Rust OWP Client for exchanging typed OMS messages through an LA-CAL Server backed by Zenoh. The repository currently contains standards, reference implementations, research and resolved design decisions, but no project-owned Rust workspace implementing that exchange. SLEET supplies useful OWP frontend, routing, validation and generation code, but its in-process router does not implement the selected Zenoh/Protobuf ASB binding, and its existing client does not provide the selected independent typed reception and concurrent-operation behavior.

The user needs a small, runnable demonstration that makes client usability concrete: a Commander requests STOP through an Orchestrator, the Orchestrator commands a separate Commandee, and independent command results, Service Status and actual process exit establish what happened. Publication acknowledgment alone cannot establish those outcomes. The first milestone must be practical to build and verify on native macOS without turning into a generic Service lifecycle framework or a comprehensive conformance campaign.

## Solution

Build a native Rust workspace containing `la-cal-client`, `la-cal`, `orchestrator`, `commandee-service` and `demo-driver`. Provide one cloneable, typed client with persistent subscription streams, application-owned response matching and explicitly bounded closure. Adapt the selected SLEET OWP frontend in project-owned code and replace its publication/delivery path with the real Zenoh adapter carrying original OMS JSON bytes in a minimal Protobuf envelope.

Provide a documented demo entry point that launches and owns a local router, one shared LA-CAL Server and both Service applications; observes readiness and reporting; initiates the two-command STOP relay; verifies the required command, state and exit milestones; and cleans up remaining processes. Use the selected OMS 2.5 / UCI 002.5.0 profile throughout.

Initial acceptance consists of a successful native macOS demonstration, practical reuse of the external CAL harness, straightforward reuse of relevant SLEET tests and an honest record of executed checks, defects and coverage limits. The final acceptance decision governs this spec wherever earlier planning called for broader gates.

## User Stories

1. As a Rust application developer, I want a reusable OWP Client library, so that I can exchange OMS messages without implementing transport handling in each application.
2. As a Rust application developer, I want connection establishment to await successful WebSocket and OWP negotiation, so that a returned client is immediately usable for messaging.
3. As a Rust application developer, I want generated bindings for the three selected UCI message roots, so that I can construct and consume typed messages for the demo.
4. As a Rust application developer, I want generated root, topic and profile metadata, so that I do not repeat wire-format strings at each send or subscription.
5. As a Rust application developer, I want typed send completion to await OWP Publication Acknowledgment, so that I can distinguish operation completion from local serialization or socket writing.
6. As a Rust application developer, I want a cloneable client sharing one connection, so that reporting and application messaging can use the same Service connection.
7. As a Rust application developer, I want concurrent application calls to remain attributable to their own acknowledgments, so that an operation cannot consume another operation's response.
8. As a Rust application developer, I want message reception to continue while an operation awaits acknowledgment, so that a received message cannot stall or invalidate an otherwise successful publication.
9. As a Rust application developer, I want persistent typed subscription streams, so that my application controls message processing and task ownership.
10. As a Rust application developer, I want each subscription to own an independent registration, so that repeated subscriptions to one type can be closed independently.
11. As a Rust application developer, I want explicit asynchronous subscription closure, so that I can observe whether unsubscribe completed.
12. As a Rust application developer, I want dropped subscriptions to stop local delivery and request best-effort cleanup, so that incidental resource release does not require a blocking destructor.
13. As a Rust application developer, I want a message-specific decoding error to be visible without ending the stream, so that subsequent valid messages can still be processed.
14. As a Rust application developer, I want unexpected connection failure to appear before the subscription ends, so that application code can distinguish failure from ordinary completion.
15. As a Rust application developer, I want server rejection details and uncertain publication outcomes preserved in errors, so that I do not infer delivery or nonpublication incorrectly.
16. As a Rust application developer, I want transmitted operations to remain tracked after caller cancellation, so that later acknowledgments cannot be misattributed.
17. As a Rust application developer, I want a missing acknowledgment to fail the shared connection, so that a late response cannot be assigned to a subsequent operation.
18. As a Rust application developer, I want bounded graceful client closure, so that final messaging and process cleanup can complete without indefinite waits.
19. As a Service application author, I want to own lifecycle logic and protocol replies directly, so that the demonstration does not require a generic execution framework.
20. As a Service application author, I want INITIALIZING, NORMAL and periodic Service Status reporting, so that observers can distinguish communication establishment from application readiness.
21. As a Service application author, I want explicit command targeting and state checks, so that receiving a message-type publication does not imply that my Service must execute it.
22. As an LA-CAL Server operator, I want OWP participant and topic authorization, so that omitting credentials in the loopback demo does not bypass protocol support checks.
23. As an LA-CAL Server operator, I want validated publications to traverse the actual Zenoh/Protobuf adapter, so that the demo proves the selected ASB integration rather than a local echo.
24. As an LA-CAL Server operator, I want validation at both OWP publication and ASB ingress, so that rejected or malformed input is not forwarded as a valid message.
25. As an LA-CAL Server operator, I want accepted OMS JSON bytes preserved, so that validation does not alter the payload carried through the ASB.
26. As an LA-CAL Server operator, I want exact message-type routing keys and one profile-scoped startup listener, so that routing remains simple and subscriptions need no per-client Zenoh resources.
27. As an LA-CAL Server operator, I want ordinary subscription fan-out and required server-side group behavior, so that the server retains its selected OWP contract even though the demo omits groups.
28. As an LA-CAL Server operator, I want connection-owned subscription cleanup, so that closing a client does not leave stale routing entries or tear down the shared ASB listener.
29. As an LA-CAL Server operator, I want slow server-side client queues to fail visibly and close that connection, so that one slow recipient does not block all clients or silently lose queued messages.
30. As an LA-CAL Server operator, I want router readiness before client admission and visible failure on detected router loss, so that a detected outage cannot be hidden by SDK reconnection.
31. As a demo user, I want one driver to launch and own the router, server and applications, so that setup, verdict and cleanup are reproducible.
32. As a demo user, I want observations installed before reports and commands can occur, so that startup and completion evidence is not lost to subscription races.
33. As a demo user, I want both distinct Services observed as NORMAL before STOP, so that the exchange starts from application readiness rather than assumed readiness.
34. As a Commander, I want a STOP request addressed to the Orchestrator with the Commandee as Affected Service, so that the demonstration exercises distinct effecting and affected roles.
35. As an Orchestrator, I want a distinct downstream command and retained upstream context, so that I can answer my own command interaction without rewriting the child's response.
36. As an Orchestrator, I want to observe the downstream successful result and final INOPERABLE report in either order, so that success does not depend on an unsupported cross-message arrival-order assumption.
37. As a Commandee, I want to stop workload and complete final status publications before closing and exiting, so that the selected shutdown milestones are observable.
38. As a Commander, I want separate CommandIDs and processor identities for the two results, so that the downstream result cannot be mistaken for the Orchestrator's upstream result.
39. As a demo user, I want actual Commandee process exit observed independently, so that a final state report is not treated as proof of process termination.
40. As an Orchestrator, I want to remain NORMAL after successfully stopping the Commandee, so that managing another Service does not automatically stop my own Service.
41. As a Commander, I want a truthful terminal upstream failure when relay success conditions are not met, so that missing results cannot produce fabricated success.
42. As a demo user, I want configurable bounded waits and visible failure causes, so that invalid setup or missing milestones fail predictably and allow a fresh run.
43. As a demo user, I want distinct predefined identities and Mission configuration with overrides, so that the default demonstration is easy to run and remains consistent on the wire.
44. As a maintainer, I want checked-in UCI bindings and explicit regeneration, so that ordinary Cargo builds do not depend on running the Python UCI generator.
45. As a maintainer, I want a checked-in bytes-envelope definition and build-time Protobuf generation, so that the small ASB framing contract remains explicit.
46. As a maintainer, I want latest-stable tooling selected at baseline establishment with a retained lockfile and version record, so that subsequent builds use an identified dependency snapshot.
47. As a maintainer, I want read-only reference submodules and recorded source adaptations, so that upstream provenance and project modifications remain distinguishable.
48. As a maintainer, I want practical external CAL harness reuse against project-owned code, so that existing protocol evidence can be gathered without creating an unrelated test migration project.
49. As a maintainer, I want straightforward existing tests carried over where useful, so that adopted components retain relevant regression coverage.
50. As a maintainer, I want nonblocking known defects retained and blockers repaired for selected runs, so that initial acceptance stays focused on the demonstration and practical test reuse.
51. As an evidence consumer, I want executed checks, failures and exclusions linked to their requirements, so that successful observations are not overstated as complete conformance.
52. As an evidence consumer, I want the plain-WebSocket TLS exception and native macOS scope recorded, so that the milestone is not mistaken for complete WebSocket, Linux or OMS compliance evidence.

## Implementation Decisions

### Scope, baseline and responsibility boundaries

- Establish a new project-owned Rust workspace with the five confirmed components: `la-cal-client`, `la-cal`, `orchestrator`, `commandee-service` and `demo-driver`. Reference code is available, but the project implementation does not yet exist.
- Implement and accept the first milestone natively on one macOS host. Linux execution and verification are deferred. A macOS-hosted Linux container serving the application does not provide native macOS server evidence.
- Fix the supported wire profile to OMS 2.5, UCI schema 002.5.0, LA-CAL OMSC-SPC-013 Rev B and OWP 1.0, with WebSocket subprotocol `owp`. Source the UCI 2.5 artifacts from revision `73a286fc4b881d9f328dbd64148525606a92c33a`; the current UCI reference checkout's 2.6 artifacts are not substitutes.
- `la-cal-client` owns transport, OWP operation attribution, typed encoding/decoding and subscription delivery. The applications own Service lifecycle state, reporting, processing, response matching, protocol replies and teardown.
- Each Service owns one OWP connection shared by its application tasks. The driver has its own independent connection. Do not introduce a reusable lifecycle runtime, callback execution framework or automatic lifecycle reply formatter.
- `la-cal` owns the OWP frontend, authorization, local subscription/group routing, direct Rust validation, server-side queues and the Zenoh/Protobuf adapter. Adopt SLEET's frontend and generator by project-owned source adaptation, not by modifying reference submodules or retaining its in-memory delivery path as the ASB.
- `demo-driver` owns launch, readiness, process observation, the overall verdict and bounded cleanup. The Orchestrator does not launch or own the Commandee process.

### Typed client contract

- Provide a single cloneable `Client` with asynchronous connect, generic typed send by reference, generic typed subscribe and explicit close operations, returning attributable client errors. Connection configuration supplies endpoint, OWP participant identity and configurable deadlines; the supported profile and verbose acknowledgment mode are fixed by the selected implementation.
- Connect returns only after successful WebSocket establishment, OWP INIT/INFO exchange and supported-profile checks. Communications readiness is distinct from application NORMAL.
- Generate the three application-facing global roots: `ServiceLifecycleCommand`, `ServiceLifecycleCommandStatus` and `ServiceStatus`, including their complete required type/value dependencies, security markings, headers and OMS JSON root wrappers. Keep bindings in a versioned messages module within the client library.
- Generated message support supplies root/topic/profile encoding and decoding. Typed send and subscribe infer wire mapping; internal XSD types are not independently publishable roots. No additional raw-message public API is selected.
- Each subscribe creates its own connection-local OWP subscription ID and typed stream, including repeated subscriptions to the same type. Each stream receives matching messages independently. The first client API uses ordinary subscriptions without group options.
- Subscription streams yield either a decoded message or a receive error, then end when closed. A message-specific decoding failure is nonterminal; unexpected connection failure yields a visible terminal error before end-of-stream. Malformed OWP control/framing is a protocol failure, not a recoverable payload decode error.
- Explicit asynchronous subscription close consumes the subscription, awaits UNSUB completion and leaves other registrations active. Drop ends local delivery and requests best-effort unsubscribe without promising acknowledged cleanup.
- Closing any client clone closes the shared connection for all handles and subscriptions. Dropping an ordinary clone does not close it; last-owner resource cleanup is best effort. Applications own reliably awaited teardown.
- Keep client receive-buffer mechanics simple and delegated. No configurable overflow API or receive-loss guarantee is selected; this is distinct from the fixed server-side outbound queue contract.

### Operation attribution, errors and closure

- Permit concurrent application calls while internally serializing acknowledgment-tracked PUB, SUB and UNSUB operations: only one transmitted operation awaits its response per connection. OWP responses lack operation IDs; do not rely on a new response-pipelining/FIFO contract.
- Dispatch incoming MSG and WebSocket controls independently while operations await responses. A MSG may arrive before the corresponding publication acknowledgment. Application consumption must not hold the connection reader hostage.
- Typed send completes on successful OWP Publication Acknowledgment. It does not establish subscriber receipt, command execution, lifecycle completion or process exit.
- Once transmitted, background I/O retains ownership of an operation after caller cancellation. Consume its response or reach its operation deadline before advancing the queue; cancellation does not undo publication, retry it or independently close the connection.
- If a transmitted operation's acknowledgment deadline expires, fail the shared connection, stop admitting/sending operations, fail pending callers, notify subscriptions terminally and perform bounded cleanup. A transmitted publication has an uncertain outcome; queued unsent operations were not transmitted. Do not attribute a late response to a new operation.
- Distinguish invalid configuration, connection/negotiation failures, local encoding/decoding failures, attributable OWP rejection, deadline/transport/protocol failures and closed-connection use. Preserve server details and sent-versus-unsent facts; exact error variant names are implementation details.
- Graceful client close stops new admissions, drains already-admitted operations within the configured deadline, closes WebSocket and finishes background I/O. Report drain/closure failure and continue bounded cleanup rather than retrying or waiting indefinitely. Applications await final publications before invoking close.

### ASB encoding, validation and routing

- Use a project-defined proto3 `OmsJsonEnvelope` with only the bytes field `oms_json`, permanently assigned field number 1. Carry complete OMS JSON payload bytes, not OWP framing. Do not duplicate type/profile/version metadata in the envelope or generate typed UCI Protobuf from XSD.
- Validate on OWP publication before envelope encoding/publication and on inbound ASB samples after envelope decoding before OWP forwarding. Keep production validation in the SLEET-style direct Rust validator; client applications perform typed parsing and application checks without runtime schema assessment.
- Preserve accepted original OMS JSON UTF-8 bytes. Validation may parse or normalize assessment values but must not replace the original payload with reserialized JSON.
- Reject malformed Protobuf, missing/empty payloads, invalid UTF-8/JSON, unsupported roots, topic/type mismatches and schema-invalid messages according to the selected validator's supported behavior. Ignore ordinary unknown envelope fields while still validating the payload. Report and discard invalid inbound ASB samples; do not forward them as valid.
- Known temporal/whitespace validator defects and polymorphic-codec limitations remain documented. They may remain when they do not cause problems in the selected demo/tests. Repair defects that break selected runs or corrupt exercised data; the initial milestone does not require comprehensive remediation or independent XSD assessment. Complete validator correctness is not established by successful compilation or Serde decoding.
- No production XML/libxml2 assessment or native fallback is pre-authorized. A proposed production engine replacement requires a new decision. If independent assessment is used in development, retain strict JSON checks and the documented schema-ordered array / `$type` to `xsi:type` reconstruction interpretation rather than claiming an exact normative XML bridge.
- Map each OWP topic to the identically named root and one exact Zenoh publication key:

  | OWP topic / root | Zenoh publication key |
  | --- | --- |
  | `ServiceLifecycleCommand` | `la-cal/uci/002.5.0/ServiceLifecycleCommand` |
  | `ServiceLifecycleCommandStatus` | `la-cal/uci/002.5.0/ServiceLifecycleCommandStatus` |
  | `ServiceStatus` | `la-cal/uci/002.5.0/ServiceStatus` |

- Open one Zenoh session with an explicit loopback router connection and external discovery disabled. Establish one shared startup subscriber on `la-cal/uci/002.5.0/*`, retained until server shutdown. Check the received exact key and JSON root; the wildcard does not expand supported roots or versions.
- Before admitting OWP clients, establish router/session readiness and the startup listener. SUB installs its local routing entry before successful acknowledgment; UNSUB removes its entry before successful acknowledgment. These operations do not create or destroy Zenoh listeners.
- PUB success requires validation, original-byte envelope encoding and successful completion of the Zenoh publication operation. Only then may the server send verbose +OK. Errors are visible in both verbose and non-verbose mode. Known failures never receive a successful acknowledgment.
- Deliver through actual envelope encode, Zenoh publication/subscriber handling, envelope decode, validation and OWP forwarding. SDK-local delivery is acceptable. Do not directly echo OWP PUB into OWP recipients or republish inbound samples into Zenoh.
- Fan out valid inbound samples to every matching ordinary subscription, including the publisher's matching subscriptions. Subscription IDs are connection-local; repeated matching registrations each receive delivery.
- Retain required server-side group registration and one matching member per group, with independent groups scoped by topic and ordinary subscriptions still receiving fan-out. Group selection need not be round-robin. Client group options and grouped demo use remain deferred.
- Delivery is live and non-durable, with no history, replay, application delivery acknowledgment, duplicate suppression or exactly-once guarantee. Do not assume a total arrival order across message types.

### Server execution, connectivity and cleanup

- Expose an ordinary async adapter publication operation to the OWP frontend. The adapter internally offloads synchronous validation/encoding/Zenoh work, owns its deadline and returns success/error without exposing blocking-worker orchestration to callers. Connection I/O remains independent.
- Started blocking work cannot be forcibly canceled; timeout does not prove nonpublication. Bounded cleanup must not depend on indefinitely joining it. Driver-owned process termination/reaping bounds a stalled child's lifetime.
- Use stable router-ZID snapshots for bounded startup readiness and periodic connectivity observation, with a default 100 ms interval. Observed router loss latches run failure and closes client connections; SDK reconnection cannot resume that run. Between-poll outages may be missed, and underlying transport detection is not guaranteed to occur within an event-wait deadline.
- Preserve OWP participant/topic authorization, negotiation, subscription ID checks and standard errors. Ordinary publication rejection does not itself close an otherwise valid connection. Infrastructure failure may produce Internal-Error or connection failure; it must not invent a Service lifecycle transition.
- Use a simple 64-message bounded outbound queue per server connection with nonblocking enqueue. On overflow, report failure, close the slow connection and dispose of its registrations rather than silently dropping messages or blocking all clients.
- Explicit unsubscribe removes only its connection-owned entry. Connection closure removes all of that connection's entries; neither removes the startup listener. Server shutdown disposes of active connections, subscriber and session with bounded cleanup.

### Demonstration and application behavior

- The driver launches the router and server, waits for communications readiness, connects and installs observations before launching applications that can emit startup reports. It launches and owns both Services, which are already running before the command phase.
- Each application establishes its own client connection and required subscriptions, reports INITIALIZING before initialization completes, reports NORMAL once ready, and emits periodic plus transition Service Status directly from application code. Default periodic reporting is one second, with the selected whole-second 1–600-second configurability.
- The driver waits for both distinct Service identities to report NORMAL before publishing STOP. Application code checks configured Mission context, effecting/affected identities, supported command and relevant state eligibility; message-type routing is not application targeting.
- The driver sends NEW/STOP with fresh CommandID A, Orchestrator as Effecting Service and Commandee as Affected Service. The Orchestrator retains this upstream context and issues a separate NEW/STOP with fresh CommandID B, Commandee as both Effecting and Affected Service.
- Install persistent result and state subscriptions before relevant commands. Applications own matching through their typed streams; do not add client reply waiters. Match downstream command result by B and Commandee processor identity, and match final INOPERABLE Service Status separately by Commandee identity.
- The Commandee stops its workload, sends a terminal result for B with its processor ServiceID, CommandProcessingState ACCEPTED and LifecycleCommandStatus SUCCESS, and publishes final ServiceStatus(INOPERABLE). Await both publication operations before bounded connection closure and process exit. Either report may arrive first.
- The Orchestrator keeps reporting while awaiting downstream observations. Only after observing both B's successful result and Commandee INOPERABLE does it send its own terminal ACCEPTED/SUCCESS for A with its own processor identity. It remains NORMAL and does not claim to have observed child process exit.
- Both command results include required SecurityInformation and MessageHeader, fresh message identities and the actual sender identity. ServiceStatus has no CommandID. Retain A-to-B context; never rewrite the child's result into an upstream reply or emit an early terminal ACCEPTED followed by another completion response.
- The driver verifies the upstream A result, final Commandee state and actual Commandee process exit independently, alongside the downstream result and required reporting observations. Publication acknowledgment, command results, state and exit are separate milestones. Expected Commandee closure after final messaging is successful completion; premature loss is failure.
- Downstream rejection/failure or expiry of a required downstream result/state observation produces the Orchestrator's own terminal REJECTED/FAIL for A when its connection remains usable, including if B was already sent. This project failure classification does not prove downstream nonexecution or any unobserved state. Do not fabricate success or retry.
- Optional failure diagnostics use CommandProcessingStateReason with a valid required Reason and truthful optional Description. Omit optional polymorphic AssociatedID in the minimal sender. If upstream communications fail, report the cause visibly and let driver failure/deadlines establish the verdict.
- Use configurable five-second defaults per required operation, event and cleanup wait, not for the whole run. Wait on bounded readiness/events rather than fixed sleeps. Invalid configuration, initialization failure, required-operation failure, missing milestones or detected unexpected disconnect fail visibly and require a fresh run.
- After verdict, the driver performs bounded shutdown/termination and reaping of remaining owned processes, including failed-startup paths. Provide documented build, configuration and launch commands and a repeatable demo entry point; exact command-line/configuration spelling is delegated.

### Configuration, tooling and provenance

- Bind native WebSockets and Zenoh TCP to loopback with explicit local router configuration, no external discovery and no credentials. Plain `ws` is selected; TLS capability, certificate/trust configuration and `wss` support are deferred, retaining the explicit RFC 6455 §10.6 exception.
- Supply distinct predefined participant/Service identities and a Mission UUID with environment overrides, keeping OWP identity, sender/header identity and targeting consistent. Fail invalid configuration visibly. Endpoint defaults and environment-variable names are mechanical implementation choices.
- Select latest stable Rust/Cargo, Tokio, tokio-tungstenite, Zenoh, prost/prost-build and necessary stream/Serde/schema support dependencies at implementation baseline establishment. Recheck actual releases; earlier research lookup versions are not an immutable toolchain pin or a demonstrated complete workspace baseline.
- Retain the workspace Cargo.lock, use locked builds/checks and record actual tool, dependency, generator and schema versions. Use stable Zenoh APIs and plain-WebSocket features.
- Explicitly generate and check in selected UCI Rust artifacts with schema/generator provenance. Ordinary Cargo builds compile these artifacts without running the Python UCI generator. Document explicit regeneration; a separate mandatory regenerate/compare campaign is not an initial acceptance gate.
- Check in the project-owned bytes-envelope Protobuf source. Use prost-build during Cargo builds with latest stable upstream protoc on PATH; protoc is build-only. No custom compiler path, vendored compiler or older compiler pin is selected.
- Adapt SLEET revision `e3c2d798a229766571ed7be1f5b28d2dcdb51868` with preserved Apache-2.0 terms/attribution and prominent modified-file notices. Record each actual source/generator adaptation's date, affected files, upstream revision and change description in the project's modification notice. Preserve separate UCI artifact provenance/terms. Reference submodules remain read-only.
- Keep mechanical helper placement, internal types, incidental buffers, equivalent configuration spellings and simple implementation choices delegated under KISS; do not reopen settled interfaces for each detail.

## Testing Decisions

### Testing seams and test quality

- Use the already approved highest-level seam as the main oracle: the driver observes real processes, typed messages through the deployed server/adapter and independent process exit. This exercises the client, server, adapter, applications and harness together without adding internal test-only interfaces.
- Reuse the external CAL harness at the existing WebSocket/OWP boundary for server protocol checks. This complements the end-to-end verdict without exposing new internals or substituting in-memory delivery for adapter evidence.
- Good tests assert externally visible behavior: acknowledgments, messages, identities/correlation, independently observed state/exit, errors and bounded completion. Avoid assertions about private task layout, worker handles, queue representation or a particular group-selection algorithm.
- Use subscriptions installed before publications and event-driven deadlines rather than sleeps. Distinguish an OWP operation's success from delivery and application outcomes. No arrival-order assumption is permitted between command results and Service Status.
- Existing public client operations, real OWP peers and deployed adapter behavior are the preferred regression boundaries where selected reuse or encountered blockers need checks. Do not create a separate broad failure-injection suite as an initial gate.

### Required native macOS acceptance

1. Run the project-owned driver, server, Services and actual Zenoh/Protobuf adapter natively on macOS with the configured local router. Record commands, configuration, versions and results.
2. Observe both Services' startup/transition and periodic reporting and both NORMAL identities before initiating A. Confirm the distinct A/B interactions and processor identities.
3. Observe B's terminal ACCEPTED/SUCCESS and Commandee's final INOPERABLE in either order, followed by the Orchestrator's own terminal ACCEPTED/SUCCESS for A while it remains NORMAL.
4. Observe Commandee process exit separately. Final publication operations must complete before its closure/exit; missing required observations fail the demonstration. Bound all required waits and remaining-child cleanup.
5. Establish that the messages used the actual Protobuf encode, Zenoh publish/subscriber, decode, validation and OWP forwarding path. SDK-local delivery is accepted; direct in-memory echo is not. The initial gate does not require forced physical-router traversal or a separate adapter-disable/failure-injection campaign.

### Practical test reuse and known defects

- Use the now-available read-only external LA-CAL harness reference and [source assessment](../../docs/la-cal-harness-assessment.md) to retarget its documented preflight, target URL and configuration to project-owned `la-cal`. Record the actual run revision and adaptations. Use UCI 002.5.0 and align authorized identities, topics and fixtures to the three-root surface rather than copying SLEET's eleven-family configuration unchanged.
- Start from the exhaustive tier as a reference and try certification tests where adaptation is practical. Record the actual revision, selected tier/cases, fixture/configuration changes, failures and excluded/unrun coverage. Disproportionate adaptation may be documented as a concrete obstacle without blocking the first successful demonstration.
- Carry over relevant embedded Rust and generator tests where straightforward during migration. There is no all-tests migration gate. Tests must exercise project-owned adaptations; running a suite against the read-only SLEET submodule does not establish project behavior.
- Prior art includes SLEET's external CAL harness integration, embedded OWP socket tests, local routing/group checks and generator self-tests. Socket tests that instantiate its in-memory router need adaptation before they can supply project ASB delivery evidence. Pure routing tests establish only local membership/selection behavior.
- Reconcile copied SLEET internal-test assertions with the approved contract: round-robin is not mandatory, and server outbound overflow closes the slow connection rather than silently dropping messages. The inspected external group tests do not require round-robin; use the [harness assessment](../../docs/la-cal-harness-assessment.md#executable-coverage-and-adaptation-risks) for its concrete assertion conflicts and report limits. Preserve truthful coverage when adapting tests.
- A failed selected check must be fixed or explicitly recorded as a justified adaptation/exclusion with its coverage limit. Do not describe removed assertions, unrun tests or upstream historical passes as project passes.
- Retain nonblocking temporal/whitespace and polymorphic-codec findings in the evidence record. Repair defects that obstruct selected runs or corrupt exercised data. Comprehensive defect repair, expanded negative corpora, independent XSD assessment and exhaustive fidelity/generation campaigns are not required initial gates.

### Requirement-linked evidence and completion claim

For each executed requirement-sensitive check, record the actual source identifier and coverage rather than treating the following links as already-tested requirements:

| Behavior | Selected source |
| --- | --- |
| WebSocket/OWP establishment and negotiation | OMS 2.5 OMSC-SPC-013 Rev B §§5.1, 5.1.1.1.3.1/.5/.7; CERT LACAL-000002–000005, -000007, -000011, -000013 |
| Publication, subscription/group delivery and cleanup | Same document §§4.8, 5.1.1.1.1.4, 5.1.1.1.3.2–.4/.6/.8; CERT LACAL-000008–000010, -000012, -000014 |
| OMS JSON mapping | Same document §5.1.1.1 / CERT LACAL-000006 and §§6.1.1–.5 |
| Service state and reporting | OMS 2.5 OMSC-STD-001 Rev M §6.1.1 / Table 6.1-1 and §7.2.1; RQMT STD-000319–000321, -000872/-000873 |
| Command payload and interaction | Pinned UCI 002.5.0 XSD and Normalized Interface Specification OAC-SPC-002 Rev A §§4, 4.6.1 / Figure 4.6-1 / CERT UNIS-000105 and §4.6.2 |
| TLS-capability exception | RFC 6455 §10.6; selected plain-WebSocket implementation omits required TLS capability |
| Zenoh/Protobuf, relay ownership and acknowledgment milestones | Project encoding, ASB, component and relayed STOP decisions; these are not a standardized UCI ASB binding |

Retain OS/architecture, commands/configuration, source/schema/tool/generator/dependency/harness versions, results, known relevant defects and exclusions. Earlier limited research probes are feasibility evidence, not completed prototype acceptance. Unrun group checks and other protocol checks limit the evidence even when the server retains their intended implementation behavior.

The permitted completion statement is: **the native macOS relayed STOP demonstration succeeds, with the identified practical CAL/test reuse results and explicitly recorded coverage limits.** This establishes observed behavior and executed checks, not unqualified OWP/LA-CAL/WebSocket conformance, complete three-root validation correctness, Linux portability or full OMS Service/Platform compliance.

## Out of Scope

- Linux execution/verification, distributed deployment, additional application Services, multiple CAL Servers and mandatory physical-router-hop proof for this first milestone.
- WASIp3 portability, target/runtime/executor/transport selection or a Wasm demonstration requirement; the deferred spike must be re-scoped around application-owned lifecycle behavior and the concrete native client.
- A reusable Service lifecycle runtime, supervisor, health/control framework, application hooks or generic execution abstraction.
- START, Orchestrator-driven process launch, RESTART, PAUSE/RESUME and lifecycle actions beyond the selected STOP flow; a full Minimum Message Set or Service/Platform compliance case.
- Client callback dispatch, automatic command replies, client-side reply waiters, raw-message API additions, grouped client subscriptions and elaborate configurable client backpressure/overflow design.
- UCI 2.6 support, full-schema Rust generation, typed UCI Protobuf, XSD-to-Protobuf generator selection or matching an existing external ASB contract.
- TLS capability and credentials, production security deployment, application-managed reconnection/retries/replay, lifecycle recovery, durability, exactly-once processing and performance targets.
- Mandatory comprehensive known-defect remediation, independent XSD assessment, exhaustive byte-fidelity/regeneration campaigns, broad new failure injection or all-tests migration as initial acceptance gates.
- A production XML/libxml2 engine or pre-authorized native validation fallback. Any replacement needs a new decision.

## Further Notes

### Decision authority and implementation handoff

This spec synthesizes the completed decision-only map; publishing it does not implement or verify the prototype. Use the approved Answers rather than historical Questions/Comments when they differ. In particular, the final acceptance decision supersedes earlier mandatory repair, independent-assessment and broader evidence gates; the relayed STOP and final component decisions supersede older self-RESTART, START and lifecycle-runtime designs.

- [Define the prototype acceptance and conformance evidence](issues/08-conformance-acceptance-evidence.md#answer): final initial gate and coverage/claim limits.
- [Choose the Rust component interfaces and dependencies](issues/06-rust-component-boundaries.md#answer): approved public signatures, responsibilities, errors, defaults and artifact workflow, subject to the final acceptance narrowing.
- [Choose a client-focused relayed STOP demonstration](issues/11-client-focused-stop-demo.md#answer): application roles, A/B interaction and distinct milestones.
- [Choose the OMS/UCI profile with a UCI 2.6 preference](issues/01-standards-profile.md#answer): final selection is OMS 2.5 / UCI 002.5.0 and pinned authoritative artifacts.
- [Define the OMS JSON-in-Protobuf encoding contract](issues/04-schema-protobuf-mapping.md#answer): bytes-envelope and fidelity/validation design.
- [Define the Zenoh ASB routing and delivery contract](issues/05-zenoh-asb-contract.md#answer): namespace, listener ownership and acknowledgment/delivery semantics.
- [Bound the prototype's deployment and failure behavior](issues/07-prototype-operating-boundaries.md#answer): loopback deployment, visible failure and per-wait deadlines, with macOS-only initial acceptance superseding earlier Linux evidence expectations.
- [Verify the selected lifecycle bindings and validation coverage](issues/13-lifecycle-bindings-validation.md#answer) and [verified findings](research/lifecycle-bindings-validation.md): grounded failure fields and known defects; earlier comprehensive repair gates are superseded.
- [Verify the native Rust and Protobuf toolchain compatibility](issues/12-native-toolchain-compatibility.md#answer): limited executed feasibility evidence, not final workspace acceptance.
- [Assess reusing SLEET's LA-CAL server with a Zenoh/Protobuf ASB](issues/10-sleet-server-reuse-feasibility.md#answer): source-adaptation seams and migration findings.

Use the project's [domain glossary](../../GLOSSARY.md) terminology and load `oms-la-cal` for implementation. The standards and reference submodules remain read-only. The existing [modification notice](../../NOTICE) records design provenance; append actual adaptations when they occur.

The external [LA-CAL harness](https://gitlab.com/open-arsenal/ams-gra/hello-world-sk/test/la-cal-harness) is checked out under `standards-and-references/la-cal-harness`. The [source assessment](../../docs/la-cal-harness-assessment.md) records the inspected revision and test-adaptation boundaries. Native macOS execution and selected-fixture compatibility remain implementation feasibility checks, not completed evidence.

[Spike WASIp3 support for Service lifecycle and LA-CAL clients](../wasip3-service-portability/issues/01-wasip3-service-runtime-spike.md) remains a separate deferred follow-up and does not block this spec. Its historical runtime/START assumptions do not restore those requirements here.
