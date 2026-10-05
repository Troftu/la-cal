# Ticket 17: typed Rust client evidence

Executed 2026-10-05 from clean baseline
`797b6011603987d6416515cccde5e6e875aba08b` on main, native macOS 26.6.2
(25G83), arm64 / `aarch64-apple-darwin`.

## Implemented API and behavior

`la-cal-client` provides the approved cloneable asynchronous public surface:

```rust
Client::connect(config: ClientConfig) -> Result<Client, ClientError>
Client::send<T: Message>(&self, message: &T) -> Result<(), ClientError>
Client::subscribe<T: Message>(&self) -> Result<Subscription<T>, ClientError>
Client::close(&self) -> Result<(), ClientError>
Subscription::<T>::close(self) -> Result<(), ClientError>
```

Only the three generated global roots implement the sealed Message trait.
Versioned bindings include full reachable value dependencies, SecurityInformation,
MessageHeader, OMS JSON wrapping, inferred topics and fixed 002.5.0 profile.
Derived field-position codecs retain `$type` and derived-only data instead of
silently truncating it. Checked-in Rust compiles without Python at build time.

One private background connection loop serializes acknowledgment-tracked operations,
retains pending-operation ownership independently of the caller future, and dispatches
MSG/control frames without application consumption. Every subscription owns an
independent registration and application-polled typed stream. Payload errors are
nonterminal; protocol errors and unexpected loss are terminal stream errors.
Server rejection codes/details are preserved for PUB/SUB/UNSUB. Transport, protocol
and deadline failures are machine-readable; attempted transmission and queued-unsent
failures remain distinct. Clone/subscription drop cleanup and explicit shared closure
use the approved ownership model.

## Sources and versions

- Official normative source consulted directly: OMS 2.5
  `standards-and-references/oms-standard/docs_official/20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.docx`.
- UCI 002.5.0 Main and SecurityMarkings inputs at
  `spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd` and
  `spec/002.5.0/UCI_SecurityMarkings_v2_5_0.xsd`, extracted from revision
  `73a286fc4b881d9f328dbd64148525606a92c33a`.
- SLEET generator/frontend adaptation source revision
  `e3c2d798a229766571ed7be1f5b28d2dcdb51868`.
- Rust `1.99.0 (b940084d7 2026-09-28)`, Cargo
  `1.99.0 (5f94df478 2026-08-27)`, Python `3.14.8`, protoc `36.2`.
- Locked dependencies: Tokio `1.53.2`, tokio-tungstenite `0.30.0`, futures-util
  `0.3.34`, serde `1.0.229`, serde_json `1.0.151`; existing server Zenoh
  `1.10.1`, prost/prost-build `0.14.4`. Cargo.lock retains the resolved graph.
- Native zenohd `1.10.1`, the verified release binary retained from ticket 16;
  executable path appears in the commands below. Router/source digest evidence
  remains in [ASB delivery evidence](asb-delivery-evidence.md).
- Generated input/generator hashes, terms and explicit commands:
  [UCI provenance](../spec/002.5.0/PROVENANCE.md). Standards submodules remain
  read-only. Actual source adaptations are recorded in [NOTICE](../NOTICE).

## Executed checks

```sh
cargo check --locked --workspace --all-targets
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets
cargo test --locked -p la-cal-client --test owp_peer
cargo test --locked -p la-cal-client --doc
cargo test --locked -p la-cal --bin la-cal server::tests:: -- --nocapture
ZENOHD="/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd" cargo test --locked -p la-cal --test native_baseline -- --nocapture
python3 -B scripts/generate_uci_types.py --self-test
python3 -B scripts/generate_uci_types.py --client --generate-rust --output /var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/ticket17-uci-002-5.rs
rustfmt --edition 2024 /var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/ticket17-uci-002-5.rs
git diff --no-index -- crates/la-cal-client/src/messages/uci_002_5.rs /var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/ticket17-uci-002-5.rs
git diff --check
```

Focused results: **9 public-client/peer tests**, **2 doctests**, **41 retained server
socket tests**, **2 native process tests**, and **15 retained generator self-tests**,
zero failures/ignored in the final selected runs. Regenerated and formatted client
artifact compares identically. All-target check, Clippy, formatting and whitespace
checks pass. The implementation subagent left the full workspace suite to the
coordinator; its final result is recorded below.

### Red/green and encountered failures

- The initial public typed-client test failed to compile before the API/bindings
  existed, then passed after implementation.
- Last-owner drop initially failed its bounded transport cleanup test because a
  closed watch channel starved command-channel completion; disabling that closed
  watch branch fixed the observed ownership/cleanup behavior.
- A representationally impossible deadline was initially not rejected as invalid
  configuration; validating nonzero representable deadlines fixed the test.
- Rejected SUB initially queued an unnecessary UNSUB through its drop guard. The
  peer test caught that extra operation; definitive failure now disarms the guard
  while cancellation retains best-effort cleanup ownership.
- The first native invocation without ZENOHD failed to locate zenohd. Selecting
  the existing verified native executable allowed the deployed run.
- Extending the ASB observer first exposed queued samples from earlier raw injection
  checks. A fresh subscriber isolates typed observations; per-root counts avoid
  imposing cross-message arrival order.
- Explicit typed close exposed the inherited server's unflushed automatic close
  reply (connection reset without closing handshake). A bounded flush repaired
  the selected deployed run; the retained 41 server socket checks still pass.
- Intermediate typechecking/Clippy findings (Send bounds, atomic API rename,
  lock-lifetime formatting, private-module visibility and callback error size)
  were addressed; no failed intermediate check is counted as a final pass.

