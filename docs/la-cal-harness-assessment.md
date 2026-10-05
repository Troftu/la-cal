# External LA-CAL harness: reuse assessment

Source and environment-metadata inspection on 2026-10-05, before integration. **No preflight,
certification suite or harness dependency installation was performed.** This
assessment updates the implementation handoff, not the approved acceptance gate.

## Baseline and authority

- Read-only reference: `standards-and-references/la-cal-harness`, revision
  `cdf9a9428d5f5cddeee971b1fa55261fffcbcd09`, tag `v2026.09.01`.
- Upstream: <https://gitlab.com/open-arsenal/ams-gra/hello-world-sk/test/la-cal-harness>.
- Harness [package metadata](../standards-and-references/la-cal-harness/pyproject.toml):
  `0.1.0`; Python `>=3.12`. Use the revision, not the
  package version or mutable `latest` image, to identify the inspected source.
- Its [schema sourcing contract](../standards-and-references/la-cal-harness/docs/schema-sourcing.md)
  and Makefile/Containerfile pin UCI `002.5.0` at
  `73a286fc4b881d9f328dbd64148525606a92c33a`, matching the project's
  [schema provenance](../spec/002.5.0/PROVENANCE.md). The separate UCI reference
  submodule remains at 2.6 and is not the input for this effort.
- Normative authority remains OMS 2.5 **OMSC-SPC-013 Rev B**, in the
  [official DOCX](../standards-and-references/oms-standard/docs_official/20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.docx).
  The harness is executable test evidence, not a replacement
  specification or proof of complete conformance.

