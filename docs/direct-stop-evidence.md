# Ticket 21: native direct Commandee STOP

Executed 2026-10-05 on native macOS **26.6.2 (25G83), arm64**, from ticket 20's
baseline `354194b`. This is direct STOP evidence, not relayed STOP acceptance.

## Behavior and source grounding

- Both Services are launched and owned by the driver. Its persistent typed result
  and Service Status observations are installed before application launch. Both
  INITIALIZING, NORMAL and later periodic NORMAL reports precede the command.
- The driver publishes NEW/STOP with a fresh CommandID, configured Mission and
  Commandee as both Effecting and sole Affected Service. Application code, not
  topic routing, decides eligibility. Commandee ignores other explicit processors;
  self-targeted requests with mismatched System/header or payload Mission, missing
  effecting identity, different/multiple affected identities or unsupported action
  or command state receive terminal REJECTED/FAIL and remain running.
- The minimal Commandee workload is an application-owned periodic unit counter.
  Exiting its work/reporting loop stops it before the final publications. There is
  no asynchronous worker left running, lifecycle runtime or generic handler layer.
  This is a deliberately trivial demo workload, not an operational mission function.
- Valid STOP sends one terminal ACCEPTED/SUCCESS with the original CommandID and
  Commandee processor identity, then final ServiceStatus(INOPERABLE) with no
  CommandID. Both sends are awaited before bounded shared-client closure and
  process exit. An error in either send or closure exits unsuccessfully and is
  never retried or covered by fabricated success/state.
- Payload helpers supply required unclassified SecurityInformation and fresh
  Timestamp/actual sender headers. The pinned UCI `HeaderType` has **no MessageID**;
  freshness does not introduce a non-schema field. CommandID is preserved in the
  result rather than replaced with a new command identity. Failure diagnostics use
  `INVALID_INPUT_PARAMETER`, a truthful Description and no optional AssociatedID.
- The driver separately checks command correlation/processor, final Service state
  and the owned process's successful exit. Required observations run concurrently
  with bounded waits starting after publication completion; unrelated messages
  do not reset deadlines. Buffered final messages may be consumed after the process
  exit event. Neither result/state arrival order is assumed, and exit cannot replace
  missing messaging. Infrastructure/Orchestrator exits and observer loss fail the run.
- Orchestrator ignores commands for other effecting Services and continues NORMAL
  reporting. The driver requires a NORMAL Timestamp newer than completion of all
  three STOP observations (same native host clock), rejecting stale/buffered reports
  as proof of continued reporting. It then performs bounded termination
  and reaping of remaining processes. It does not keep processes alive after verdict.

Grounding read from the **official OMS 2.5 OMSC-STD-001 Rev M §6.1.1,
Table 6.1-1**: NORMAL → INOPERABLE on STOP; INOPERABLE no longer performs mission
functions; final reporting is Service-contract-dependent. Reporting requirements
remain §7.2.1 RQMT STD-000319–000321 / -000872/-000873. These are selected behavior
links, not complete verification of those requirements or a delivered Service Contract.
The **pinned UCI 002.5.0 XSD** defines ServiceLifecycleCommandMDT (Mission, effecting,
affected and command fields), CommandProcessingStateEnum (ACCEPTED/REJECTED are
terminal), ServiceLifecycleCommandStatusMDT (processor/SUCCESS/FAIL), HeaderType,
CannotComplyType and ServiceStatusMDT. Provenance: `spec/002.5.0/PROVENANCE.md`.
No UCI 2.6 definitions are substituted. The profile remains **OMS 2.5 / UCI 002.5.0 /
OMSC-SPC-013 Rev B / OWP 1.0**. Strict direct-target policy, delayed terminal
response, final publication/closure/exit sequencing, driver process ownership and
Zenoh/Protobuf framing are approved project decisions, not prescribed UCI ASB semantics.

## Executed checks

Use the installed tool paths for this host:

```sh
export DEVELOPER_DIR=/Applications/Xcode-26.6.0.app/Contents/Developer
export ZENOHD=/private/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/zenoh-1.10.1/zenohd
cargo build --locked --workspace --bins --examples
target/debug/demo-driver --direct-stop
cargo test --locked -p demo-driver --test direct_stop
cargo test --locked -p demo-driver --test commandee_stop
cargo test --locked -p commandee-service --test final_publications
cargo check --locked --workspace --all-targets
```

These commands passed. The first direct driver test was red (unsupported CLI)
before implementation. Missing-state/result/exit and unsuccessful-exit fixture
checks each failed before their controlled fixture behavior was implemented, then
passed with the driver correctly refusing success. Production behavior is checked
at public process/message boundaries, never through private state/task introspection.