## Requirement-linked coverage

| Behavior | Source | Observed coverage |
| --- | --- | --- |
| Connect readiness and fixed profile | Rev B §§5.1, 5.1.1.1.1.1/.3, .3.1/.5; CERT LACAL-000003/-000007/-000011 | Peer checks requested `owp`, INIT first, only OWP 1.0/UCI 002.5.0, verbose=true and participant. Wrong INFO version and missing INFO fail; a connection deadline bounds setup. Native server negotiation succeeds. INFO has no schema field in Table 5.1-5; INIT success accepts the requested schema |
| Typed publication and roots | Rev B §§5.1.1.1.3.2/.6/.7/.8 and §§6.1.1–.4; pinned XSD global ServiceLifecycleCommand/ServiceLifecycleCommandStatus/ServiceStatus declarations and dependencies | Peer and native fixtures preserve root/security/header/message fields. All three roots publish and independently arrive through decoded Zenoh field-1 envelopes. Send preserves negative acknowledgment diagnostics; it is separate from observed delivery |
| Concurrent operation attribution and live dispatch | Approved ticket 06 public client/serialization decisions; Rev B responses have no operation IDs | Peer holds the first PUB acknowledgment and verifies no second transmitted SUB; PUB receives its own detailed rejection and SUB succeeds. Interleaved MSG before SUB/PUB acknowledgment and ping/pong are handled while application streams remain independent |
| Independent subscriptions and cleanup | Rev B §§5.1.1.1.3.3/.4/.6 and connection shutdown; CERT LACAL-000009/-000010/-000012; ticket 06 ownership decisions | Distinct SIDs for repeated subscriptions; both receive independently. Explicit UNSUB leaves the other active; Drop sends cleanup, with a subsequent acknowledged operation as barrier. Clone drop leaves the connection usable; clone close closes shared use; last-owner cleanup progresses |
| Payload errors versus protocol/loss | Approved ticket 06 stream/error contract; Rev B textual framing and one operation per message | Invalid JSON, missing required data and wrong roots are nonterminal typed items followed by valid delivery. Malformed MSG, +OK with extra fields and unknown control are terminal protocol failures; abrupt peer loss yields terminal failure before None |
| Derived data fidelity | Rev B §6.1.2 `$type`; pinned CannotComplyType/UnallocatedReasonType/ID_Type/TaskPlanID_Type definitions | Peer preserves UnallocatedReasonType's SystemID and TaskPlanID_Type's Version=7 through typed reception/re-publication. Unrelated, abstract, unknown or missing tags and unknown derived fields yield nonterminal decoding errors. Native server/validator/adapter also accepts and preserves the same derived fixture |
| Only global roots publishable and explicit generation | Approved ticket 06 artifact/interface decisions | Compile-fail doctest rejects ServiceStatusMt as Message; example compiles. Three-root generation includes 306 complex/35 simple dependencies with no unresolved references, compiles from checked-in Rust, and matches explicit regeneration |

Fixtures are independently specified from the pinned XSD, not synthesized from
the generated codec's expected output. Command fixtures use NEW/STOP, explicit
Mission/effecting/affected IDs, and status fixtures use distinct processor identity
and correlated CommandID. Diagnostic fixtures use a valid Reason, Description and
legal associated derived ID. These checks are payload/transport evidence, not a
Service lifecycle execution or process-exit demonstration.

## Remaining scope and review handoff

- Ticket 18 retains the additional cancellation, missing acknowledgment and bounded
  drain/closure scenarios; the connection state machine is designed for them but
  this session does not claim that subsequent ticket's acceptance checks.
- The server's known temporal/security-whitespace validation limitations remain.
  Selected client polymorphic truncation is fixed; baseline owp-types bindings
  remain adoption support. Client Serde decoding does not perform XSD facet or
  occurrence assessment, and full three-root validation correctness is not claimed.
- SDK-local delivery is permitted; native external Zenoh observation does not
  establish a mandatory physical router hop on every self-delivery.
- Ticket 19 retains queue-overflow and detected-router-loss work. The relayed STOP
  apps, external harness, Linux and TLS evidence remain later scope. Plain ws retains
  the selected RFC 6455 §10.6 TLS-capability exception.
- The coordinator delegated implementation to this implementation subagent and
  ran separate independent Standards and Spec review agents. Initial Spec review
  reported zero findings; Standards reported zero hard violations and one P3
  possible Duplicated Code smell in the negotiation/runtime INFO guards. That
  smell is addressed by a small shared private INFO-validation predicate, retaining
  the distinct negotiation and runtime errors. Follow-up Standards review confirmed
  zero remaining findings. Final review totals: **Standards 0; Spec 0**.

Review follow-up verification on 2026-10-05: the existing `owp_peer` test file
passes all 9 tests after the predicate extraction. Locked all-target workspace
check, formatting check, workspace Clippy and `git diff --check` pass. No new
tests were added for this pure refactor, and the implementation subagent did not
run the full workspace suite.

## Final coordinator verification

After independent review and the INFO-predicate refactor, the coordinator ran the
full workspace suite once:

```sh
ZENOHD="/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd" SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
```

**293 Rust tests passed, zero failed or ignored:** 6 baseline-client tests,
219 server tests (including the enabled full-schema parser check), 2 native
process tests, 9 public-client/peer tests, 55 OWP/baseline-binding tests and
2 client doctests. The missing-router negative test emitted its expected startup
error and passed. Ticket 17 is resolved; ticket 18 is the next unblocked step.
