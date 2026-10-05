# 18: Complete client cancellation, failure and graceful closure

**What to build:** Applications can safely cancel waits, encounter operation/connection failure and close a shared client without losing operation attribution, making false publication-outcome claims or hanging during cleanup.

**Blocked by:** 17 — Provide the selected typed Rust client.

**Status:** resolved

Source: [Approved prototype spec](../spec.md), especially operation attribution, errors and closure.

- [x] After an operation is transmitted, background I/O retains ownership when its caller stops awaiting it. Consume its response or reach the configured operation deadline before transmitting the next acknowledgment-tracked operation. Cancellation alone does not close the connection, undo publication or retry it; reception continues independently.
- [x] A transmitted operation's acknowledgment deadline fails the shared connection: stop admitting/sending operations, fail pending callers, emit terminal subscription errors and perform bounded cleanup. Do not advance the queue or attribute a late response to another operation.
- [x] Errors distinguish invalid configuration, establishment/profile failure, local encoding/decoding failure, attributable OWP rejection, deadline/transport/protocol failure and closed-client use. Preserve server details and transmitted-versus-unsent facts. A sent publication with missing acknowledgment has an uncertain outcome; queued unsent operations were not transmitted.
- [x] An ordinary attributable server rejection does not itself fail a valid connection. Malformed OWP controls are terminal protocol failures rather than recoverable payload decode errors. All live clones and streams observe shared terminal failure consistently.
- [x] Explicit close through any clone stops new admissions, drains already-admitted operations within the configured deadline, progresses bounded WebSocket closure and finishes background I/O. Return visible drain/closure failure and continue bounded cleanup when necessary. Applications can await final publications before calling close.
- [x] Dropping an ordinary client clone does not close the shared connection. Last-owner and subscription drop cleanup remain best effort; reliably acknowledged teardown comes from explicitly awaited close.
- [x] Configurable required-operation and cleanup waits default to five seconds per wait. Missing acknowledgments and closure failure cannot cause indefinite waits, automatic retry or implicit recovery.
- [x] Execute focused observable checks using public client operations and real OWP peers where needed to establish canceled-sent-operation attribution, no queued sends after acknowledgment expiry, uncertain sent outcomes, terminal stream notification and bounded graceful closure. Record results without creating a separate broad failure-injection campaign.

## Implementation evidence and review

2026-10-05: Implemented with an implementation subagent and separate independent
Standards and Spec review subagents. See [client closure evidence](../../../docs/client-closure-evidence.md)
for executed checks, observed red/green defects, requirement links and coverage limits.
Independent review exposed a receive-liveness defect under write backpressure;
the single-task reader/writer split and focused canceled-backpressure regression
repair it. Two low duplicated-code findings were consolidated. Final review totals:
**Standards 0; Spec 0**.

The coordinator's final locked full-workspace run passed **300 Rust tests**, zero
failed/ignored, including all 16 public-client/peer checks, 2 doctests, native
Zenoh process checks and the enabled pinned full-schema parser check. Locked
all-target checking, formatting, Clippy and whitespace checks pass. Ticket 19
is the next unblocked implementation slice.
