# Ticket 15: three-root topic-routing evidence

Executed 2026-10-04 on native macOS 26.6.2 (25G83), Darwin arm64,
`aarch64-apple-darwin`, from the repository root.

**Result:** configuration and OWP PUB/SUB now enforce exactly
`ServiceLifecycleCommand`, `ServiceLifecycleCommandStatus` and `ServiceStatus`,
each paired with its identically named root. Optional bindings cannot widen
this surface. Ordinary and grouped delivery continue through the adopted local
router. Ticket 15 is complete; ticket 16 introduces the real ASB path.

## Inputs and executed checks

- Starting revision: `43ed79b95fdf6c85abc7fc5b811e294ed3fca04f`.
- OMS 2.5 / OMSC-SPC-013 Rev B / OWP 1.0, subprotocol `owp`.
- UCI 002.5.0 inputs from `73a286fc4b881d9f328dbd64148525606a92c33a`.
- SLEET source revision: `e3c2d798a229766571ed7be1f5b28d2dcdb51868`.
- Rust: `rustc 1.99.0 (b940084d7 2026-09-28)`; Cargo:
  `1.99.0 (5f94df478 2026-08-27)`.
- Dependency lockfile, generator, generated bindings and schema artifacts are
  unchanged from [ticket 14](baseline-evidence.md#sources-and-versions).

```sh
cargo test --locked -p la-cal config::tests
cargo test --locked -p la-cal server::tests
cargo test --locked -p la-cal router::tests
cargo test --locked -p la-cal --test native_baseline -- --nocapture
cargo fmt --all
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets
SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
cargo fmt --all -- --check
git diff --check
```

Final checks passed: **281 Rust tests**, zero failures/ignored, comprising
219 server/configuration/router/schema/validator/admin/telemetry tests, six
baseline-client tests, 55 protocol/generated-helper tests and one native
process integration test. There are no doctests. Focused selections passed:
55 configuration checks, 41 socket checks and 14 local-router checks.

The new unsupported-topic test first failed because `mission` remained
configurable. The adapted absent-binding socket test first failed because a
mismatched root received `+OK`. Both passed after policy enforcement. Fixture
migration exposed one remaining baseline-client assertion expecting `Msg`;
it was updated to the explicit selected root. Clippy found missing documentation
backticks and an oversized native-test function after adding policy checks;
the comment was corrected and the policy exchange extracted into a test helper.
No failed checks are represented as passes.

## Requirement-linked observations

The official OMS 2.5 OMSC-SPC-013 Rev B DOCX was read directly for PUB, SUB,
UNSUB errors and subscription-group behavior. The three-topic restriction
is a project profile decision, not an OWP-defined list of topics.

| Observed behavior | Source | Executed coverage |
| --- | --- | --- |
| Fixed topic/root pairing and participant permissions | Project spec §§ASB encoding, validation and routing; pinned UCI roots | Reject aliases/unsupported configured topics, widened/mismatched bindings and UCI 2.6 configuration; PUB/SUB reject unsupported roots and mismatches even without bindings |
| PUB rejection, authorization and continued use | OMSC-SPC-013 Rev B §5.1.1.1.3.2, CERT LACAL-000008; project Invalid-Message policy | Synthetic-schema socket checks distinguish unauthorized topic Illegal-State from invalid payload/pair Invalid-Message; errors have details and remain visible in non-verbose mode; later valid operations succeed |
| SUB and UNSUB registration/error behavior | Same document §§5.1.1.1.3.3–.4, CERT LACAL-000009/-000010 | Authorization, unknown/unsupported roots, mismatches, duplicate SIDs, rejected-request SID reuse, explicit unsubscribe and same-SID independence across connections |
| Ordinary/self/repeated delivery and topic isolation | Same document §5.1.1.1.3.6, CERT LACAL-000012 | Two self-registrations receive independently; closing one registration preserves the other; publications do not leak into subscriptions for other selected topics |
| Topic-scoped independent groups plus ordinary fan-out | Same document §4.8 / CERT LACAL-000009 | Socket checks cover all three topic names, one member of the matching group, independent groups and ordinary copies; local-router test checks each publication has one copy without requiring balanced selection |
| Connection-owned cleanup | Same document §5.1.1.1.3.8, CERT LACAL-000014 | Local-router ordinary/group cleanup, including same-SID/group survivors on another connection; this is local routing evidence |
| Negotiation, actual schema and original bytes | Same document §§5.1.1.1.3.1/.5, §6.1; CERT LACAL-000007/-000011/-000006; project byte fidelity | Native server with real pinned schema accepts subscriptions to all three roots, rejects the valid ServiceStatus fixture on both other topics, then acknowledges and delivers it unchanged on ServiceStatus |

The native process test launches the actual project-owned binary on an ephemeral
loopback port using `config/server.toml` and participant `baseline-tester`.
Its permissions cover all three topics; only ServiceStatus has an optional
explicit binding, so both other-topic rejections exercise absent bindings.
Observers are registered before publication. The test distinguishes PUB `+OK`
from MSG, accepts either arrival order and verifies the original payload and SID.
Readiness/exchange are bounded and the owned server is killed and reaped.

## Limits and retained behavior

- Socket policy/routing fixtures use small synthetic types under the selected
  root names. Their empty status and `Id`-only command objects are not valid
  complete UCI messages. Only the native ServiceStatus exchange supplies
  real-schema message-validation evidence; the other native roots are subscribed
  to, not published with complete valid payloads.
- Local-router tests retain generic routing/filtering fixtures where useful.
  Cleanup tests exercise the public router operations, not complete WebSocket
  close/abort cleanup. No new external CAL harness or full conformance campaign
  was run.
- The inherited permissive absent-binding PUB assertion was replaced with
  mismatch rejection. The balanced round-robin oracle was replaced with the
  approved one-member-per-group oracle. Round-robin implementation is retained.
- Production delivery/queue behavior remains the local router and inherited
  1,024-entry outbound queue with overflow drops. The selected Zenoh/Protobuf
  path and 64-message close-on-overflow behavior are subsequent tickets.
- The validator's temporal/whitespace and generated polymorphic-codec defects,
  temporary baseline-client limitations, copied socket-fixture leaks/readiness
  polling/port races and upstream shutdown limitations remain as recorded in
  [ticket 14](baseline-evidence.md#known-defects-and-exclusions).
- Plain `ws` retains the RFC 6455 §10.6 TLS-capability exception. No Linux,
  relayed STOP, Service lifecycle, full validation or OMS compliance claim is made.

Source modifications and preserved terms are in [`NOTICE`](../NOTICE).
Reference submodules were not changed.

## Review

Independent Standards and Spec reviews compared the staged changes against
the starting revision above. Standards identified two duplication heuristics:
the PUB/SUB pairing predicate and two unordered acknowledgment/delivery test
loops. These were consolidated into `ServiceConfig::allows_message` and
`expect_publication_delivery`; follow-up review confirmed both findings resolved.
Spec reported zero gaps and confirmed the refactoring preserves ticket behavior.
The reviewers did not rerun tests. After the refactoring, all 281 tests,
all-target typechecking/Clippy and formatting checks passed again.
