# Ticket 19: server validation and bounded failure evidence

Executed 2026-10-05 on native macOS **26.6.2 (25G83), arm64**, from baseline
`fd9722316ca56521f66cbf4248738a7facb0bc24`. Ticket 19 is resolved after subagent
implementation, independent Standards/Spec review and coordinator verification.

## Implemented behavior

- Both OWP PUB and decoded ASB ingress retain the shared direct Rust assessment,
  fixed three-root/topic policy and accepted original OMS JSON bytes. Invalid
  inbound samples are reported and discarded; ordinary unknown Protobuf fields
  neither invalidate valid payloads nor bypass assessment of invalid ones.
- Every server connection uses a **64-message nonblocking outbound queue**.
  Overflow logs failure, invalidates/removes every registration on that connection
  and interrupts its socket writes. Healthy recipients continue. No recovery,
  silent drop policy or application lifecycle transition is introduced.
- Registration-specific lifetime tokens invalidate stale queued delivery after
  UNSUB; a reused SID gets a new registration. Connection RAII guards remove all
  registrations and decrement active counts even when a connection task is
  aborted. Per-client cleanup retains the one startup ASB listener.
- Two matching, nonempty, sorted router-ZID snapshots establish startup readiness.
  Runtime snapshots default to **100 ms**. Loss/replacement of the startup set
  latches the first infrastructure failure, stops admission/readiness and ingress,
  closes connected clients and produces an unsuccessful run. Reconnecting the
  SDK or restoring the endpoint cannot revive that adapter/run. Publication checks
  failure before work and before returning success.
- Handshake, INIT and socket writes use the configured operation wait; WebSocket
  close/reply flushing and resource reclamation use the cleanup wait. Server drain
  is separately configured by the existing `shutdown_drain_seconds` (default 5).
  Shutdown interrupts a backpressured write without waiting for its operation
  deadline. Bind failure after adapter startup still cleans up acquired resources.
- Session open, subscriber declaration/undeclaration, router lookup/iteration,
  publication and session close construct/resolve synchronous SDK work off async
  threads. Resource ownership guards also defer potentially blocking SDK Drop,
  including outputs arriving after timeout/cancellation. Deadlines bound waits,
  not the lifetime of started workers. Cleanup does not indefinitely join them.
- Main supervision awaits server cleanup if the admin task fails first, bounds
  admin cleanup, and finishes bounded runtime shutdown before its exit code.

`asb_operation_timeout_ms`, `asb_startup_timeout_ms` and
`asb_cleanup_timeout_ms` each default to **5000**. The router period is
`asb_router_poll_interval_ms`, default **100**. All four allow **1–60000 ms**.
Startup readiness, subscriber acquisition and cleanup stages have separate waits.

## Profile, sources and versions

- **OMS 2.5 / UCI 002.5.0 / OWP 1.0 / OMSC-SPC-013 Rev B**, plain loopback
  WebSockets with `owp`. No UCI 2.6 substitution or new encoding profile.
- UCI source revision `73a286fc4b881d9f328dbd64148525606a92c33a`; pinned
  schema/terms and generated-artifact provenance remain in
  [UCI provenance](../spec/002.5.0/PROVENANCE.md).
- SLEET source revision `e3c2d798a229766571ed7be1f5b28d2dcdb51868`;
  adaptations and retained terms are recorded in [NOTICE](../NOTICE).
- Implementation agents read the official DOCX directly:
  `standards-and-references/oms-standard/docs_official/20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.docx`.
  Relevant sources: §4.8; §§5.1.1.1.1.3–.5; SUB/UNSUB/MSG/+OK/−ERR
  §§5.1.1.1.3.3–.4/.6–.8; Table 5.1-7;
  CERT LACAL-000009/-000010/-000012–000014. These ground ordinary/group delivery,
  acknowledgments, infrastructure failure and connection-owned disposal.
  Queue capacity/overflow, terminal router loss, wait bounds and the ASB binding
  are **project decisions**, specifically approved
  [spec](../.scratch/rust-la-cal-prototype/spec.md) §§ASB encoding/validation/routing
  and Server execution/connectivity/cleanup, and
  [ticket 19](../.scratch/rust-la-cal-prototype/issues/19-server-validation-bounded-failure.md).
- Rechecked tools: Rust `1.99.0 (b940084d7 2026-09-28)`, Cargo
  `1.99.0 (5f94df478 2026-08-27)`, upstream `libprotoc 36.2` on PATH.
  Native `zenohd v1.10.1 built with rustc 1.97.1 (8bab26f4f 2026-07-14)`;
  binary/release provenance remains in [ASB evidence](asb-delivery-evidence.md).
- Cargo.lock unchanged: Zenoh **1.10.1**, prost/prost-build **0.14.4**,
  Tokio **1.53.2**, tokio-tungstenite **0.30.0**. No dependency, generator or
  generated-binding changes. Reference submodules remain read-only/unmodified.

## Executed checks and observations

Implementation agents ran focused router, server/socket, ASB and configuration
modules and individual native cases throughout implementation and repair.
Coordinator final checks after the review fixes:

```sh
cargo check --locked --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets
ZENOHD="/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd" SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
git diff --check
```

**All passed. 319 Rust tests, zero failed or ignored:** 236 server-unit checks,
4 native process tests, 16 public-client/OWP-peer checks, 6 baseline-client checks,
55 OWP/baseline-binding checks and 2 client doctests. The full-schema parser test
was enabled by `SLEET_SCHEMA_PATH`. The missing-router test's startup-error output
is its expected successful negative result. Focused/module counts overlap this
suite and are not extra unique passes.

