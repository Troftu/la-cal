# Ticket 16: actual Zenoh/Protobuf delivery evidence

Executed 2026-10-05 on native macOS 26.6.2 (25G83), arm64,
`aarch64-apple-darwin`, from the repository root.

**Result:** the OWP exchange traverses the project-owned ASB adapter:
authorization, exact topic/root and direct Rust schema assessment, original-byte
Protobuf encoding, Zenoh publication, startup subscriber receipt, decoding,
inbound assessment and local subscription/group fan-out. OWP PUB never dispatches
directly to local recipients, and ASB ingress is not republished. Successful
verbose PUB acknowledgment follows adapter completion, independently of delivery.

## Sources and versions

- Starting revision: `dfff2847287e29a505cfb4dd15be8b5a0bc5f04f`.
- OMS 2.5 / OMSC-SPC-013 Rev B / OWP 1.0, WebSocket subprotocol `owp`.
- UCI 002.5.0 schema revision `73a286fc4b881d9f328dbd64148525606a92c33a`;
  schema and generator inputs remain those recorded in
  [provenance](../spec/002.5.0/PROVENANCE.md).
- SLEET adoption revision `e3c2d798a229766571ed7be1f5b28d2dcdb51868`.
- Rust `1.99.0 (b940084d7 2026-09-28)`; Cargo
  `1.99.0 (5f94df478 2026-08-27)`.
- Zenoh `1.10.1`, default features disabled, only `transport_tcp`; no unstable
  transport listeners. prost/prost-build `0.14.4`, upstream `libprotoc 36.2` on
  PATH. Existing Tokio `1.53.2` and tokio-tungstenite `0.30.0` remain selected.
  The combined resolved graph is retained in `Cargo.lock`.
- Native router: official `zenoh-1.10.1-aarch64-apple-darwin-standalone.zip`,
  `zenohd v1.10.1 built with rustc 1.97.1 (8bab26f4f 2026-07-14)`.
  Archive SHA-256 verified against the official release digest:
  `a27a177014fa81bc1bdb9b824373a2a8bce0e0a33c40e6c189d6fc9414b57b8a`.
  Executed binary:
  `/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd`.
