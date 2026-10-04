# 21: Implement and observe direct Commandee STOP

**What to build:** A directly addressed Lifecycle Command stops the Commandee workload and produces separately observable terminal command status, final INOPERABLE Service Status and actual process exit. This establishes the downstream behavior the Orchestrator relay will use.

**Blocked by:** 18 — Complete client cancellation, failure and graceful closure; 20 — Launch and observe both ready Services.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md), especially Commandee targeting, final publications and independent result/state/exit milestones. A focused direct-command verification establishes this slice; the final driver acceptance uses the relay added in ticket 22.

- [ ] The ready Commandee consumes its typed Lifecycle Command stream and checks Mission context, effecting/affected identities, NEW/STOP and relevant state eligibility in application code. Receiving a publication on the command topic does not itself authorize execution.
- [ ] For a valid direct STOP, the Commandee ceases its workload and sends one terminal ServiceLifecycleCommandStatus with the original CommandID, its own processor ServiceID, CommandProcessingState ACCEPTED and LifecycleCommandStatus SUCCESS. Include required SecurityInformation and MessageHeader with a fresh message identity and actual sender.
- [ ] Publish final ServiceStatus(INOPERABLE), with its Service identity and no CommandID. Await both terminal result and final state publication operations before bounded shared-client closure and process exit. Neither report's arrival order is assumed.
- [ ] Install persistent result and Service Status observations before sending the direct command. Match result by CommandID and processor identity, match final state independently by Service identity, and observe the owned Commandee process exit separately.
- [ ] The observer fails on missing required milestones, premature disconnect/exit or failed final publication, using configurable per-wait deadlines and bounded remaining-child cleanup. Expected closure after final messaging is distinguishable from premature loss.
- [ ] Application failures and ineligible/unsupported targeted requests are reported truthfully under the selected terminal failure policy when communications permit. Do not fabricate success, infer unobserved state or retry. Optional diagnostics have a valid required Reason and truthful Description; omit optional polymorphic AssociatedID in the minimal sender.
- [ ] Execute a native macOS direct STOP check through the actual Protobuf/Zenoh adapter and record the independently observed result, state and process-exit milestones. Keep the already-running Orchestrator available for the next relay slice.
