# 23: Retarget the external CAL harness and record practical reuse results

**What to build:** Gather practical external protocol evidence against the project-owned server using the existing LA-CAL harness, aligned to the selected profile and three-root surface, with a truthful record of executed checks and coverage limits.

**Blocked by:** 19 — Complete remaining server validation and bounded failure behavior.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md) and the [final practical test-reuse decision](08-conformance-acceptance-evidence.md#practical-sleet-test-reuse).

- [ ] Attempt acquisition of a concrete revision of the external Open Arsenal LA-CAL harness and record the actual revision used. Use its documented preflight, target URL and configuration route to target project-owned `la-cal` running natively on macOS.
- [ ] Retain UCI 002.5.0 inputs and align authorized identities, exact topics and fixtures with the project's three-root surface. Record configuration/fixture changes rather than adopting the upstream eleven-family SLEET configuration unchanged.
- [ ] Start from the exhaustive tier as the reference and try certification cases where adaptation is practical. Record the actual selected tier/cases, commands, configuration, OS/architecture, harness/source/schema versions and observed results.
- [ ] Each failed selected check is fixed or explicitly recorded as a justified adaptation/exclusion with its remaining coverage limit. Reconcile round-robin expectations with algorithm-independent group selection and queue-drop expectations with slow-connection closure.
- [ ] Checks used as ASB delivery evidence target the actual Protobuf/Zenoh path. Local routing checks establish membership/selection behavior only; tests against the reference SLEET checkout and historical upstream passes do not establish project behavior.
- [ ] If harness acquisition or adaptation is disproportionate, document the concrete attempted route, obstacle and missing coverage. Such a recorded obstacle satisfies this ticket's practical-reuse investigation outcome and need not block the first successful demonstration; unrun tests are never recorded as passing.
- [ ] Summarize relevant embedded Rust/generator tests carried over during preceding adaptations, actual project-owned results, known defects and exclusions. There is no all-tests migration requirement.
- [ ] Map executed requirement-sensitive checks to their actual OMS 2.5 LA-CAL Rev B section/certification identifiers and clearly identify unrun coverage, including group/protocol coverage where applicable, for the final acceptance record.
