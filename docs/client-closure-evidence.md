# Ticket 18: client cancellation, failure and closure evidence

Implementation and focused checks executed 2026-10-05 from clean main baseline
`57fbc5f299b24ff1fadb2fcb4043790529441f54`, native macOS 26.6.2 (25G83),
arm64 / `aarch64-apple-darwin`. Ticket 18 is **resolved** after independent
review and final coordinator verification recorded below.

## Behavior and fixes

The existing approved Client/Subscription API and serialized background OWP loop
are retained. No public API, wire profile or retry/recovery mechanism was added.

- A sent operation remains owned by background I/O after its caller cancels.
  Its response/deadline precedes the next tracked transmission; MSG and ping/pong
  reception remains live even under socket-write backpressure. A canceled
  publication's rejection is consumed by that operation, never returned to the
  next caller. Automatic Pong flushing does not stall later MSG dispatch.
- Terminal connection failure is latched at the admission boundary. Pending
  transmission, queued-unsent callers, later clone calls and close waiters retain
  the same cause, with transmission facts appropriate to each caller. A failed
  sent PUB has an uncertain outcome; this is not evidence of nonpublication.
- Terminal operation/drain failures notify live streams before waiting on peer
  cleanup. Successful draining keeps registrations alive until WebSocket closure
  finishes, so a closure-only failure also becomes a terminal stream error.
- Explicit close atomically stops admissions across clones and drains admitted
  operations. An attributable rejection during drain is reported by close after
  continuing the drain. It remains an ordinary operation rejection, not a
  protocol failure. Terminal connection failure takes precedence.
- Draining and WebSocket closure each receive a separate configurable
  `close_timeout` wait. Cleanup proceeds after drain failure; an expired drain
  deadline no longer suppresses the WebSocket Close attempt. An in-progress
  socket write observes a later shutdown request and adopts its drain deadline.
- Canceling a close waiter leaves shutdown running; concurrent/later waiters
  receive the same completed result. Ordinary clone Drop leaves the connection
  usable, and last-owner/subscription Drop remain best effort.

Connection, operation and each cleanup wait retain five-second defaults.
Applications should await final publications before close. Configuration and
establishment/profile failures, local encoding, nonterminal typed decoding,
exact OWP rejections, deadline/transport/protocol failures and gracefully
closed-client use retain the existing distinct error surfaces. Failure latching
does not set `transmitted: true` on a new operation or on close itself.

## Sources and profile

- Fixed profile: **OMS 2.5 / UCI 002.5.0 / OWP 1.0 / OMSC-SPC-013 Rev B**,
  verbose acknowledgments and plain loopback `ws` with subprotocol `owp`.
- Normative source read directly from the official DOCX, using read-only ZIP/XML
  extraction of its paragraphs:
  `standards-and-references/oms-standard/docs_official/20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.docx`.
  Relevant text: §5.1.1.1.1.3 positive/negative acknowledgments;
  §5.1.1.1.1.4 shutdown and connection-owned subscription disposal;
  §5.1.1.1.2 protocol conventions (one operation per WebSocket message);
  §§5.1.1.1.3.6–.8 MSG, +OK and -ERR syntax; CERT LACAL-000012–000014.
  The unofficial Markdown was used only for navigation.
