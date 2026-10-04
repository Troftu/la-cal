# 24: Run and document native macOS prototype acceptance

**What to build:** A repeatable documented entry point and completed native macOS acceptance record demonstrate the full relayed STOP exchange through the real ASB, with independent reporting/result/state/exit evidence, bounded cleanup and identified practical test-reuse outcomes.

**Blocked by:** 22 — Complete the two-command Orchestrator STOP relay; 23 — Retarget the external CAL harness and record practical reuse results.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md) and the [final acceptance decision](08-conformance-acceptance-evidence.md#answer). This is integration, execution and evidence consolidation for the approved behavior, not a broader conformance campaign.

- [ ] Document repeatable build, configuration and launch commands for the project-owned driver, local router, server and both Service applications, all executing natively on one macOS host with loopback plain WebSockets and explicit Zenoh TCP configuration. Use the retained dependency lockfile and record actual tool/generator/protoc/router/schema/source versions.
- [ ] Run the complete driver entry point. Observations are installed before application reports/commands; both distinct Services' INITIALIZING/NORMAL transitions and periodic reporting are observed before A is initiated.
- [ ] Observe the distinct A/B command interactions and actual processor identities. B terminates ACCEPTED/SUCCESS and the Commandee reports INOPERABLE in either arrival order; the Orchestrator then sends its own ACCEPTED/SUCCESS for A and remains NORMAL.
- [ ] Observe Commandee process exit independently, with its final publication operations completed before closure/exit. Missing result/state/exit or reporting observations fail acceptance; no OWP acknowledgment substitutes for those milestones.
- [ ] Establish that exercised messages traverse actual envelope encoding, Zenoh publication/subscriber handling, decoding, validation and OWP forwarding. SDK-local delivery is accepted; direct in-memory echo is not final acceptance evidence. Mandatory physical-router-hop proof or a separate adapter-disable/failure-injection campaign is not required.
- [ ] Required waits and cleanup are configurable and bounded with five-second defaults per wait. The driver terminates/reaps remaining owned processes after verdict and on failed/partial startup. Record the actual verdict and cleanup result.
- [ ] Consolidate executed CAL harness and carried-over Rust/generator results, any concrete harness acquisition/adaptation obstacle, failed checks and justified exclusions. Retain nonblocking temporal/whitespace and polymorphic-codec defects; repair issues that obstruct selected runs or corrupt exercised data and rerun affected checks.
- [ ] Link executed requirement-sensitive observations/checks to the selected OMS 2.5 LA-CAL OMSC-SPC-013 Rev B, OMS OMSC-STD-001 Rev M and pinned UCI 002.5.0 source identifiers provided by the spec. Identify Zenoh/Protobuf binding, relay ownership and process-exit milestones as project decisions.
- [ ] Record OS/architecture, exact commands/configuration, versions, results and coverage limits, including the RFC 6455 section 10.6 TLS-capability exception, native macOS-only scope and router-polling/detection limits. Unrun tests and research probes are not completed project acceptance evidence.
- [ ] Use the approved completion claim only after a successful run: the native macOS relayed STOP demonstration succeeds, with identified practical CAL/test reuse results and explicitly recorded coverage limits. The evidence does not imply unqualified OWP/LA-CAL/WebSocket conformance, complete three-root validation correctness, Linux portability or full OMS Service/Platform compliance.
