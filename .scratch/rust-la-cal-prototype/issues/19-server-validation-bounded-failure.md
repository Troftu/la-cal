# 19: Complete remaining server validation and bounded failure behavior

**What to build:** The adapted server preserves its tested topic/group contract while rejecting invalid ASB input, isolating slow recipients, cleaning up registrations reliably and visibly failing a run after detected router loss, with bounded resource shutdown.

**Blocked by:** 16 — Replace local publication delivery with Zenoh/Protobuf.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md), especially inbound validation, server execution, connectivity and cleanup. Topic policy and ordinary/group routing are established in ticket 15.

- [ ] Both OWP publication and decoded ASB ingress use the selected direct Rust validation and supported three-root policy while preserving accepted original OMS JSON bytes. Inbound malformed Protobuf, missing/empty payload, invalid UTF-8/JSON, unsupported root/key, topic/type mismatch and schema-invalid input are visibly reported and discarded; ordinary unknown envelope fields do not bypass payload checks.
- [ ] Retain known temporal/whitespace and polymorphic-codec findings in the evidence record. Fix defects that obstruct selected demo/tests or corrupt exercised data; comprehensive remediation or independent XSD assessment is not a required initial gate. Any production validator replacement needs a new decision.
- [ ] Use a 64-message outbound queue per server connection with nonblocking enqueue. Overflow visibly fails and closes the slow connection and disposes of all its registrations while healthy recipients continue; silent dropping and blocking all clients are not the selected behavior.
- [ ] Explicit unsubscribe removes only its registration, and connection closure removes every connection-owned registration, including forced task termination paths. Queued stale delivery cannot be attributed to a newly reused subscription ID. Per-client cleanup leaves the shared ASB listener active.
- [ ] Observe stable router-ZID snapshots periodically, defaulting to 100 ms. Detected router loss latches run failure, clears readiness/stops admission and closes clients with bounded cleanup. SDK reconnect cannot resume that run. Record the limits on between-poll outages and underlying transport-loss detection latency.
- [ ] Known infrastructure/publication failures produce visible Internal-Error or connection failure, never successful acknowledgment or an invented Service lifecycle transition. Started blocking work may complete after timeout; cleanup does not depend on indefinitely joining it and timeout does not prove nonpublication.
- [ ] Server shutdown and partial-startup failure dispose of active connections, connection-owned registrations, startup subscriber and session within configured bounds, with bounded WebSocket closure. Required-operation/cleanup waits use configurable five-second defaults per wait.
- [ ] Adapt relevant existing validation, routing, socket and shutdown tests where straightforward. Reconcile queue-drop assertions with overflow-close behavior and group assertions with algorithm-independent one-member delivery. Record executed results, defects and justified exclusions; delivery checks presented as ASB evidence traverse real Zenoh.
