# Define the prototype acceptance and conformance evidence

Parent: ../map.md
Labels: wayfinder:grilling
Type: grilling
Status: resolved
Blocked by: 01, 03, 04, 05, 06, 07, 09, 11

## Question

What observable demonstration and requirement-linked checks establish that the planned Rust prototype meets the selected OWP client/LA-CAL server scope and lifecycle integration contract?

Define the launch/configuration story, envelope-to-Rust generation reproducibility evidence, positive command/status exchange through actual Zenoh, correlation and byte-for-byte JSON preservation checks, relevant negative protocol/validation cases, and failure behavior. Incorporate the [encoding decision's evidence requirements](04-schema-protobuf-mapping.md#generation-and-evidence); no typed UCI Protobuf or XSD-to-Protobuf generation claim is intended. Trace compliance-sensitive checks to exact versions, documents, sections, and requirement or certification identifiers. State the precise claims supported by the evidence and any explicit exceptions. This is the map's exit criterion for an implementation-ready plan; running the demonstration belongs to implementation.

## Context

The approved Answer below supersedes the earlier broad acceptance scope in this Question/Context wherever they conflict. Those sections preserve the inputs to the decision, not additional mandatory initial gates.

Use [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer) as the current demo contract. Verify readiness before commands, both command interactions and their distinct CommandIDs/processor identities, downstream result/state observation without an arrival-order assumption, upstream completion, final publications, and actual commandee process exit. Observe both Services' startup and periodic/transition reporting independently of command results.

Exercise missing/rejected downstream results, publication failure, premature disconnect, and timeout according to the [resolved interface/failure contract](06-rust-component-boundaries.md#answer); never fabricate command success. The orchestrator sends its own terminal REJECTED/FAIL for upstream A when possible, including after issuing B; that classification does not establish downstream nonexecution or an inferred Service state. Distinguish expected commandee closure/exit from premature connection loss. demo-driver owns router/server/application launch, readiness, process-exit observation and bounded cleanup; define evidence for that harness contract rather than reopening process ownership. START/process creation, repeated-START occupancy, and RESTART reset checks are no longer first-demo requirements.

The [operating boundary](07-prototype-operating-boundaries.md#answer) fixes native macOS/Linux one-machine deployment, loopback unencrypted connections, no automatic recovery or performance targets, and configurable five-second deadlines per required wait. Record the explicit RFC 6455 §10.6 TLS-support exception; do not claim complete WebSocket conformance. Include meaningful bounded-failure evidence without inventing lifecycle transitions or requiring recovery.

Lifecycle behavior is direct application code, not the historical reusable runtime. Use [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer) for success observations and claim limits. Do not turn status-data-request, duplicate-cache, or PAUSE/RESUME support into assumed requirements. [Choose the Rust component interfaces and dependencies](06-rust-component-boundaries.md#answer) is now resolved and supplies the approved public API, dependency/artifact workflow and operating defaults; this acceptance decision is unblocked.

The [ASB routing evidence](05-zenoh-asb-contract.md#evidence-and-source-grounding) requires actual Protobuf encode/Zenoh publish/subscriber/decode and accepts SDK-local delivery for this shared-server topology. Verify the version-scoped startup wildcard listener, topic/type validation, local OWP registration acknowledgment, multi-client fan-out/cleanup, publication acknowledgment semantics, invalid-input rejection, slow-client overflow, and detected-failure/deadline behavior. Disable adapter publication or receive/forwarding to detect hidden shortcuts. Multiple CAL Servers and physical-router-hop proof remain deferred. SDK-managed reconnect attempts do not authorize resuming a failed demonstration.

Account for OWP subscription-group support separately from the ordinary-subscription STOP scenario: LA-CAL Rev B section 4.8 and section 5.1.1.1.3.3/CERT LACAL-000009 specify one matching member per group, independent groups scoped by topic, and ordinary fan-out. The optional group argument permits clients to omit it; it does not make server support optional. Include requirement-linked group delivery and membership cleanup evidence when defining the OWP server claim. Round-robin is an implementation choice, not a required algorithm; SLEET's implementation/tests are reference evidence only.

WASIp3 compilation/execution and portability-driven dependency/adapter selection are deferred to a [separate spike](../../wasip3-service-portability/issues/01-wasip3-service-runtime-spike.md), not this native first-demo acceptance plan.

### Resolved interface inputs

- Single cloneable Client, type-inferred topics/root wrappers and independent typed streams; application-owned matching and no client callback/runtime framework.
- Serialized acknowledgment-tracked operations with independent receive dispatch. Sent operations remain tracked after caller cancellation; missing-ack deadline fails/closes the connection, with uncertain transmitted publication outcome and unsent queued-operation failure. Test ordinary rejection separately from terminal transport/protocol failure and nonterminal message-decoding errors.
- Explicit bounded connection/subscription close and best-effort drop cleanup; demo-driver bounds termination/reaping of remaining children. Server overflow visibly closes a slow connection at the default 64-message outbound capacity. Elaborate client buffer/overflow design remains excluded.
- One-second periodic plus transition reporting, configurable five-second required waits, and stable 100 ms router snapshots. Observe/report the polling and underlying transport-detection limits; no guarantee of catching every between-poll outage or five-second transport-loss detection is selected. Detected failure is latched despite SDK reconnect.
- Ordinary async adapter publication seam with internal synchronous-work offload/deadlines. Timeout cannot forcibly cancel started work and must not imply nonpublication; stalled child cleanup must remain bounded at the harness level.
- Latest stable tools/dependencies with retained Cargo.lock and actual-version/provenance recording. Check in explicitly generated UCI Rust and bytes-envelope .proto; regenerate/compare UCI artifacts independently of ordinary Cargo builds and exercise prost-build using upstream protoc on PATH.
- Production direct Rust validation with scoped repairs; independent XSD assessment is test/development evidence using the documented strict-JSON-plus-array/$type reconstruction interpretation. Include the [verified repair/coverage gates](../research/lifecycle-bindings-validation.md), generated polymorphic-data preservation, temporal/whitespace negatives/positives and original-byte fidelity. Do not silently introduce a production XML/native fallback.
- Existing probes verify limited macOS/Rust 1.91.0 behavior, not the complete latest-stable workspace, Linux, actual router integration or repaired full coverage. Define the required implementation evidence for those targets without treating research observations as completed acceptance tests.

## Comments

- 2026-10-04: [Choose the Rust component interfaces and dependencies](06-rust-component-boundaries.md#answer) resolved after user approval of its final surface/defaults bundle. All blockers are now resolved. This ticket remains open and unclaimed for the next Wayfinder session; the current session has resolved its one non-research decision.
- 2026-10-04: Resumed and claimed. User selected an initial acceptance gate consisting of the relayed STOP happy-path demonstration plus the existing suites run against SLEET, adapted to run against project-owned `la-cal`. The proposed additional independent protocol/validation/failure acceptance bundle is not selected as the initial gate. Exact reusable suite inventory and adaptation boundaries are being investigated before final confirmation.
- 2026-10-04: User selected macOS-only acceptance for now; Linux execution is deferred. Earlier macOS probes remain feasibility evidence, not completed prototype acceptance.
- 2026-10-04: Suite inventory at SLEET `e3c2d798a229766571ed7be1f5b28d2dcdb51868` distinguishes its external black-box LA-CAL harness (`docs/la-cal-harness-integration.md`, CI exhaustive tier) from embedded Rust and generator tests (`make host-gate`). External harness source is not checked out here; SLEET CI references image `v2026.09.01`, while local Compose uses `latest`. Existing harness configuration covers eleven message families, not the selected three-root lifecycle surface. Socket tests currently instantiate SLEET's in-memory router and need the project adapter path for delivery evidence. SLEET round-robin and saturated-queue-drop assertions must be reconciled with the project's unprescribed group-selection algorithm and selected overflow-close behavior. These are test-adaptation facts, not accepted exclusions or executed results.
- 2026-10-04: User prioritized CAL tests, if obtainable/adaptable without disproportionate migration effort. Other SLEET Rust/generator tests should be carried over where straightforward during migration, rather than all being mandatory. Exact harness availability/configuration compatibility remains an implementation feasibility check; record any unrun/excluded coverage rather than treating it as passed.
- 2026-10-04: User superseded the earlier mandatory repair/independent-assessment acceptance gates: retain known defects when they do not cause problems for the selected demonstration/tests; repair blockers encountered there. Do not require a separate comprehensive defect-remediation campaign merely to reach initial acceptance. Final operational wording and limited claim are awaiting confirmation.
- 2026-10-04: Public upstream [LA-CAL harness README](https://gitlab.com/open-arsenal/ams-gra/hello-world-sk/test/la-cal-harness/-/raw/main/README.md) is accessible and documents native host targets, `LACAL_TARGET_URL`/`LACAL_CONFIG`, pinned UCI 2.5 inputs, preflight and configurable smoke/standard/exhaustive tiers. This establishes a documented retargeting route, not an executed macOS compatibility or three-root fixture result. Pin the actual revision used at implementation rather than treating mutable `main` or `latest` as the evidence baseline.
- 2026-10-04: User approved the final six-part initial acceptance contract. Resolved as a planning decision; no prototype acceptance run has occurred.
- 2026-10-05: User added the external harness reference. The [source assessment](../../../docs/la-cal-harness-assessment.md) records the pinned checkout and concrete retargeting/coverage risks, superseding the earlier source-unavailability observation. No harness run or change to the approved Answer is implied.

## Answer

### Approved initial gate and authority

On 2026-10-04 the user approved demo-first acceptance: a required native macOS relayed STOP demonstration, practical reuse of the external CAL harness, opportunistic migration of other SLEET tests, and repair only of defects that cause problems in the selected demonstration/tests. This decision narrows the map's exit contract and supersedes conflicting mandatory repair, independent-assessment and broad conformance-evidence gates in earlier tickets, including [Choose the Rust component interfaces and dependencies](06-rust-component-boundaries.md#bindings-schema-validation-and-artifacts), [Define the OMS JSON-in-Protobuf encoding contract](04-schema-protobuf-mapping.md#generation-and-evidence), and [Verify the selected lifecycle bindings and validation coverage](13-lifecycle-bindings-validation.md#answer).

The supported wire profile, component responsibilities, public API and artifact workflow remain the selected design inputs. Acceptance does not require broadening the three-root message surface or adding lifecycle features. Implementation begins separately after this map.

### Required native macOS demonstration

Use [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer) and the [approved component/harness interfaces](06-rust-component-boundaries.md#answer):

1. `demo-driver` launches/owns the router, `la-cal`, `orchestrator` and `commandee-service`, installs observations before relevant application publications, and uses the selected readiness checks before commanding. Observe both distinct Services' startup and periodic reporting and wait for both to report NORMAL.
2. The driver sends NEW/STOP with fresh CommandID A, orchestrator as Effecting Service, commandee as Affected Service and the configured MissionID. Orchestrator issues NEW/STOP with distinct CommandID B targeting commandee and retains A-to-B context.
3. Observe commandee's terminal ACCEPTED/SUCCESS for B with its processor identity and final ServiceStatus(INOPERABLE). Accept either arrival order. Final publication operations complete before commandee closes its connection and exits.
4. Orchestrator observes both downstream milestones and publishes its own terminal ACCEPTED/SUCCESS for A with orchestrator processor identity. It remains NORMAL. The driver separately observes upstream success, final commandee state and actual commandee process exit; no one observation substitutes for another.
5. Publications/delivery use the actual Protobuf encode, Zenoh publish/subscriber, decode and OWP forwarding path. SDK-local delivery within the shared-server topology is allowed; no physical router hop is required. An in-memory echo cannot supply this demonstration.
6. Apply the selected configurable five-second per-wait deadlines and bounded remaining-child cleanup. Missing required observations produce a failed demonstration rather than fabricated success. A separate failure-injection campaign is not an initial gate.

Linux execution is deferred. A macOS-hosted Linux container running the application server is not native macOS server evidence. The driver command/configuration and actual launch/readiness procedure must be documented by implementation so the run can be repeated.

### Practical SLEET test reuse

Prioritize the external [LA-CAL harness](https://gitlab.com/open-arsenal/ams-gra/hello-world-sk/test/la-cal-harness), described in SLEET's `docs/la-cal-harness-integration.md`. SLEET revision `e3c2d798a229766571ed7be1f5b28d2dcdb51868` is the inspected integration reference; its CI exhaustive tier is the reference run, not a promised test count or a transferred pass result.

- Obtain a concrete harness revision and record it. Use its documented preflight and `LACAL_TARGET_URL`/`LACAL_CONFIG` support to point at project-owned `la-cal`. Retain the pinned UCI 002.5.0 inputs and align authorized identities, topics and message roots with the project. Existing eleven-family SLEET configuration is not the project's configuration.
- Try the existing certification tests, with the exhaustive tier as the starting reference, where adaptation is practical. Document configuration/fixture changes, actual tier and cases, failures and excluded/unrun checks. If acquisition or adaptation is disproportionate, record the concrete obstacle and missing coverage; it need not block the first successful demonstration. Unrun tests are never reported as passed.
- Carry over relevant embedded Rust and generator tests where straightforward during migration. There is no all-tests migration gate. Tests copied with an adopted component should target project-owned code; invoking Cargo against the read-only SLEET submodule is not evidence about our `la-cal`.
- Adapt assertions to the selected contract: group-member selection is not required to be round-robin, and server queue overflow closes the slow connection rather than silently dropping messages. Delivery scenarios used as adapter evidence must traverse real Zenoh; pure routing-unit checks establish only local membership/selection behavior.
- A failed selected check must be resolved or explicitly recorded as a justified adaptation/exclusion with its coverage limit. Do not transfer SLEET's historical harness pass claim or remove an assertion and describe its coverage as passing.

### Known defects and artifact workflow

The [known temporal/whitespace and polymorphic-codec findings](../research/lifecycle-bindings-validation.md) remain recorded. They may remain in this first slice when they do not cause problems for the selected demo/tests. Fix defects that break those runs or corrupt the messages/data they exercise. Comprehensive selected-constraint repair, expanded negative/derived-value corpora and independent XSD assessment are deferred acceptance work, not automatic reasons to block this initial milestone or change production engines.

Retain checked-in generated UCI Rust and the project envelope `.proto`, explicit UCI regeneration, build-time prost-build with upstream protoc on PATH, Cargo.lock and original-byte-preserving adapter design. This acceptance decision does not add a mandatory separate regenerate/compare, exhaustive byte-fidelity or independent-validation campaign. Preserve the existing provenance/NOTICE workflow for source adaptations.

### Requirement links and the supported claim

These links ground demonstrated or reused-test behavior; they are not a claim that every listed requirement has already been tested. The run report must map each executed harness check to its actual source identifier and record remaining coverage.

| Behavior | Pinned source / distinction |
| --- | --- |
| OWP negotiation and connection establishment | OMS 2.5 OMSC-SPC-013 Rev B §§5.1, 5.1.1.1.3.1/.5/.7; CERT LACAL-000002–000005, -000007, -000011, -000013. OWP INIT/INFO establishes communications, not completed Service initialization. |
| Publish, subscriptions, delivery and cleanup | Same document §§4.8, 5.1.1.1.1.4, 5.1.1.1.3.2–.4/.6/.8; CERT LACAL-000008–000010, -000012, -000014. Server group support is required even though demo/client subscriptions omit groups; unrun group checks limit the evidence. |
| OMS JSON message mapping | Same document §5.1.1.1/CERT LACAL-000006 and §6.1.1–.5. Test reuse and a successful three-root exchange do not establish comprehensive schema-validity equivalence. |
| Service state/readiness/reporting | OMS 2.5 OMSC-STD-001 Rev M §6.1.1/Table 6.1-1 and §7.2.1/RQMT STD-000319–000321, -000872/-000873. Startup/transition reports and final report/process exit are selected demo behavior; periodic demo reporting is narrower than full Service compliance. |
| Command fields and interaction | Pinned UCI 002.5.0 XSD (`73a286fc4b881d9f328dbd64148525606a92c33a`) and UCI Normalized Interface Specification Rev A §§4, 4.6.1/Figure 4.6-1/CERT UNIS-000105, §4.6.2. A/B relay ownership, waits and exit observation are project choices. ServiceStatus has no CommandID. |
| Transport exception | RFC 6455 §10.6 requires TLS support. Selected loopback plain-WebSocket deployment omits that capability; preserve the explicit exception. |
| Zenoh/Protobuf and operation milestones | [Project ASB contract](05-zenoh-asb-contract.md#answer), [encoding contract](04-schema-protobuf-mapping.md#answer) and [client/component contract](06-rust-component-boundaries.md#answer), rather than a standardized ASB binding. Publication acknowledgment is distinct from delivery, command success and process exit. |

Retain commands, configuration, OS/architecture, source/schema/tool/generator/dependency/harness versions and results, including known relevant defects and any exclusions. Actual workspace/test results must replace the earlier limited research probes as implementation evidence.

The permitted completion statement is: **the native macOS relayed STOP demonstration succeeds, with the identified practical CAL/test reuse results and explicitly recorded coverage limits.** Do not label an unrun or failed optional suite as passing. This supports only observed behavior and executed checks; it does not establish unqualified OWP/LA-CAL/WebSocket conformance, complete three-root validator correctness, Linux portability, or full OMS Service/Platform compliance.

### Map exit

All planning decisions for this narrowed first milestone are settled. Implementation owns the concrete harness adaptation effort and run evidence within the approved limits; there is no remaining in-scope decision ticket. Broader validation, failure coverage and Linux verification are deferred beyond this map's destination.
