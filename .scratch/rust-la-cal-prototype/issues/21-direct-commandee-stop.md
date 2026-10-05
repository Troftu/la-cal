# 21: Implement and observe direct Commandee STOP

**What to build:** A directly addressed Lifecycle Command stops the Commandee workload and produces separately observable terminal command status, final INOPERABLE Service Status and actual process exit. This establishes the downstream behavior the Orchestrator relay will use.

**Blocked by:** 18 — Complete client cancellation, failure and graceful closure; 20 — Launch and observe both ready Services.

**Status:** resolved

Source: [Approved prototype spec](../spec.md), especially Commandee targeting, final publications and independent result/state/exit milestones. A focused direct-command verification establishes this slice; the final driver acceptance uses the relay added in ticket 22.

- [x] The ready Commandee consumes its typed Lifecycle Command stream and checks Mission context, effecting/affected identities, NEW/STOP and relevant state eligibility in application code. Receiving a publication on the command topic does not itself authorize execution.
- [x] For a valid direct STOP, the Commandee ceases its workload and sends one terminal ServiceLifecycleCommandStatus with the original CommandID, its own processor ServiceID, CommandProcessingState ACCEPTED and LifecycleCommandStatus SUCCESS. Include required SecurityInformation and MessageHeader with a fresh message identity and actual sender.
- [x] Publish final ServiceStatus(INOPERABLE), with its Service identity and no CommandID. Await both terminal result and final state publication operations before bounded shared-client closure and process exit. Neither report's arrival order is assumed.
- [x] Install persistent result and Service Status observations before sending the direct command. Match result by CommandID and processor identity, match final state independently by Service identity, and observe the owned Commandee process exit separately.
- [x] The observer fails on missing required milestones, premature disconnect/exit or failed final publication, using configurable per-wait deadlines and bounded remaining-child cleanup. Expected closure after final messaging is distinguishable from premature loss.
- [x] Application failures and ineligible/unsupported targeted requests are reported truthfully under the selected terminal failure policy when communications permit. Do not fabricate success, infer unobserved state or retry. Optional diagnostics have a valid required Reason and truthful Description; omit optional polymorphic AssociatedID in the minimal sender.
- [x] Execute a native macOS direct STOP check through the actual Protobuf/Zenoh adapter and record the independently observed result, state and process-exit milestones. Keep the already-running Orchestrator available for the next relay slice.

## Comments

2026-10-05: Claimed from `354194b`. Use the spec §Testing Decisions' already
approved seam: real owned processes, typed observations through the deployed
server/Zenoh adapter, and independently observed process exit. This slice adds
direct STOP only; relay remains ticket 22.

2026-10-05: Completed `demo-driver --direct-stop` natively through the real
Zenoh/Protobuf adapter. Independent terminal result, INOPERABLE and successful
Commandee exit are required; Orchestrator must freshly report NORMAL after all
three observations before bounded remaining-child cleanup. Targeted ineligible
requests receive REJECTED/FAIL; other processors are ignored. Fresh headers use
Timestamp/actual sender: pinned UCI HeaderType defines no additional MessageID.

Ten new checks cover real application targeting/STOP, separate missing milestones,
state-first delivery, failed exit, stale Orchestrator reports, final acknowledgment
waits and both final-publication failures. Full locked workspace suite passed
**336 Rust tests, zero failed/ignored**, with pinned full-schema parsing enabled;
generator self-tests **15/15**. Locked all-target check, formatting and Clippy pass.
Independent review final totals: Standards 0, Spec 0. Evidence and coverage limits:
[Direct STOP evidence](../../../docs/direct-stop-evidence.md). Relay remains ticket 22.
