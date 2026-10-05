# 20: Launch and observe both ready Services

**What to build:** A project-owned driver launches and owns the local router, LA-CAL Server, Orchestrator and Commandee, installs observations before application reports can occur and observes both distinct Services become ready and keep reporting. Startup and cleanup are repeatable and bounded.

**Blocked by:** 17 — Provide the selected typed Rust client.

**Status:** resolved

Source: [Approved prototype spec](../spec.md), especially component ownership, reporting, configuration and driver readiness.

- [x] Provide the confirmed `demo-driver`, `orchestrator` and `commandee-service` applications alongside `la-cal` and `la-cal-client`. Application lifecycle/reporting logic is owned directly by each Service; no generic lifecycle runtime or handler framework is introduced.
- [x] The driver launches and owns router/server processes, waits for communications readiness, establishes its independent typed client connection and installs status/result observations before launching applications. It then launches and owns both Service processes; the Orchestrator does not launch or own the Commandee.
- [x] Each Service owns one client connection shared by its application tasks, installs required persistent subscriptions, reports INITIALIZING before initialization completes and NORMAL only when ready, and emits transition plus periodic Service Status. Periodic reporting defaults to one second and supports whole-second values from 1 through 600.
- [x] Supply distinct predefined OWP participant/Service identities and a Mission UUID with overrides. Resolve identities consistently across server authorization, OWP INIT, payload headers/senders and application targeting. Invalid configuration or initialization fails visibly.
- [x] The driver observes both distinct Service identities' startup transitions and periodic reporting and gates the command phase on both reporting NORMAL. Communications establishment alone does not establish application readiness.
- [x] Use event/readiness-based waits with configurable five-second defaults per required wait, rather than fixed sleeps. Required-operation failure, missing readiness, unexpected process exit or disconnect fails startup visibly.
- [x] On normal readiness completion and failed/partial startup, the driver performs bounded shutdown/termination and reaping of remaining owned processes, including the router/server and both applications. Document and execute a repeatable native macOS readiness run with actual versions/configuration and results.

## Comments

2026-10-05: Claimed for implementation from `0f06f6d`. Tests use the already
approved spec §Testing Decisions seam: real driver processes and typed reports
through the deployed server/Zenoh adapter, with independent cleanup observations.
STOP execution/relay remain tickets 21–22; no generic lifecycle runtime is added.

2026-10-05: Completed native `demo-driver --readiness` and seven deployed checks,
including identity/Mission/period overrides, partial-startup fresh run, early exit,
missing periodic reporting and forced termination/reaping. Initial stub test was
red before implementation. Locked all-target workspace check, formatting and
Clippy pass; full workspace suite **326 tests, zero failed/ignored**, with pinned
full-schema parsing enabled. Independent review final totals: Standards 0, Spec 0;
the sole suggested smell was clarified with named driver readiness milestones.
Versions, executed commands, native output and limits are recorded in
[Service readiness evidence](../../../docs/service-readiness-evidence.md).
Readiness is complete; command execution/relay/final verdict remain tickets 21–23.
