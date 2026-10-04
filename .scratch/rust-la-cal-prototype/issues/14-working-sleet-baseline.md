# 14: Establish a working project-owned SLEET baseline

**What to build:** A native macOS, project-owned working copy of the selected SLEET components that can establish OWP connections, validate an OMS JSON publication and deliver it to a subscription through the existing in-process router. This is the executable adoption baseline from which subsequent tickets change topic policy and then replace delivery with the actual ASB.

**Blocked by:** None (can start immediately).

**Status:** resolved

Source: [Approved prototype spec](../spec.md). The baseline's in-process exchange establishes source-adoption behavior; final prototype acceptance uses the Zenoh/Protobuf path introduced in ticket 16.

- [x] Adopt the OWP server frontend, protocol support, local subscription/group routing, direct Rust validator, UCI generator and relevant tests from SLEET revision `e3c2d798a229766571ed7be1f5b28d2dcdb51868` into project-owned code. Preserve the upstream license and attribution, add prominent modified-file notices where applicable, and record actual adaptations with revision, date, affected components and changes in the project's modification notice.
- [x] Establish the project-owned Rust workspace, selecting and recording latest-stable tooling and dependencies at baseline establishment. Retain the workspace lockfile and use locked builds/checks. Keep baseline changes narrow enough to identify compatibility adaptations separately from later topic and ASB behavior changes.
- [x] Supply the selected OMS 2.5 / UCI 002.5.0 inputs from UCI revision `73a286fc4b881d9f328dbd64148525606a92c33a`, including the security-marking include and separate artifact provenance/terms. The reference submodules remain read-only.
- [x] Document and execute native macOS build and loopback server launch commands. An authorized test participant completes WebSocket subprotocol `owp` and OWP 1.0 INIT/INFO negotiation, subscribes, publishes a valid OMS JSON message, observes operation acknowledgment and receives the corresponding MSG through the copied local router.
- [x] Execute relevant copied OWP, routing, configuration, schema/validator and generator tests where straightforward, against project-owned code. Record actual results, baseline defects and justified exclusions; fix build/run blockers without creating an all-upstream-tests migration gate.
- [x] The copied generator can be explicitly invoked with the pinned schema inputs and its relevant self-tests execute. Record its source/version and output provenance; ordinary Rust builds do not require running the Python generator.
- [x] Record the OS/architecture, commands, configuration, tool/dependency/schema versions and observed baseline results. Document that this baseline has not yet implemented the selected ASB or final client contract.

## Completion evidence

2026-10-04: Implemented the project-owned workspace and native process exchange.
See [build/run instructions](../../../README.md),
[executed evidence and known limits](../../../docs/baseline-evidence.md), and
[modification record](../../../NOTICE). All 273 Rust tests and 15 generator
self-tests passed on native macOS arm64; the full suite supplied the real pinned
schema path. The baseline retains local-router delivery; ticket 15 is the next
unblocked implementation step.
