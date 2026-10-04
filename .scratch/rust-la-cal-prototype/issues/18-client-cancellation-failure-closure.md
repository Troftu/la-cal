# 18: Complete client cancellation, failure and graceful closure

**What to build:** Applications can safely cancel waits, encounter operation/connection failure and close a shared client without losing operation attribution, making false publication-outcome claims or hanging during cleanup.

**Blocked by:** 17 — Provide the selected typed Rust client.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md), especially operation attribution, errors and closure.

- [ ] After an operation is transmitted, background I/O retains ownership when its caller stops awaiting it. Consume its response or reach the configured operation deadline before transmitting the next acknowledgment-tracked operation. Cancellation alone does not close the connection, undo publication or retry it; reception continues independently.
- [ ] A transmitted operation's acknowledgment deadline fails the shared connection: stop admitting/sending operations, fail pending callers, emit terminal subscription errors and perform bounded cleanup. Do not advance the queue or attribute a late response to another operation.
- [ ] Errors distinguish invalid configuration, establishment/profile failure, local encoding/decoding failure, attributable OWP rejection, deadline/transport/protocol failure and closed-client use. Preserve server details and transmitted-versus-unsent facts. A sent publication with missing acknowledgment has an uncertain outcome; queued unsent operations were not transmitted.
- [ ] An ordinary attributable server rejection does not itself fail a valid connection. Malformed OWP controls are terminal protocol failures rather than recoverable payload decode errors. All live clones and streams observe shared terminal failure consistently.
- [ ] Explicit close through any clone stops new admissions, drains already-admitted operations within the configured deadline, progresses bounded WebSocket closure and finishes background I/O. Return visible drain/closure failure and continue bounded cleanup when necessary. Applications can await final publications before calling close.
- [ ] Dropping an ordinary client clone does not close the shared connection. Last-owner and subscription drop cleanup remain best effort; reliably acknowledged teardown comes from explicitly awaited close.
- [ ] Configurable required-operation and cleanup waits default to five seconds per wait. Missing acknowledgments and closure failure cannot cause indefinite waits, automatic retry or implicit recovery.
- [ ] Execute focused observable checks using public client operations and real OWP peers where needed to establish canceled-sent-operation attribution, no queued sends after acknowledgment expiry, uncertain sent outcomes, terminal stream notification and bounded graceful closure. Record results without creating a separate broad failure-injection campaign.