| Check | Observation and coverage |
| --- | --- |
| Native exchange/validation | All three typed roots and the retained polymorphic diagnostic traverse actual envelope encode/Zenoh/decode/assessment/delivery. Independent Zenoh observation verifies payloads; OWP PUB rejects mismatched roots and a deep invalid ServiceState enum while the connection remains usable. |
| Twelve native invalid ingress samples | Malformed Protobuf; missing and explicitly empty bytes; invalid UTF-8/JSON; schema-valid command root on status key; missing required fields; invalid enum; unsupported root/key; unknown field with invalid payload or without payload. Each is logged, then followed by distinct valid original bytes from the same real Zenoh publisher, preventing broken delivery from masquerading as rejection. |
| Native listener retention | Valid envelope with unknown field 2 forwards original bytes. UNSUB/resubscribe and closure with live registrations leave the shared listener usable by a replacement connection. |
| Native slow socket | A small receive window/unread peer and schema-valid whitespace-padded payloads create real TCP backpressure through real Zenoh ingress. Overflow is visible; failed socket closes, healthy socket continues and server stays alive. Focused implementation runs reached overflow after 69 publications; this is a host observation, not a fixed threshold contract. |
| Native sustained router loss | Two clients first receive real ingress. Kill/reap actual zenohd; observe idle client closure, restore endpoint, attempt PUB on remaining client: no +OK, bounded unsuccessful server exit, no resumed admission. |
| Router regressions (17 tests total) | Exact 64/65-delivery local capacity boundary; ordinary/group overflow closes only failed connection and removes all registrations; same-SID independence; stale queued registration invalidation; forced-owner-abort cleanup; surviving group/ordinary registrations. Local membership/queue evidence, not standalone ASB evidence. |
| Socket regressions (47 tests total) | Overflow-close/shared listener, forced abort/count cleanup, bounded stalled write/close, shutdown interrupting long write/unfinished upgrade, bind-failure readiness cleanup. Deterministic local bursts exercise the router seam; later survival publications traverse actual Zenoh with minimal synthetic schemas. Existing authorization/group/negotiation checks retained. |
| ASB regressions (6 tests total) | Actual embedded Zenoh loss/restoration retains failure; validation rejection is nonterminal; partial readiness failure permits a fresh run. Controlled blocking work proves wait expiry while worker remains blocked, responsive last-owner Drop and disposal of late-acquired resources. |
| Configuration regressions | Default/override bounds and invalid zero/out-of-range values for operation/startup/cleanup/poll settings. |

## Encountered failures and review repairs

- The deployed router-loss scenario initially failed its first client-closure
  deadline during parallel integration. Direct ASB cancellation propagation into
  frontend admission/connection writes and failure retention repaired it. Coordinator
  focused run passed, then the final full suite passed after all review fixes.
- Intermediate Clippy rejected `manual_let_else` in an ASB unit test; this was
  corrected, and final all-target workspace Clippy passes.
- Independent Spec review found **P1**: Zenoh 1.10.1 subscriber undeclaration
  resolves synchronously in `IntoFuture`, before a timeout can protect it. The
  repair audited SDK startup, observation, cleanup and resource destructors and
  moved construction/resolution/ownership disposal to bounded worker waits.
  No forced private-SDK lock-stall test is claimed. New controlled blocking
  regressions verify the worker/ownership boundary; six ASB checks passed.
- Pinned dependency audit covered Zenoh `api/builders/{session,subscriber,info,close}.rs`,
  `api/subscriber.rs`, `api/session.rs`, `net/runtime/mod.rs` and zenoh-runtime
  `src/lib.rs`: several builders use `ready(self.wait())`, and raw Drop can
  synchronously undeclare/close. Offload covers router iterator traversal as well.
- Independent Standards review found **Low possible Duplicated Code** in UNSUB
  and whole-connection finalization. A minimal shared helper now owns token
  invalidation, metrics and direct/group index removal.

Both reviewers rereviewed the resulting combined diff against the baseline.
**Standards: 0 hard violations, 0 remaining actionable smells. Spec: 0 remaining
findings; original P1 resolved.** Reviewers inspected source without rerunning
tests; final execution results above are coordinator verification.

## Retained defects and coverage limits

- Direct Rust temporal lexical/calendar and security-whitespace-collapse defects,
  partial XSD/namespace support and unassessed generator/codec constructs remain
  recorded in [verified research](../.scratch/rust-la-cal-prototype/research/lifecycle-bindings-validation.md)
  and [baseline evidence](baseline-evidence.md). Ticket 17 repaired the exercised
  client derived-codec loss; its typed fixture still passes here. Broader baseline
  generated polymorphic-codec limitations remain outside this slice. No production
  validator replacement, independent XSD assessment or comprehensive remediation.
- Snapshots cannot detect every between-poll outage; the underlying transport's
  loss-detection latency has no guaranteed bound. The executed loss case is a
  sustained killed-router outage, not proof of every outage or reconnection race.
- Started blocking work and SDK resource destructors are noncancellable. Timeout
  does not prove nonpublication or completed disposal; detached fallback destruction
  is never joined indefinitely. Process ownership/termination remains the final
  lifetime bound. Controlled seam tests do not force contention in SDK internals.
- Slow-socket evidence is native-host-specific; queue capacity is separately
  deterministic at the local seam. Free-port selection races remain in fixtures.
  Delivery is live, nondurable and may use SDK-local routing; no mandatory physical
  router-hop, exactly-once or multi-server claim.
- No external CAL harness, relayed STOP applications, Linux execution or full
  OMS Service/Platform/OWP/WebSocket conformance evidence. Plain `ws` retains the
  selected RFC 6455 §10.6 TLS-capability exception. Ticket 20 is the next slice.
