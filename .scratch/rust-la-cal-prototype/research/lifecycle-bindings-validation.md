# Selected lifecycle bindings and validation: verified findings

Date: 2026-10-04
Context: [Verify the selected lifecycle bindings and validation coverage](../issues/13-lifecycle-bindings-validation.md)
Status: executed scoped research; repairs and final application behavior are not implemented

## Summary

SLEET's selected three-root generation and conservative derived-value closure both compile on the available macOS arm64/Rust 1.91.0 host. The existing direct validator is a viable scoped repair starting point, but representative checks expose real temporal/whitespace defects and generated-code loss of polymorphic data. Installed xmllint/libxml2 supplies independent XSD assessment evidence, not a complete OMS JSON acceptance oracle or a selected production engine.

## Sources and exact profile

- UCI 002.5.0 Main and SecurityMarkings XSDs, plus Normalized Interface Specification Rev A, at Git `73a286fc4b881d9f328dbd64148525606a92c33a`. Historical blobs were extracted without substituting the current 2.6 checkout.
- Official OMS 2.5 OMSC-SPC-013 Rev B LA-CAL DOCX section 6.1, OMS revision `726272bd0390982a759c91a9cf4e13b81c2b510b`; unofficial Markdown was navigation only.
- SLEET revision `e3c2d798a229766571ed7be1f5b28d2dcdb51868`, used as implementation evidence. Copied probe modules remained unmodified and retained upstream license and schema provenance.

## Legal upstream failure fields

Pinned ServiceLifecycleCommandStatusMDT extends CommandStatusBaseType. Its MessageData requires CommandID, CommandProcessingState, LifecycleCommandStatus, and processor ServiceID. CommandProcessingStateReason is optional; if present, its CannotComplyType requires a valid Reason enum, while Description and AssociatedID are optional. There is no LifecycleCommandStatusReason or generic Error member.

Both ACCEPTED/FAIL and REJECTED/FAIL were independently assessed as schema-valid, including without diagnostics. The recommended application distinction is **ACCEPTED/FAIL after undertaking A by issuing B**, versus **REJECTED/FAIL when refusing A before undertaking it**. This distinction is a project policy, not a schema-forced timeout mapping. ACCEPTED, REJECTED, and CANCELED are terminal under Normalized Interface Rev A sections 4.6.1–.2/Figure 4.6-1/CERT UNIS-000105; do not send an early ACCEPTED then a second terminal result.

For upstream A, use A's CommandID and orchestrator's processor ServiceID. CONSTRAINT_TIME is a legal diagnostic Reason for expired observations; OPERATIONAL_INTENT_NOT_MET is a possible accurate diagnostic for downstream negative results. Include a truthful Description; timeout does not establish nonexecution or a child state. Omit optional polymorphic AssociatedID in the minimal demo sender until its codec is repaired. Exact local helper names remain implementation details.

Source ranges in pinned Main XSD: CommandStatusBaseType 18542–18563; ServiceLifecycleCommandStatusMDT 93202–93222; CannotComplyType 14278–14299; CommandProcessingStateEnum 113795–113821; LifecycleCommandStatusEnum 141020–141035. Full message still needs the normal OMS root, SecurityInformation, and MessageHeader.

## Executed generation and assessment

- Generated ordinary closure: 26 complex plus 33 simple types. Conservative reachable polymorphic-value closure: 306 complex plus 35 simple types. Both preserve only the three selected message roots and compile without compile repairs.
- Fifteen existing generator self-tests passed. These are not selected-profile compliance evidence by themselves.
- Fifty-four representative cases exercised the copied direct validator, generated Serde codec, and independent XML assessment against complete original Main plus Security schemas. Selected effective-facet compilation produced zero warnings.
- Direct validation falsely accepted six malformed dateTime/duration cases and falsely rejected two schema-valid security whitespace-collapse cases.
- Generated decoding discarded $type and derived-only fields such as Version, even with expanded declarations. Serde also discarded ordinary unknown members and accepted tested occurrence violations. Serde decoding is not schema validation.
- Installed xmllint/libxml2 2.9.13 compiled the pinned schemas. Valid XML assessments exited 0, invalid documents 3. XML reconstruction alone accepted three malformed JSON shape/primitive cases, proving it cannot replace strict OMS JSON checks.

## Concrete implementation repairs and evidence gates

