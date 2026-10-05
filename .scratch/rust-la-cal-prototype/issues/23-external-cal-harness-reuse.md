# 23: Retarget the external CAL harness and record practical reuse results

**What to build:** Gather practical external protocol evidence against the project-owned server using the existing LA-CAL harness, aligned to the selected profile and three-root surface, with a truthful record of executed checks and coverage limits.

**Blocked by:** 19 — Complete remaining server validation and bounded failure behavior.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md) and the [final practical test-reuse decision](08-conformance-acceptance-evidence.md#practical-sleet-test-reuse).

- [x] Acquire and inspect a concrete revision of the external Open Arsenal LA-CAL harness: read-only reference `standards-and-references/la-cal-harness` at `cdf9a9428d5f5cddeee971b1fa55261fffcbcd09` (`v2026.09.01`). The [source assessment](../../../docs/la-cal-harness-assessment.md) records the available route and adaptation risks; acquisition is not an executed harness result.
- [ ] Use its documented preflight, target URL and configuration route to target project-owned `la-cal` running natively on macOS. Retain the actual execution revision and any working-copy changes separately from the inspected reference.
- [ ] Retain UCI 002.5.0 inputs and align authorized identities, exact topics and fixtures with the project's three-root surface. Record configuration/fixture changes rather than adopting the upstream eleven-family SLEET configuration unchanged.
- [ ] Start from the exhaustive tier as the reference and try certification cases where adaptation is practical. Record the actual selected tier/cases, commands, configuration, OS/architecture, harness/source/schema versions and observed results.
- [ ] Each failed selected check is fixed or explicitly recorded as a justified adaptation/exclusion with its remaining coverage limit. Review the [concrete source conflicts and assertion/report limits](../../../docs/la-cal-harness-assessment.md#executable-coverage-and-adaptation-risks); do not assume the external group tests require round-robin. Earlier round-robin/queue-drop concerns concern copied SLEET internal tests.
- [ ] Checks used as ASB delivery evidence target the actual Protobuf/Zenoh path. Local routing checks establish membership/selection behavior only; tests against the reference SLEET checkout and historical upstream passes do not establish project behavior.
- [ ] If harness adaptation is disproportionate, document the concrete attempted route, obstacle and missing coverage. Such a recorded obstacle satisfies this ticket's practical-reuse investigation outcome and need not block the first successful demonstration; unrun tests are never recorded as passing. Acquisition itself is no longer an unresolved obstacle.
- [ ] Summarize relevant embedded Rust/generator tests carried over during preceding adaptations, actual project-owned results, known defects and exclusions. There is no all-tests migration requirement.
- [ ] Map executed requirement-sensitive checks to their actual OMS 2.5 LA-CAL Rev B section/certification identifiers and clearly identify unrun coverage, including group/protocol coverage where applicable, for the final acceptance record.

## Comments

- 2026-10-05: Documentation-only source assessment requested before continuing implementation. Harness acquisition is complete; no dependencies were installed and no preflight/certification cases were run. Follow the [assessment's retargeting route and coverage caveats](../../../docs/la-cal-harness-assessment.md) during integration. This issue remains open; the approved demo-first acceptance scope is unchanged.
