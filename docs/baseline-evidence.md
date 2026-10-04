# Ticket 14: native SLEET adoption baseline evidence

Executed 2026-10-04 on macOS 26.6.2 (build 25G83), Darwin arm64,
`aarch64-apple-darwin`. Working directory: repository root.

**Result:** the project-owned native server negotiates OWP, validates and
delivers the selected `ServiceStatus` fixture through the copied local router.
Ticket 14 is complete. This is the source-adoption milestone; final relayed
STOP and Zenoh/Protobuf acceptance have not been executed.

## Sources and versions

| Input | Recorded selection |
| --- | --- |
| SLEET | `e3c2d798a229766571ed7be1f5b28d2dcdb51868`, tag `v2026.09.01` |
| OMS documents | 2.5; reference revision `726272bd0390982a759c91a9cf4e13b81c2b510b` |
| UCI schemas | 002.5.0; `73a286fc4b881d9f328dbd64148525606a92c33a` |
| Rust | `rustc 1.99.0 (b940084d7 2026-09-28)` |
| Cargo | `cargo 1.99.0 (5f94df478 2026-08-27)` |
| Python | 3.14.8; generator uses only the standard library |
| Generator | project-owned `scripts/generate_uci_types.py`, SLEET revision above plus provenance-header adaptation dated 2026-10-04 |
| Runtime dependencies | Tokio 1.53.2, tokio-tungstenite/Tungstenite 0.30.0, quick-xml 0.42.0, Serde 1.0.229, serde_json 1.0.151, TOML 1.1.6+spec-1.1.0 |
| Additional direct dependencies | futures-util 0.3.34, http-body-util 0.1.5, hyper 1.11.1, hyper-util 0.1.21, metrics 0.24.6, metrics-exporter-prometheus 0.18.3, metrics-process 2.4.3, regex 1.13.1, tokio-util 0.7.19, tracing 0.1.44, tracing-subscriber 0.3.23, UUID 1.27.0, tempfile 3.27.0 |
| Optional OTLP dependencies | opentelemetry / opentelemetry-otlp / opentelemetry_sdk 0.33.0, tracing-opentelemetry 0.34.0, tonic 0.14.6 |

Latest stable releases were rechecked at baseline establishment through the
official Rust stable manifest (2026-10-01 release) and the crates.io index
(`cargo search` plus fresh `cargo generate-lockfile`). `rust-toolchain.toml`
pins Rust 1.99.0; `Cargo.lock` records the resolved graph (238 packages).
The installed starting Rust/Cargo 1.91.0 was not used for project acceptance.
Current Zenoh 1.10.1 and prost/prost-build 0.14.4 were looked up for subsequent
work; they are not selected/build-tested adapter dependencies in this ticket.
Installed `protoc` was 36.2; this baseline does not invoke it. Ticket 16 must
recheck/select its compiler and adapter dependencies when introducing them.

## Executed commands and results

```sh
rustup toolchain install 1.99.0 --profile minimal --component rustfmt --component clippy
cargo generate-lockfile
python3 -B scripts/generate_uci_types.py --self-test
python3 -B scripts/generate_uci_types.py --generate-rust --output crates/owp-types/src/uci/v2_5.rs
cargo check --locked --workspace --all-targets
cargo test --locked -p la-cal schema::parser::tests::
SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked -p la-cal schema::parser::tests::verify_uci_v25_schema -- --nocapture
cargo test --locked -p la-cal --test native_baseline -- --nocapture
cargo fmt --all
cargo clippy --locked --workspace --all-targets
cargo check --locked -p la-cal --features otlp
SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
cargo build --locked --workspace
cargo fmt --all -- --check
git diff --check
```

All final commands passed. The initial check exposed seven quick-xml 0.42
byte-to-UTF-8 API compilation errors; these were fixed in the copied parser.
Initial Clippy reported inherited argument-count and new assertion-style
lints; two targeted handler annotations and one workspace style allowance
resolved them without rewriting router/validator behavior. The new native
test first exposed incorrect test assumptions about INFO fields (INFO has
`uuids.service`, not `service_uuid`, and no schema member); those were corrected
against the adopted protocol type. Its first executable run failed because
the fixture had not yet been added. After adding the schema-derived fixture,
the exchange passed. No failing checks are concealed as passes.
Staging exposed Git's host CRLF conversion and upstream documentation
whitespace. `.gitattributes` preserves the authoritative XSD/VDD bytes and
exempts those unmodified artifacts from whitespace style checks; authored
code/docs retain normal checks.

- **Rust:** 273 tests passed, zero failed/ignored: 211 server/configuration/
  routing/schema/validator/admin/telemetry tests, six baseline-client tests,
  55 protocol/generated-helper tests and one native process integration test.
  Zero doctests were present. The final full suite supplied `SLEET_SCHEMA_PATH`.
- **Generator:** all 15 copied default-root self-tests passed against the
  actual extracted files, including filesystem include loading. The baseline
  artifact was explicitly regenerated and successfully compiled.
- **Full-schema parser:** 722 global elements, 4,612 complex types, 945
  user-defined simple types, 70 abstract types, 8,169 facets and 926 compiled
  effective-facet entries; the copied facet-compilation checks passed. Counts
  and compilation do not establish complete schema validation correctness.
