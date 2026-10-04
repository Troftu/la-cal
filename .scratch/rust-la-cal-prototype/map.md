# Find the way to a working Rust LA-CAL prototype

Labels: wayfinder:map
Status: resolved

## Destination

Settle an implementation-ready, easy-to-use native Rust LA-CAL client and a minimal relayed STOP demonstration using two applications and a driver on one shared LA-CAL Server. Finish the client/component-interface and initial macOS demo-first acceptance decisions, with practical SLEET CAL test reuse and scoped evidence claims, retaining the Zenoh ASB carrying OMS JSON inside Protobuf envelopes.

## Notes

- Planning only: this map ends when the decisions are settled; implementation follows separately.
- Planning completed on 2026-10-04. All child decisions are resolved; use [Define the prototype acceptance and conformance evidence](issues/08-conformance-acceptance-evidence.md#answer) as the final initial acceptance contract and authority over earlier conflicting evidence gates.
- In interface discussions, show short illustrative code examples for the recommended approach and the alternative; the user wants to compare both before choosing.
- Standing design preference: KISS. Consolidate mechanical implementation details rather than interviewing on each one. Production validation remains direct Rust; the final acceptance decision permits known defects unless they cause problems in the selected demo/tests, superseding earlier mandatory repair/independent-assessment gates. A different production engine is not pre-authorized.
- Load `oms-la-cal` every session. For decision conversations, also load `grilling` and `domain-modeling`.
- Standing user constraints: Rust; OWP/WebSockets between client and server; Zenoh and Protobuf on the ASB side. There is no existing ASB contract to match, so this effort may define one.
- Use official UCI **002.5.0**, paired with OMS **2.5**, LA-CAL **OMSC-SPC-013 Rev B**, and **OWP 1.0**. The resolved profile decision supersedes the initial UCI 2.6 preference and supplies the pinned source provenance.
- The user chose OMS JSON inside a project-defined Protobuf envelope over Zenoh as the prototype destination. The resolved encoding decision records its contract; this supersedes schema-derived typed UCI Protobuf generation for this effort.
- Initial acceptance targets the macOS relayed STOP integration behavior and practical CAL/test reuse, with requirement links and recorded coverage limits. Broad OWP client/LA-CAL server conformance evidence is deferred; no unqualified WebSocket, full OMS Service or Platform compliance claim is selected.
- Use [Choose a client-focused relayed STOP demonstration](issues/11-client-focused-stop-demo.md#answer) for the current demo and component responsibilities. Earlier self-RESTART, START/process-launch, and lifecycle-runtime designs are historical.
- Confirmed crate names: `orchestrator`, `commandee-service`, `la-cal`, `la-cal-client`, and `demo-driver`. Applications own their lifecycle logic and client connections directly; focus on concrete client usability and protocol behavior.
- The [resolved component-interface contract](issues/06-rust-component-boundaries.md#answer) supplies the approved cloneable Client API, streams/acknowledgment handling, server/adapter seams, tooling/artifact workflow, defaults, relay failure policy and driver-owned processes. Its final approval supersedes earlier provisional type names/owner-handle split; the final acceptance decision supersedes its broader mandatory evidence/repair gates.
- Use latest stable tooling/dependency releases per the user's 2026-10-04 direction, with latest stable upstream protoc on PATH for build-time generation. The earlier Rust 1.91.0 compatibility probe is evidence, not the selected toolchain baseline. This preference does not change the pinned OMS/UCI profile.
- `la-cal` and `orchestrator` are native. Wasm capability for other Services and the client is a desired follow-up; WASIp3 choices and Wasm demo requirements remain outside this map. Re-scope the deferred spike's old runtime assumptions before execution.
- Initial acceptance runs natively on one macOS machine with loopback-only deployment; Linux execution/verification is deferred. TLS capability is explicitly deferred, reconfirmed for the client on 2026-10-04; evidence must record that exception rather than claim complete WebSocket conformance.
- ASB evidence must exercise the actual Zenoh/Protobuf adapter. The resolved shared-server topology retains SDK-local delivery; a physical router hop and multiple CAL Servers remain later work.
- User selected a project-owned SLEET OWP frontend adaptation for `la-cal` and reuse of its generator for the three demo message roots. Track implemented modifications and pinned provenance in root [NOTICE](../../NOTICE), preserving upstream license/attribution and modified-file notices; the resolved interface ticket holds the adoption/interface contract.
- Tickets are local Markdown children under `issues/`; open tickets are discovered there, not listed in this map. Follow `docs/agents/issue-tracker.md` for claims and blocking.

## Decisions so far

<!-- Resolution details live only in their tickets; add a gist and link here when each closes. -->

- [Investigate an official UCI schema-to-Protobuf generator](issues/02-official-protobuf-generator.md): No official generator located in the checked sources; an official definition effort and generic candidates inform the mapping decision.
- [Choose the OMS/UCI profile with a UCI 2.6 preference](issues/01-standards-profile.md): User chose OMS 2.5 / UCI 002.5.0, matching the standard's release pairing and SLEET; exact artifacts and pinned schema provenance are recorded in the ticket.
- [Define the Service lifecycle command-and-status round trip](issues/03-lifecycle-round-trip.md): Historical self-RESTART/START contracts and reporting grounding; the later relayed STOP decision supplies the current exchange.
- [Define the OMS JSON-in-Protobuf encoding contract](issues/04-schema-protobuf-mapping.md): Single-field envelope preserving validated JSON bytes, server/adapter validation in both directions, no client runtime schema validation, and prost/prost-build bindings.
- [Bound the prototype's deployment and failure behavior](issues/07-prototype-operating-boundaries.md): One-machine macOS/Linux deployment, loopback unencrypted transports with TLS-support exception, fixed demo identities with overrides, visible failure without recovery, and configurable five-second per-wait deadlines.
- [Define the Zenoh ASB routing and delivery contract](issues/05-zenoh-asb-contract.md): One session/startup wildcard listener, message-type routing, local OWP registration, and validated publication acknowledgment retained for multiple clients sharing one CAL Server.
- [Re-evaluate self-RESTART versus an orchestrator and an orchestrated Service](issues/09-demo-participant-topology.md): Historical selection of two Services and a shared server; START/process-launch and runtime choices superseded by the client-focused STOP decision.
- [Assess reusing SLEET's LA-CAL server with a Zenoh/Protobuf ASB](issues/10-sleet-server-reuse-feasibility.md): Source adaptation is feasible through publication/receive seams; required repairs and verification limits inform the selected adaptation and remaining interfaces.
- [Choose a client-focused relayed STOP demonstration](issues/11-client-focused-stop-demo.md): Direct application lifecycle logic, two correlated STOP interactions, commandee final reporting/exit, and an orchestrator-owned upstream result replace START and the lifecycle runtime.
- [Verify the native Rust and Protobuf toolchain compatibility](issues/12-native-toolchain-compatibility.md): Combined macOS probe passed with Rust 1.91.0 and the proposed crate/protoc pins; router execution, Linux verification, and final validator/application graph remain untested.
- [Verify the selected lifecycle bindings and validation coverage](issues/13-lifecycle-bindings-validation.md): Failure fields are grounded and generated closures compile; representative checks expose temporal/whitespace and polymorphic-codec defects; mandatory repair/assessment gates are superseded by initial acceptance.
- [Choose the Rust component interfaces and dependencies](issues/06-rust-component-boundaries.md#answer): Approved single cloneable Client, typed streams, serialized acknowledgments, async offloaded adapter, driver-owned processes, latest-stable tooling and checked-in artifacts; broader repair/evidence gates are superseded by initial acceptance.
- [Define the prototype acceptance and conformance evidence](issues/08-conformance-acceptance-evidence.md#answer): Required native macOS STOP demo, practical external CAL harness reuse and opportunistic internal tests; retain nonblocking defects and limit claims to observed behavior/coverage, superseding broader mandatory repair/evidence gates.

## Not yet specified

<!-- No remaining in-scope fog or open child decisions. The approved initial acceptance contract settles the map's exit; implementation follows separately. -->

## Out of scope

- Building the prototype during this map: the user chose a decision-only Wayfinder effort.
- Linux execution/verification for the initial milestone: user selected macOS only in [Define the prototype acceptance and conformance evidence](issues/08-conformance-acceptance-evidence.md#answer).
- Broad new conformance/failure-injection coverage, comprehensive known-defect repair and mandatory independent XSD assessment: user selected demo-first acceptance plus practical existing CAL/test reuse, retaining defects that do not cause problems in selected runs; see [Define the prototype acceptance and conformance evidence](issues/08-conformance-acceptance-evidence.md#answer). These replace earlier mandatory evidence gates rather than remain unresolved fog.
- A full OMS Service or OMS Platform compliance case: this effort targets the initial demonstration and scoped reused-test evidence described above.
- Integration with an existing external ASB contract: none is required for this prototype.
- Schema-derived typed UCI Protobuf definitions and XSD-to-Protobuf generator selection: deferred to a separate effort by the user's JSON-in-Protobuf decision.
- Distributed deployment, TLS capability, application-managed reconnection/retries/replay, lifecycle recovery, and performance targets: beyond the selected first-draft operating boundary; see [Bound the prototype's deployment and failure behavior](issues/07-prototype-operating-boundaries.md#answer). SDK-managed reconnect attempts do not permit a detected-outage run to resume; see [Define the Zenoh ASB routing and delivery contract](issues/05-zenoh-asb-contract.md#answer).
- Additional application Services beyond the orchestrator and commandee-service, multiple CAL Servers, and forced physical-router-hop evidence: beyond the selected first native demo; see [Choose a client-focused relayed STOP demonstration](issues/11-client-focused-stop-demo.md#answer).
- START, orchestrator-driven process launch, RESTART, and additional lifecycle actions beyond STOP: deferred to keep the client demonstration concrete and small; see [Choose a client-focused relayed STOP demonstration](issues/11-client-focused-stop-demo.md#answer).
- A reusable Service lifecycle runtime, supervisor, application-hook abstraction, and generic lifecycle execution framework: dropped for now because application behavior is not concrete enough to justify the abstraction; see [Choose a client-focused relayed STOP demonstration](issues/11-client-focused-stop-demo.md#answer).
- Detailed client receive buffering/backpressure and configurable overflow-policy design: outside the first client slice; the user delegated simple incidental handling/logging in [Choose the Rust component interfaces and dependencies](issues/06-rust-component-boundaries.md).
- Grouped client subscriptions and a client-facing group-options API: deferred for the first client slice/demo in [Choose the Rust component interfaces and dependencies](issues/06-rust-component-boundaries.md).
- WASIp3 portability and a Wasm first-demo requirement: user deferred these to [Spike WASIp3 support for Service lifecycle and LA-CAL clients](../wasip3-service-portability/issues/01-wasip3-service-runtime-spike.md). This follow-up does not block the native map.
- Production XML/libxml2 assessment and a pre-authorized native fallback: direct Rust validation remains selected. Independent XSD assessment is deferred development/test evidence, not an initial acceptance gate. Any production-engine replacement requires a new decision; see [Choose the Rust component interfaces and dependencies](issues/06-rust-component-boundaries.md#answer) and the superseding [initial acceptance contract](issues/08-conformance-acceptance-evidence.md#answer).