| Check | Observed result / limit |
| --- | --- |
| Actual applications through deployed Zenoh/Protobuf | Direct result, final state, successful owned exit and surviving Orchestrator reporting; all four child PIDs independently checked nonexistent after cleanup |
| Three separately omitted milestones | Missing result, state or exit each fails its deadline despite the other two; all remaining children reaped |
| State-first fixture | Final state published before result succeeds; fixture is not evidence of real Commandee implementation behavior |
| Failed-exit fixture | Nonzero child exit fails even after successful messages; no successful verdict |
| Stale-Orchestrator fixture | Old NORMAL timestamps after STOP cannot satisfy the post-STOP reporting wait; missing fresh reporting fails its own deadline |
| Real Commandee targeting/rejection | Other processor ignored, eight ineligible targeted requests rejected with schema-valid diagnostic, later NORMAL and then valid STOP/result/state/successful exit |
| Scripted OWP peer | Withheld result and state acknowledgments prevent later publication/close/exit; each final-send rejection is visible and exits unsuccessfully without retry. This supplies no ASB evidence |

The deployed tests traverse the server's existing only-ASB-ingress dispatch path:
OMS JSON → Protobuf envelope → actual Zenoh publish/subscriber → decode/validation
→ OWP MSG. That path and independent envelope evidence are recorded in
`docs/asb-delivery-evidence.md`; no direct PUB echo was added. SDK-local delivery
remains acceptable, and no forced physical router hop is claimed.

Standalone final run: `ws://127.0.0.1:49629/`, router `tcp/127.0.0.1:49628`, default
Mission ending `4020`, period 1 second and wait 5000 ms. Both periodic NORMAL
reports arrived at `2026-10-05T19:02:01Z`. Command
`2fdc69ad-40eb-42c7-a42f-19bda5679ba4` produced Commandee ACCEPTED/SUCCESS,
INOPERABLE and successful exit; Orchestrator subsequently reported NORMAL. Owned
PIDs 63962 (router), 63963 (server), 63964 (Orchestrator), 63965 (Commandee) were
reaped. Verdict: **PASS direct STOP**. Ports vary between runs.

Versions are unchanged from ticket 20: Rust/Cargo **1.99.0**, upstream
**libprotoc 36.2**, native **zenohd 1.10.1**, locked Zenoh **1.10.1**, prost/prost-build
**0.14.4**, Tokio **1.53.2**, tokio-tungstenite **0.30.0**, uuid **1.27.0** (v4 enabled
for fresh CommandIDs). Cargo.lock adds only existing dependency edges; there are
no generator, generated-binding, schema, adopted-source or reference-submodule changes.

Final checks also passed:

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets
SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
python3 -B scripts/generate_uci_types.py --self-test
git diff --cached --check
```

**336 Rust tests passed, zero failed/ignored**: 7 direct-driver checks, 1 deployed
Commandee targeting/STOP check, 2 final-publication peer checks, 7 readiness checks,
236 server checks (full-schema parsing enabled), 4 native ASB/server checks,
16 client/OWP-peer checks, 6 baseline-client checks, 55 OWP/binding checks and
2 client doctests. Focused passes overlap the full suite. The missing-router
startup-error output is expected negative-test evidence. Generator self-tests
passed **15/15**; generated artifacts remain unchanged.

Independent `/code-review` compared staged work with baseline `354194b`, before
the implementation commit existed. Initial Standards review: no documented
violations, one duplicated test-setup smell; Spec: no findings. Consolidated the
three missing-milestone assertions. An additional self-review reproduced stale
Orchestrator NORMAL incorrectly satisfying the post-STOP availability check with
a failing deployed test; added bounded timestamp freshness. Both reviewers
rechecked the updated staged diff: **final Standards 0, Spec 0 actionable findings**.
Reviewers did not rerun tests. The full workspace suite ran once after these repairs.

## Coverage limits

- Orchestrator relay, distinct A/B contexts and final prototype/harness acceptance
  remain tickets 22–24. No external CAL harness execution or Linux verification.
- No full Service/Platform, OWP/LA-CAL/WebSocket compliance claim. Plain `ws`
  retains the RFC 6455 §10.6 TLS-capability exception. This is loopback only.
- No recovery, retries, replay or duplicate suppression. Eligibility outside the
  implemented NORMAL direct-STOP path is not a full lifecycle-state test campaign.
- Minimal workload cannot fail independently; operational workload/drain failures
  are not simulated. Scripted OWP checks isolate final-send failure/ack timing;
  deployed fixture omissions exercise the driver, not application failure handling.
- Existing validator temporal/whitespace and polymorphic-codec limits remain as
  documented in earlier evidence. No complete three-root validation claim.
- Free-port binding races fail visibly. Uncatchable driver death cannot promise
  cleanup. Bounded observed process exit and publication acknowledgments do not
  establish durable or exactly-once delivery.
- Post-STOP report freshness uses the one-host UTC clock. Clock rollback can fail
  that bounded wait; no distributed-clock synchronization contract is established.
