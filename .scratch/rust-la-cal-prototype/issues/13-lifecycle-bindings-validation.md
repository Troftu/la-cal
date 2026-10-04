# Verify the selected lifecycle bindings and validation coverage

Parent: ../map.md
Labels: wayfinder:research
Type: research
Status: resolved
Blocked by: 01, 04, 10, 11, 12

## Question

What concrete generated-binding and direct-validator repairs, OMS JSON mapping decisions, and fallback prerequisites are needed to support ServiceLifecycleCommand, ServiceLifecycleCommandStatus, and ServiceStatus under the pinned UCI 002.5.0 profile, including a legal upstream relay failure response?

Ground failure field requirements and terminal processing-state semantics in the pinned schema/normalized interaction sources. Exercise SLEET's three-root generation in an isolated probe and compile the resulting Rust if feasible; verify root wrapping and identify unsupported schema constructs rather than assuming Serde derives establish correct OMS JSON. Investigate the selected closure's known validator gaps and which can be specified as implementation repairs versus unresolved mapping/engine decisions. Execute representative positive/negative schema or direct-validator checks where practical, clearly separating observed failures, source-inspected gaps, proposed fixes, and untested coverage. Investigate a concrete independent assessment/fallback path without installing system software or claiming that a generic JSON-to-XML utility establishes OMS mapping correctness.

Do not implement the prototype or select a broader message profile. A useful resolution is a bounded repair/verification plan with precise remaining decisions, not unsupported proof of full validation coverage.

## Context

- [Choose the Rust component interfaces and dependencies](06-rust-component-boundaries.md) records the selected SLEET generator/direct-validator starting point and current relay failure policy.
- Existing analysis: [OMS JSON validation options](../research/json-validation-options.md) and [SLEET reuse feasibility](../research/sleet-server-reuse-feasibility.md).
- Use official OMS 2.5 / LA-CAL Rev B mapping and pinned UCI 002.5.0 provenance; the current uci-standard checkout is 2.6 and must not be silently substituted.
- Isolated branch: `research/lifecycle-bindings-validation`; worktree: `/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-lifecycle-validation-research`.
- Write the research note at `.scratch/rust-la-cal-prototype/research/lifecycle-bindings-validation.md` in that worktree. No commits are authorized. Reference submodules and the main tracker remain read-only for the researcher.

## Comments

- 2026-10-04: Created and claimed for AFK source/execution verification after the user selected explicit upstream relay failure. Findings inform the remaining mapping/validation/interface decisions, without resolving those human choices.

## Answer

[Verified lifecycle binding and validation findings](../research/lifecycle-bindings-validation.md) grounds legal failure fields and terminal semantics in pinned UCI 002.5.0, exercises both generated closures and 54 representative assessments, and specifies concrete codec/validator repair gates. Generated Rust compiles; temporal/whitespace defects and loss of polymorphic data are real, unrepaired findings. Independent XSD assessment is available through installed xmllint but cannot enforce OMS JSON shape/primitive rules by itself.

The research recommends attempted-relay ACCEPTED/FAIL versus pre-execution REJECTED/FAIL as application policy, documents the intended-encoding assessment interpretation, and outlines a strict-JSON-plus-XSD fallback boundary. Those human choices and any production fallback engine remain unselected. The linked note retains exact evidence/provenance and isolated branch/probe pointers. Latest Rust/Linux/full-coverage/application integration remain untested.