1. Configure deterministic three-root generation with complete needed dependencies and provenance. Generate global-root metadata for selected topic/profile/root wrapping; bare message-body serialization is insufficient.
2. Generate field-position-aware base/derived dispatch preserving $type and derived fields, and verify valid/invalid derivation, abstractness and unknown fields. More declarations alone do not fix polymorphism.
3. Model/inherit whiteSpace and normalize an assessment value before enum/pattern/length checks, preserving original JSON bytes.
4. Implement XSD dateTime/duration lexical/calendar checks before selected restrictions. Generic RFC3339/duration parsers are not automatically equivalent; preserve valid XSD boundary cases and reject the observed invalid cases.
5. Audit selected schema constructs/facets at startup and fail rather than skip selected unsupported constraints. Original full inputs are retained, but routing remains restricted to three roots.
6. Verify namespace/type-name rules, root/primitive/array shapes, choices/occurrences, UUID restrictions, original-number lexemes and original-byte preservation. Pinned UUID rules accept nil or version1–5 with RFC4122 variant bits; a modern any-UUID parser accepting v7 would broaden acceptance.
7. Gate release on no unexplained selected-fixture discrepancy, no skipped selected constraints, and independent assessment of generated sends. Extend the representative corpus to unsampled security/derived/lexical cases. Do not add client runtime schema validation contrary to the selected encoding contract.

The conservative closure contains no float/decimal leaves or explicit numeric facets and its sampled uint32 boundaries worked. General f64 facet limitations and nested-model-group omissions are source-inspected dormant limitations, not demonstrated STOP defects. Full-schema generator repair is not required by these findings.

## Mapping interpretation and fallback decisions

Official Rev B section 6.1.2 explicitly requires $type for derived values; repeating elements use arrays. These wire rules are not optional project choices.

The probe's independent XML assessment expands arrays into sibling elements and uses xsi:type for $type. Table 6.1-2's array-recursion wording and empty-attributes rule do not explicitly state that bridge. This is a documented intended-encoding assessment interpretation, not proof of exact literal infoset equivalence. Remaining choices concern how to document/assess this discrepancy, not inventing new wire encoding.

Keep the direct validator behind one original-bytes-to-validated-root/original-bytes seam used on PUB and ASB ingress. If scoped repair cannot satisfy the evidence gate, a replacement needs **strict OMS JSON front-end checks plus verified XSD assessment**. libxml2 with a Rust wrapper is a concrete candidate, not yet selected or compiled. Its non-Send validation context would need worker-local ownership. Production engine/native dependency acceptance and latest-stable packaging must be decided before adopting such a fallback. Installed xmllint is an independent test oracle, not a selected production subprocess.

## Primary links and reproducibility

- [Pinned Main XSD](https://gitlab.com/open-arsenal/uci/standard/-/blob/73a286fc4b881d9f328dbd64148525606a92c33a/03_OAC-STD-002_RevE_UCI_Schema_v2_5/UCI_MessageDefinitions_v2_5_0.xsd)
- [Pinned SecurityMarkings XSD](https://gitlab.com/open-arsenal/uci/standard/-/blob/73a286fc4b881d9f328dbd64148525606a92c33a/03_OAC-STD-002_RevE_UCI_Schema_v2_5/UCI_SecurityMarkings_v2_5_0.xsd)
- [Pinned Normalized Interface Specification](https://gitlab.com/open-arsenal/uci/standard/-/blob/73a286fc4b881d9f328dbd64148525606a92c33a/06_OAC-SPC-002_RevA_UCI_NormalizedInterfaceSpecification_v2_5.docx)
- Official LA-CAL mapping: `standards-and-references/oms-standard/docs_official/20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.docx`, sections 6.1.1–.5 and Tables 6.1-1–3.
- [Pinned SLEET generator](https://gitlab.com/open-arsenal/ams-gra/hello-world-sk/infra/sleet/-/blob/e3c2d798a229766571ed7be1f5b28d2dcdb51868/scripts/generate_uci_types.py) and [validator](https://gitlab.com/open-arsenal/ams-gra/hello-world-sk/infra/sleet/-/blob/e3c2d798a229766571ed7be1f5b28d2dcdb51868/sleet/src/validator.rs).
- [Candidate libxml wrapper ownership/API](https://docs.rs/libxml/0.3.21/libxml/schemas/struct.SchemaValidationContext.html).

Detailed cited findings, exact executed commands, fixtures, errors and provenance live in the isolated branch `research/lifecycle-bindings-validation`, without commits:

- [Full research note](/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-lifecycle-validation-research/.scratch/rust-la-cal-prototype/research/lifecycle-bindings-validation.md)
- [Probe and execution record](/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-lifecycle-validation-research/lifecycle-validation-probe/)

Reproduction from that worktree: `python3 -B lifecycle-validation-probe/run.py`; executed child commands and results are retained in executed-commands.json and run-*.log. Latest Rust 1.99.0, Linux, full coverage, repaired validator behavior, C-wrapper builds, and final application/ASB integration were not tested. No prototype implementation or final coverage claim follows from these probes.