- Release/dependency lookups: `cargo search zenoh/prost/prost-build`,
  `cargo info zenoh@1.10.1`, `cargo info zenohd`, and the official
  [Zenoh release](https://github.com/eclipse-zenoh/zenoh/releases/tag/1.10.1).
  Upstream [configuration](https://github.com/eclipse-zenoh/zenoh/blob/1.10.1/DEFAULT_CONFIG.json5)
  and released stable SessionInfo/publication sources ground the router-ZID and
  synchronous publication behavior. `protoc --version` confirmed the compiler
  already installed from the baseline's latest-upstream lookup.

## Executed checks

```sh
cargo test --locked -p la-cal --bin la-cal server::tests:: -- --nocapture
ZENOHD="/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd" cargo test --locked -p la-cal --test native_baseline -- --nocapture
cargo fmt --all
cargo clippy --locked --workspace --all-targets
cargo check --locked --workspace --all-targets
cargo fmt --all -- --check
git diff --check
ZENOHD="/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd" SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
```

All checks passed. The full suite passed **282 Rust tests**, zero failures or
ignored: 219 server/configuration/router/schema/validator/admin/telemetry checks,
six baseline-client checks, 55 OWP/generated-helper checks and two native process
tests. The 41 adopted socket checks now use the actual adapter with real Zenoh
sessions and isolated embedded loopback routers, on a shared multi-thread test
runtime. Pure routing tests still test only local membership/selection.

The independent Zenoh observer was added before implementation: after correcting
the test runtime to Zenoh's required multi-thread scheduler, the local-only server
failed its bounded exchange because no ASB sample arrived. It passed after the
adapter replaced local publication echo. Further ingress and startup/shutdown
checks passed. Typechecking caught an inferred unsized queued-frame type; Clippy
caught a needless test reference and an oversized startup function, resolved by
typing the queue and extracting stable-router observation. No failed check is
represented as a pass.

## Requirement-linked observations

The official OMS 2.5 OMSC-SPC-013 Rev B DOCX was consulted directly for PUB/SUB
delivery and group behavior. The envelope, exact keys, readiness and adapter
completion milestone are project ASB decisions, not a standardized UCI binding.

| Behavior | Source | Executed coverage |
| --- | --- | --- |
| Negotiation and topic-policy preservation | OMSC-SPC-013 Rev B §§5.1.1.1.3.1/.5, CERT LACAL-000007/-000011; project three-root profile | Actual binary negotiates `owp`/1.0, supplies expected identities, accepts all three topic subscriptions and rejects valid ServiceStatus on both other topics without breaking later use |
| Adapter publication completion and distinct delivery | Project spec ASB acknowledgment contract; same document §§5.1.1.1.3.2/.6/.7 | Real-schema ServiceStatus PUB yields one +OK and one original-byte MSG in either order; independent Zenoh session sees exact ServiceStatus key and decodes field 1 to the same bytes |
| Envelope/JSON ingress assessment | Project encoding and validation contract; pinned UCI 002.5.0; same document §6.1 / CERT LACAL-000006 | Native ASB injection rejects malformed Protobuf, empty/missing payload, invalid UTF-8/JSON, mismatched root, schema-invalid ServiceStatus and unsupported exact key; subsequent valid bytes with unknown field 2 reach OWP unchanged |
| Local subscriptions and listener retention | Project shared-subscriber contract; same document §§5.1.1.1.3.3–.4, CERT LACAL-000009/-000010 | Native UNSUB/resubscribe followed by independently injected ASB message succeeds; SUB/UNSUB never own Zenoh resources |
| Ordinary/repeated/self/group delivery and topic isolation | Same document §4.8 and §§5.1.1.1.3.3/.6 | Adopted socket tests pass through envelope/publish/subscriber/decode/assessment/fan-out, including all three exact keys with synthetic types; ordinary copies and one matching member per group remain isolated |
| Router readiness and bounded cleanup | Project spec startup/cleanup contract | Checked-in loopback router launched on isolated port; server observes two stable nonempty ZID snapshots before binding OWP. TCP listener without a valid router fails within configured 300 ms startup wait and never admits OWP. Successful process run receives SIGTERM and exits successfully after subscriber/session cleanup |

The native process test owns both children with unconditional kill/reap on failure.
OWP and Zenoh observations are installed before publication. Invalid-input checks
are followed by a valid sample from the same Zenoh publisher; the first OWP frame
must be that valid sample. The fixture is independently specified in
[`fixtures/service-status.json`](../fixtures/service-status.json), not generated
from an expected serialization.

## Limits and remaining work

- SDK-local delivery is permitted and likely used for publisher self-delivery.
  The separate observer also receives over the real router. This is not evidence
  of a mandatory physical router hop for each local delivery or multiple servers.
- Real-schema successful publication covers ServiceStatus only. Both command
  roots are subscribed to and used in mismatch checks; their successful socket
  exchanges use synthetic schemas and do not establish full UCI payload fidelity.
- Known temporal/whitespace validator and polymorphic generated-code defects
  remain as recorded in [baseline evidence](baseline-evidence.md). Direct Rust
  validation is retained; complete XSD correctness is not established.
- Blocking-work deadlines protect connection I/O and bound waits but cannot
  cancel a started worker or establish nonpublication. Startup, subscriber drain
  and session close each have bounded stages; runtime shutdown waits at most five
  seconds for blocking workers. A process owner still bounds a stalled child's
  lifetime. No hard-cancellation or publication-deadline injection was executed.
- Ticket 19 completes periodic detected-router-loss latching and the selected
  64-message outbound queue/close-on-overflow behavior. The current inherited
  1024-message queue still drops on overflow. Stable startup polling cannot prove
  detection of every outage; SDK reconnect behavior has not been accepted as
  recovery for a later failed run.
- Copied socket-fixture temporary-directory leaks and free-port races remain;
  synthetic tests now run native server/router fixtures independently of their
  client runtimes. Unexpected abort cleanup remains ticket 19 scope.
- No external CAL harness, typed final client, relayed STOP, Linux, TLS or
  complete OMS/OWP/WebSocket conformance claim follows from this ticket. Plain
  `ws` retains the explicit RFC 6455 §10.6 TLS-capability exception.

Launch instructions are in [README](../README.md); actual adaptations and terms
are in [NOTICE](../NOTICE). Reference submodules were not modified.

## Review

Independent Standards and Spec reviewers inspected the staged implementation
against the starting revision above, including new files, tests and evidence.
Standards reported zero documented-standard violations or material heuristic
smells. Spec reported zero missing/partial requirements, scope-creep findings or
incorrect implementations within ticket 16. Neither reviewer reran tests; the
executed results above are from the implementation session.