The existing [acceptance decision](../.scratch/rust-la-cal-prototype/issues/08-conformance-acceptance-evidence.md#answer)
still governs: practical external CAL test reuse alongside a native macOS
relayed STOP demonstration, with actual results and coverage limits. Harness
source acquisition is now satisfied; native execution and adaptation remain
unverified. Historical SLEET passes do not transfer to this project.

## Retargeting route

The upstream [README](../standards-and-references/la-cal-harness/README.md),
[configuration contract](../standards-and-references/la-cal-harness/docs/configuration.md),
[config loader](../standards-and-references/la-cal-harness/src/la_cal_harness/config.py)
and [preflight implementation](../standards-and-references/la-cal-harness/src/la_cal_harness/preflight.py)
provide a native Python route. Prefer that
route for the selected loopback-only native macOS target: container loopback or
Linux-style host networking must not be assumed to reach the host server.
There is no need to expose the server off loopback to perform this assessment.

Integration should use a disposable harness working copy and isolated Python
environment, with reports outside the reference submodule. Upstream
[Make targets](../standards-and-references/la-cal-harness/Makefile)
install editable packages and write reports/build metadata; `make fetch-spec`
also writes into the checkout and uses a fixed `/tmp/uci-clone`. Reuse the
project's already-pinned main XSD and security include instead of fetching them
again or modifying the read-only reference.

An illustrative harness YAML follows; **this file has not been installed or
validated**. Replace the XSD placeholder with an absolute path before use.

```yaml
url: ws://127.0.0.1:9000/
owp_versions: ["1.0"]
schema_version: "002.5.0"
services:
  - service_id: baseline-tester
topics:
  - name: ServiceStatus
    message_type: ServiceStatus
  - name: ServiceLifecycleCommand
    message_type: ServiceLifecycleCommand
  - name: ServiceLifecycleCommandStatus
    message_type: ServiceLifecycleCommandStatus
xsd_path: /absolute/project/path/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd
settle_time: 0.5
timeout: 5.0
```

The configured service identifier is an OWP participant authorization identity,
not a new Service lifecycle implementation or authentication secret. The
project's `config/services.d/baseline.toml` already authorizes `baseline-tester`
for all three exact topic/root pairs. Upstream requires at least one identity
and two distinct topics, so the three-root surface fits the configuration model.
The first service/topic are the defaults for many tests; putting `ServiceStatus`
first does not establish coverage of all three roots.

Paths in the [config loader](../standards-and-references/la-cal-harness/src/la_cal_harness/config.py)
are interpreted relative to the process working
directory, not the YAML directory. `LACAL_HARNESS_XSD_PATH` overrides the YAML
schema path; `LACAL_TIMEOUT` overrides its timeout. Python compatibility of the
pinned [dependencies](../standards-and-references/la-cal-harness/requirements.txt)
(notably `lxml==5.4.0` and `pydantic-core==2.46.3`) on the
actual macOS interpreter must be checked, not inferred from `requires-python`.

Host metadata inspection found Python `3.14.8`, with `pydantic==2.13.5` and
`pydantic-core==2.46.5`, rather than the harness pins. The inspected interpreter
has no installed pytest, pytest-asyncio, websockets, lxml, PyYAML, xmlschema,
exrex or junitparser distributions. It is not a prepared harness environment;
this observation does not prove that installing the pins will succeed or fail.

After preparing that working copy/environment, configuration and running native
Zenoh router/server, the documented entry points are:

```sh
# Run from the disposable harness working copy; use absolute config/report paths.
python -m la_cal_harness.preflight --config "$LACAL_CONFIG" --url "$LACAL_TARGET_URL"
python -m pytest tests/cert/ --validation-tier exhaustive \
  --junitxml="$REPORT_DIR/junit.xml" \
  --validation-report="$REPORT_DIR/validation.json" \
  --validation-failures="$REPORT_DIR/validation-failures.json"
python -m la_cal_harness.report "$REPORT_DIR/junit.xml" "$REPORT_DIR/compliance.md"
```

Export `LACAL_CONFIG`, **a nonempty `LACAL_TARGET_URL`**, and `REPORT_DIR` before
these commands. The URL flag on standalone preflight does not configure a
later pytest process. These are future entry points, not executed commands or
a claim that the unmodified suite passes.

## Executable coverage and adaptation risks

All paths below are inside the pinned harness reference. Source inspection found
70 test functions under `tests/cert/`, including four unmarked connection-lifecycle
tests; one parametrization gives 72 source-derived cases before selection/skips.
This is an inventory, not a collected or executed test count.

| Source | What the assertions exercise |
| --- | --- |
| `test_connection.py`, `test_websocket.py` | WebSocket establishment/subprotocol, rejected upgrade, text INIT and client-initiated close; markers LACAL-000001–000005 |
| `test_handshake.py` | INIT variants/rejections, INFO fields/UUIDs and verbose +OK ordering; -000007/-000011/-000013 |
| `test_publish.py`, `test_message_validation.py` | PUB acceptance/errors and schema-derived mutation cases; -000006/-000008 |
| `test_subscribe.py` | SUB/UNSUB errors, SID reuse, ordinary/group/topic delivery and MSG payloads; -000009/-000010/-000012 |
| `test_verbose.py`, `test_errors.py` | Acknowledgments, bounded nonverbose silence, negotiation/operation errors; -000007/-000013/-000014 |
| `test_receive_validation.py` | Invalid OWP PUB not forwarded, valid PUB delivered with parsed-JSON equality; -000006/-000012 |
| `test_lifecycle.py` | Connection/group cleanup, topic isolation, same-participant UUID consistency and recovery after a nonfatal OWP error; no CERT markers |

These are harness marker assignments, not full normative coverage of those IDs.
In particular, `test_lifecycle.py` exercises **connection lifecycle**, not OMS
Service state transitions or the relayed STOP interactions.

### Target and fixture selection

[`tests/conftest.py`](../standards-and-references/la-cal-harness/tests/conftest.py)
selects real-server mode only with a **nonempty `LACAL_TARGET_URL`**, and runs
an autouse session preflight which exits pytest on failure. Otherwise it starts
its own mock, even if the YAML contains the real server's URL. Generator fixtures
also switch on that environment variable: mock mode uses the synthetic schema,
which has none of the project's three roots. An empty URL is unsafe too: the
mutation test's `is None` check differs from fixture truthiness checks.

The mock compares message shapes to the harness's own generated examples rather
than independently validating XSD facets. Mock passes are neither project-server
results nor an independent validation oracle. Setting real mode and retaining
target/config/schema provenance is essential to interpreting any run.

### Concrete assertion conflict

[`test_pub_wrong_message_type_for_topic_returns_illegal_state`](../standards-and-references/la-cal-harness/tests/cert/test_publish.py)
(lines 87–105) expects `Illegal-State` for publishing the second configured root
on the first topic. The project's [ASB assessment/error mapping](../crates/la-cal/src/asb.rs)
returns `Invalid-Message` for this root/topic mismatch, and
[`native_baseline.rs`](../crates/la-cal/tests/native_baseline.rs) explicitly
checks that response. Configuration alignment alone will not reconcile them.

During integration, ground this case in the selected Rev B PUB/error behavior
and project topic/root policy, then record the justified assertion adaptation
or observed failure. Source inspection does not authorize changing the server's
contract or excluding the case simply to obtain a green report.

### Generated messages and tier depth

The [generator](../standards-and-references/la-cal-harness/src/la_cal_harness/oms_json/generator.py)
uses heuristic values, first-choice/default concrete types and randomized UUIDs.
Its [constraint helpers](../standards-and-references/la-cal-harness/src/la_cal_harness/oms_json/constraints.py)
can return a pattern fallback without a final validity check. Treat a rejected
"valid" generated payload as a fixture-or-server investigation, not automatically
as proof of a server defect. Capture the exact payload and schema path involved.

The [variant factory](../standards-and-references/la-cal-harness/src/la_cal_harness/oms_json/variants.py)
applies smoke/standard/exhaustive depth to format-enforcement mutation cases;
it does not expand every protocol test into exhaustive coverage. Smoke has two
positive baselines and six basic negative variants. Standard adds selected
choice/facet/array/type mutations; exhaustive adds deeper/per-path mutations.
The [mutators](../standards-and-references/la-cal-harness/src/la_cal_harness/oms_json/mutators.py)
still have important limits: basic "required-field" removal removes the first
member without checking requiredness; synthetic multiple-choice keys may test
unknown-member rejection rather than real choice exclusivity; array paths target
the first entry; extracted exclusive bounds and exact length do not receive a
complete corresponding negative corpus. Exhaustive is not all XSD constraints,
all valid values or comprehensive correctness of the three-root validator.

### Group, receive and reporting limits

- The inspected external group tests do **not** require round-robin, fairness
  or balanced distribution. Their checks are weaker than complete group evidence:
  the one-publication test collects at most one MSG per member, and topic-scoping
  checks count messages without checking every payload/SID. The cleanup test
  closes the third member, which need not have been the selected recipient.
  See [`test_subscribe.py`](../standards-and-references/la-cal-harness/tests/cert/test_subscribe.py)
  and [`test_lifecycle.py`](../standards-and-references/la-cal-harness/tests/cert/test_lifecycle.py).
  Earlier round-robin/queue-drop adaptation concerns concern copied SLEET internal
  tests; they are not established requirements of this external suite.
- [`test_receive_validation.py`](../standards-and-references/la-cal-harness/tests/cert/test_receive_validation.py)
  publishes through OWP; it does not inject malformed Zenoh ingress. Its equality
  assertion compares parsed JSON, not original bytes, and its invalid-input
  absence check lacks a subsequent valid subscriber-progress witness. Retain
  the project's separate real-ASB/envelope/fidelity evidence.
- [`test_message_validation.py`](../standards-and-references/la-cal-harness/tests/cert/test_message_validation.py)
  records selected mock-only constraint skips as `actual="skip", passed=True`.
  The [JSON collector](../standards-and-references/la-cal-harness/src/la_cal_harness/validation_report.py)
  counts those as passes and omits them from failures-only output. Read the full
  validation report, not just its passed count or failures file.
- The [CERT report](../standards-and-references/la-cal-harness/src/la_cal_harness/report.py)
  retains one worst-ranked marked testcase per CERT ID. Ordinary pytest skips
  outrank passes, but one selected passing case can yield CERT PASS with sibling
  cases deselected: there is no expected-inventory completeness check. Unmarked
  connection-lifecycle tests do not enter this matrix, and internal variant skips
  can leave the parent testcase passing. The report also lacks target/mode,
  tier, schema hash and complete sibling-result provenance. Retain full JUnit,
  JSON and session results with an explicit run manifest; CERT PASS alone is not
  a conformance verdict.

## Claim boundaries and implementation handoff

- Standalone preflight checks configuration, WebSocket connection and verbose
  INIT/INFO. It does not exercise PUB/SUB, the Zenoh adapter or Service readiness.
- The harness operates at the OWP boundary. Delivery tests targeting the actual
  project server can complement existing real-ASB evidence, but the harness
  cannot independently decode the Protobuf envelope or prove router traversal.
- The official Rev B DOCX was checked for the operation identifiers:
  INIT `LACAL-000007`, PUB `-000008`, SUB/groups `-000009`, UNSUB `-000010`,
  INFO `-000011`, MSG `-000012`, verbose +OK `-000013`, and -ERR `-000014`.
  OMS JSON format enforcement is `-000006` / §6.1. Group semantics in §4.8
  require one matching member per topic-scoped group, not round-robin selection.
- Record actual selected cases, generated variants, skips, failures, fixture or
  assertion adaptations and exclusions. An exhaustive-tier label or a report
  row is not evidence that every relevant requirement or root was exercised.
- Integration remains with [Retarget the external CAL harness and record practical reuse results](../.scratch/rust-la-cal-prototype/issues/23-external-cal-harness-reuse.md).
  The [native acceptance handoff](../.scratch/rust-la-cal-prototype/issues/24-native-macos-acceptance.md)
  must consolidate its actual results rather than treating this assessment as
  a passed suite. Source availability alone does not complete either issue.

The scoped completion claim, macOS-only milestone, direct Rust validator and
explicit TLS-capability exception remain unchanged. No new lifecycle features,
message roots, broad conformance gate or production validator replacement are
selected by this source inspection.