- Cancellation ownership, single pending acknowledgment, terminal missing-ack
  behavior, graceful drain, stream errors and per-wait defaults are **project
  decisions**: [approved spec](../.scratch/rust-la-cal-prototype/spec.md)
  lines 88–108 and 156, and
  [approved ticket 06](../.scratch/rust-la-cal-prototype/issues/06-rust-component-boundaries.md#approved-public-client-surface).
  OWP's responses carry no operation identifier; the selected serialization
  avoids assuming a general response-pipelining/FIFO guarantee.
- Existing client/deployed evidence and pinned schema provenance remain in
  [typed client evidence](typed-client-evidence.md) and
  [UCI provenance](../spec/002.5.0/PROVENANCE.md), respectively. UCI source revision
  is `73a286fc4b881d9f328dbd64148525606a92c33a`; current UCI 2.6 references
  are not substituted. Reference submodules were not modified.
- Executed tools: Rust `1.99.0 (b940084d7 2026-09-28)`, Cargo
  `1.99.0 (5f94df478 2026-08-27)`. Cargo.lock unchanged: Tokio `1.53.2`,
  tokio-tungstenite `0.30.0`, futures-util `0.3.34`, serde `1.0.229`,
  serde_json `1.0.151`. No dependency or generated-message changes.
- Review follow-up consulted RFC 6455 §§5.5.2–.3 directly at the
  [RFC Editor](https://www.rfc-editor.org/rfc/rfc6455.html#section-5.5.2): Pong
  must retain Ping application data and be sent as soon as practical. The
  pressure regression verifies the identical Pong once the peer drains the
  partially transmitted data frame; immediate reception is verified beforehand.
- Locked dependency implementation was consulted for the split/control design:
  futures-util 0.3.34 `src/stream/stream/split.rs` (empty sink slot, lock released
  on pending poll, reunite), tokio-tungstenite 0.30.0 `src/lib.rs` (poll-ready,
  start-send and flush), tungstenite 0.30.0 `src/protocol/mod.rs` (read queues
  automatic controls and continues reading when their flush would block).

## Executed focused checks

```sh
cargo check --locked --workspace --all-targets
cargo test --locked -p la-cal-client --test owp_peer
cargo test --locked -p la-cal-client --doc
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets
git diff --check
```

The focused peer run after review fixes passes **16 tests**, zero failed/ignored; doctests pass
**2 tests**, zero failed/ignored. Locked all-target workspace checking was run
throughout implementation and passes. Formatting, workspace Clippy and whitespace
checks pass after the corrections below. No full workspace test suite, deployed
server/router run, external harness run or independent review was performed by
this implementation session.

All new assertions observe the public typed client and a real loopback OWP peer.
Public futures are polled to establish admission before cancellation/close; peer
events establish transmission before testing sent cancellation. Short negative
peer-read windows test forbidden transmissions; outer deadlines bound scenarios.

| Focused check | Observed behavior / source linkage |
| --- | --- |
| `canceled_sent_publication_owns_its_response_while_reception_stays_live` | Abort a transmitted PUB caller; queue another PUB; receive MSG and pong before the first response. No next PUB/close/retry before the canceled PUB's distinct -ERR, and the next PUB receives its own +OK. Spec 102–105; Rev B MSG/ack syntax |
| `acknowledgment_expiry_never_sends_queued_operations_and_reports_uncertain_outcome` | Missing sent-PUB acknowledgment yields Deadline/transmitted=true; queued PUB and SUB yield the same cause/transmitted=false. Both existing streams receive one terminal error then end before peer cleanup. Later clone use retains the cause. A late +OK cannot release queued operations; the next client frame is Close. Spec 106–107 |
| `any_clone_close_stops_admissions_drains_operations_and_reports_drain_rejection` | Both success and exact rejection cases: close from another clone rejects new PUB/SUB, drains sent/queued operations, preserves MSG reception, completes the handshake and returns the same result to both close waiters. Ordinary rejection does not prevent the remaining admitted PUB from succeeding. Spec 97/108; Rev B shutdown and -ERR |
| `drain_expiry_still_closes_websocket_after_close_waiter_is_canceled` | A shorter drain deadline expires while PUB is pending. Canceling the original close waiter leaves shutdown active. Pending PUB is uncertain, queued PUB is unsent, stream terminal cause is visible, and the peer receives Close and completes its handshake despite drain failure. Spec 108 |
| `closure_timeout_is_visible_to_every_clone_and_live_stream` | Peer withholds Close reply: close finishes with a bounded closure Deadline, both live streams receive terminal notification then end, and later clone calls/close retain that same cause. Spec 95/97/108 |
| `close_deadline_also_bounds_an_in_progress_socket_write` | Real TCP peek establishes write transmission without draining the message. A 16 MiB typed label forces backpressure. Close finishes under a 500 ms test bound with 100 ms drain and closure waits rather than waiting for the two-second operation deadline; PUB reports attempted transmission. Oversized label is a transport-pressure fixture, not a schema-valid OMS specimen. Spec 108 |
| `canceled_backpressured_publication_keeps_message_and_control_reception_live` | TCP peek establishes that a 16 MiB PUB has started while the peer does not drain it. Abort its caller and queue another PUB. Valid subscribed MSG, Ping, then another valid MSG arrive; both MSGs must dispatch within 300 ms each before allowing the peer to read PUB. The peer then observes the identical automatic Pong, no next operation before the canceled PUB's distinct rejection, and successful acknowledgment of the queued PUB. Spec 103/105; ticket 18 line 11; RFC 6455 §§5.5.2–.3 |
| Extended `unexpected_loss_and_malformed_protocol_are_terminal_receive_failures` | Abrupt loss, malformed MSG, +OK with extra fields, incomplete/unknown -ERR and unknown control: two streams and clone calls share the terminal Transport/Protocol cause. Typed decode recovery remains covered separately. Spec 95/107; Rev B framing/ack syntax |
| Retained peer tests | Existing configuration/establishment/profile, exact PUB/SUB/UNSUB rejections, independent streams, payload decode recovery, clone Drop and last-owner/subscription cleanup checks continue to pass |

## Red/green and encountered defects

Each new scenario was executed as it was added; the final peer-file run includes
all retained and new checks. Individual red runs used the corresponding test-name
filter with `cargo test --locked -p la-cal-client --test owp_peer <name>`.

1. **Closure-only failure:** the first closure-timeout test failed because streams
   ended without any error. Keeping registrations until closure completes and
   latching its failure made the focused run pass.
2. **Prompt terminal notification:** the acknowledgment-expiry test initially
   failed its terminal-stream deadline during implementation: deferring all stream
   completion until peer cleanup was too late. Failed drains now notify/end streams
   immediately; only successful drains keep them until closure. Focused rerun passed.
3. **Hidden drain rejection:** the drain success/rejection test failed with
   `close() == Ok(())` for a rejected admitted publication. Retaining the first
   drain rejection while continuing admitted work made both cases pass.
4. **Write blocking close:** the pressure test failed its 500 ms close bound because
   the already-started write ignored shutdown. Observing the shutdown watch while
   retaining the write future and bounding it by the drain deadline initially
   made it pass. Review then exposed the missing independent read path in that
   nested write loop; the follow-up architecture below replaces that loop.
5. Code inspection found cleanup reused the already-expired drain deadline (and
   used operation_timeout for failure cleanup). Cleanup now has its own configured
   close_timeout; the drain-expiry peer regression passed when added. This is not
   claimed as a separately executed red result.
6. Intermediate Clippy runs rejected an unnecessarily held admission lock and
   similar `clone`/`close` test-local names. Releasing the lock after the state
   transition and renaming those future bindings resolved them; final Clippy passed.

The canceled-sent scenario passed when first introduced, confirming the existing
background ownership behavior rather than implying it was newly repaired.

## Independent review follow-up

The coordinator reports initial independent reviews complete: Spec found a **P2**
receive-liveness defect under pending-write backpressure; Standards found a **Low
possible Duplicated Code** smell in the two shutdown transitions. Both are
addressed here; completed rereview results are recorded below.

The new cancellation/backpressure regression was added before changing the
reviewed implementation and failed on the first MSG's 300 ms reception bound.
The peer had observed transmission with TCP peek and withheld PUB reads; this
reproduced the review finding at the public-client/real-peer seam.

### Architecture repair

`run` now splits the established WebSocket into its futures-util sink/stream
halves inside the same background task. One main select loop polls shutdown,
the minimum operation/drain deadline, writer flushing and incoming frames.
The nested pending-write loop is removed, naturally leaving **one** shutdown
transition and one owner of the operation/registration state.

An operation feeds the empty SplitSink slot, then flushes in that select loop;
feeding this empty slot does not await network I/O in the locked implementation.
`transmitted` starts false and becomes true when flushing attempts the queued
frame, rather than merely when a command is prepared. The next command is held
until both the current write/flush and its attributable response complete. A
response arriving before local flush completion is retained with that pending
operation, and a duplicate/unsolicited acknowledgment remains terminal Protocol.
The same operation/drain deadlines cover pending write and response ownership.

Reading Ping leaves Tungstenite's automatic Pong queued and requests writer
flushing; it never awaits that flush in the reader. Tungstenite's shared protocol
state continues reading MSG/control frames while writing is backpressured. No
manual duplicate Pong or independent WebSocket parser is introduced. Before
bounded cleanup, the original halves are reunited so partial-write bytes and
automatic controls remain with the original socket; an unattempted sink slot is
discarded. Existing deadline/terminal-failure and close checks still pass.

The first repaired run delivered both MSGs but then failed when the **test peer**
tried to drain the pressure PUB: the 16 MiB label plus OMS wrapper exceeds
Tungstenite's default 16 MiB frame cap (observed size 16,777,697 bytes). This one
peer now accepts up to 32 MiB frames; the 16 MiB fixture, reception deadlines,
ownership assertions and client configuration are retained. Subsequent focused
rerun passed, including the matching Pong and distinct PUB rejection. The final
16-test peer run, two doctests, locked all-target workspace check, formatting
and workspace Clippy pass. No full workspace test suite or commit was run.

### Classification-helper review follow-up

The coordinator reports Spec rereview has **zero findings**, confirming the P2
receive-liveness issue resolved. Standards rereview identified one remaining
**Low possible Duplicated Code** smell: establishment and established reception
repeated text parsing, unexpected closure, WebSocket error and nontext-frame
classification.

Extracted the minimal private `frame_to_operation` helper in `connection.rs` for
those shared classifications. Ping/Pong remain handled by their callers:
establishment awaits its socket flush within the connection deadline, while
established reception requests automatic-Pong flushing without awaiting it in
the reader and ignores Pong. Error categories, diagnostics and established
operation transmission attribution are preserved. No broader abstraction or
new tests were added for this behavior-preserving extraction.

Executed after extraction: `cargo test --locked -p la-cal-client --test owp_peer`
passes **16 tests**, zero failed/ignored; `cargo check --locked --workspace
--all-targets`, `cargo fmt --all -- --check` and `cargo clippy --locked --workspace
--all-targets` pass. The two-doctest pass above belongs to the preceding split-I/O
follow-up, not a new run after this extraction. No full suite or commit was run.

## Final coordinator verification

Independent Spec rereview confirmed the backpressure reception defect resolved
and reported **zero remaining findings**. Final Standards follow-up confirmed
both shutdown and frame-classification duplication resolved: **zero hard
breaches and zero remaining heuristic smells**. Review totals: **Standards 0;
Spec 0**.

After the final classification-helper extraction, the coordinator ran the full
workspace suite once on 2026-10-05:

```sh
ZENOHD="/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd" SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
```

**300 Rust tests passed, zero failed or ignored:** 6 baseline-client tests,
219 server tests (including the enabled full-schema parser check), 2 native
process tests, 16 public-client/peer tests, 55 OWP/baseline-binding tests and
2 client doctests. The native tests exercise the deployed typed client through
the real Zenoh/Protobuf adapter; the missing-router test emitted its expected
startup error and passed. Native router and Protobuf tool provenance remain
in [ASB delivery evidence](asb-delivery-evidence.md).

Ticket 18's checklist is resolved in the local issue tracker. Ticket 19 is the
next unblocked implementation slice.

## Coverage limits

- No ticket-19 server queue/router-loss work or demo applications are included.
  Existing server validation limitations and plain-WebSocket TLS exception remain
  as recorded in prior evidence. These focused client observations do not establish
  unqualified OWP/LA-CAL/WebSocket conformance, full XSD assessment or OMS lifecycle
  execution/compliance.
- The first terminal connection cause is preserved if subsequent cleanup also
  fails; close still waits only for bounded cleanup. A sent operation's failure
  carries attempted-transmission facts, while close/queued/later calls do not
  inherit those facts. Drop never promises acknowledged teardown.