- **Native launch:** the integration test executes `target/debug/la-cal`
  with a temporary copy of `config/server.toml`, substituting only an ephemeral
  loopback port. Its working directory is the repository root, credentials
  absent, participant `baseline-tester`, topic `baseline`, root `ServiceStatus`.
  It checks the `owp` response header, INIT `+OK` followed by INFO version 1.0,
  server/system/Service identities, SUB `+OK`, PUB `+OK`, independent MSG with
  exact original JSON and SID, then UNSUB `+OK`. It accepts either order of PUB
  acknowledgment and delivery. The owned native server is killed and reaped
  after the test, including assertion-failure paths.
- **Feature build:** the retained optional OTLP feature compiles with updated
  stable dependencies; no exporter endpoint/runtime behavior was exercised.

## Requirement-sensitive coverage

The official OMSC-SPC-013 Rev B DOCX was consulted directly for the baseline
negotiation and acknowledgment behavior. Identifiers below are coverage
pointers, not declarations that every associated certification check passed.

| Observed behavior | Selected requirement source | Coverage |
| --- | --- | --- |
| WebSocket subprotocol and INIT/INFO negotiation | OMS 2.5 OMSC-SPC-013 Rev B §§5.1, 5.1.1.1.3.1/.5; CERT LACAL-000007/-000011 | Native process happy path plus copied socket/parser checks; no complete WebSocket campaign |
| PUB, SUB, MSG, UNSUB | Same document §§5.1.1.1.3.2–.4/.6; CERT LACAL-000008/-000010/-000012/-000014 | Native validated `ServiceStatus` exchange, copied protocol/socket cases |
| Group registration/filtering/cleanup | Same document §4.8 / CERT LACAL-000009 | Copied local-router group tests; synthetic-schema sockets, no real ASB groups |
| OMS JSON original-byte delivery | Same document §6.1 / CERT LACAL-000006; project fidelity choice | Pinned real-schema fixture survives PUB→local-router→MSG unchanged; no independent XSD assessment |
| `ServiceStatus` fixture shape | Pinned UCI XSD `ServiceStatusMT`, `ServiceStatusMDT`, `MessageType`, `HeaderType` and security include | Minimal status fixture accepted by adopted validator; no Service lifecycle behavior |
| TLS-capability exception | RFC 6455 §10.6 | Plain WebSocket only; TLS capability remains omitted |

## Known defects and exclusions

- Delivery is still the adopted **in-process router**, not Zenoh/Protobuf. The
  baseline queue is 1,024 messages and overflow drops are asserted by the
  unchanged upstream tests. Those assertions prove baseline behavior only;
  ticket 19 must replace them with the selected 64-message/close-on-overflow
  contract. The required ASB and final queue changes are not ticket-14 defects.
- The temporary `baseline-client` requires mutable access and cannot preserve
  MSG while waiting for a verbose operation acknowledgment. Final cloneable,
  independently receiving, cancellation-safe typed client work is tickets 17–18.
- Direct validator temporal lexical/calendar and whitespace-collapse gaps,
  namespace simplification, partial XSD support and generated polymorphic
  `$type` limitations remain. They did not block the selected valid fixture or
  copied tests. Passing this suite is not comprehensive three-root validation
  evidence. The baseline generator retains upstream default roots, not the
  final three application roots/metadata.
- Copied socket tests largely use a synthetic schema/local router and contain
  upstream readiness sleeps, temporary-directory leaks and port-selection
  races. Their historical structure is retained to identify the adoption delta.
  The new process test uses bounded connection readiness rather than an assumed
  startup delay. No wider test migration or failure-injection campaign was run.
- WebSocket close-handshake progression, stale queued deliveries after UNSUB,
  task-abort subscription cleanup and admin supervision have known upstream
  limitations. The native test observes exchange and UNSUB acknowledgment, not
  graceful server exit or full RFC closing behavior. It forcibly cleans up its
  owned server; later tickets supply bounded production shutdown.
- No external CAL harness, Linux run, real ASB, relayed STOP, process-lifecycle
  verdict or complete OMS Service/Platform/WebSocket/OWP conformance evidence
  is claimed. External harness reuse is ticket 23.

Reference submodule checkouts were not modified. Retained licenses and actual
modifications are recorded in [`../NOTICE`](../NOTICE); UCI provenance and
regeneration are in [`../spec/002.5.0/PROVENANCE.md`](../spec/002.5.0/PROVENANCE.md).

## Integrity and review

After staging, `git diff --cached --check` passed. `git hash-object --no-filters`
of all four extracted XSD/VDD files, `git rev-parse :<path>` for their staged
blobs and `git ls-tree` at the pinned upstream schema directory agreed:

| Artifact | Upstream, working-copy and staged Git blob |
| --- | --- |
| Main XSD | `a0c09ce4e9375401c0620a1d91bb3a87a2356e7b` |
| Security markings XSD | `8d54b9c90f3ff303d73ba527f5b7f19c85f7a408` |
| Versioning XSD | `cfaab3b73b4cf901af718e7ea751092702806397` |
| VDD | `d292e34860d03cbe0dd8dc298aa22ebbb0bd5ec5` |

The implementation review compared staged changes against starting commit
`4f038604f48ff97228a7499d6ffc4b896880f5ba`. Independent Standards and Spec
review agents reported zero actionable findings for ticket 14. They inspected
the actual upstream adaptation delta, provenance, test seam and recorded
results; they did not independently rerun the build/test suite.
