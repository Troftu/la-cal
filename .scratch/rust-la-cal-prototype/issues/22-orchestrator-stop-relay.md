# 22: Complete the two-command Orchestrator STOP relay

**What to build:** The driver commands the Orchestrator to stop the separate Commandee. The Orchestrator issues a distinct downstream interaction, observes its required result/state milestones and publishes its own truthful upstream terminal result while continuing to report NORMAL.

**Blocked by:** 21 — Implement and observe direct Commandee STOP.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md), especially the two-command demonstration and relay failure contract.

- [ ] After both Services report NORMAL and required startup/periodic reports are observed, the driver sends NEW/STOP with fresh CommandID A, the Orchestrator as Effecting Service, the Commandee as Affected Service and the configured MissionID.
- [ ] The Orchestrator checks the full typed request's Mission, identities, command and state eligibility, retains upstream A context, and issues NEW/STOP with fresh distinct CommandID B and the Commandee as both Effecting and Affected Service. Each outbound message has required security/header content, a fresh message identity and the actual sender.
- [ ] Install persistent downstream result and Service Status subscriptions before issuing B. Application-owned loops match B's terminal result by CommandID and Commandee processor identity and match final INOPERABLE separately by Commandee Service identity. Reception and Orchestrator periodic reporting continue while waiting.
- [ ] Accept B's ACCEPTED/SUCCESS and Commandee INOPERABLE in either arrival order. Only after observing both does the Orchestrator send one terminal ACCEPTED/SUCCESS for A with its own processor identity. It remains NORMAL and does not claim to have observed child process exit.
- [ ] Downstream rejection/failure or expiry of a required result/state wait produces the Orchestrator's own terminal REJECTED/FAIL for A when its connection remains usable, including after B was sent. Preserve upstream context and use truthful optional diagnostics; never rewrite the child's reply, send an early terminal acceptance, fabricate success or infer downstream nonexecution/state. Broken upstream communications are reported visibly for the driver's failure verdict.
- [ ] The driver independently verifies A's upstream result, B's downstream result, final Commandee state, actual Commandee process exit and continued Orchestrator NORMAL behavior. Each observation uses the correct identity/correlation; an acknowledgment, result, state or exit cannot substitute for another milestone.
- [ ] Missing milestones, unexpected disconnects/process exits and operation failures fail visibly under configurable five-second per-wait defaults. The driver bounds shutdown/termination/reaping of remaining owned processes, with no retries or run recovery.
- [ ] Execute and document a native macOS relayed STOP run through the real shared-server Protobuf/Zenoh path, retaining observed A/B identities, reporting, final state, exit and verdict evidence.
