# Assess reusing SLEET's LA-CAL server with a Zenoh/Protobuf ASB

Parent: ../map.md
Labels: wayfinder:research
Type: research
Status: resolved
Blocked by: 01, 04, 05, 07, 09

## Question

Can we reuse SLEET's LA-CAL server as the native OWP frontend for our selected START demonstration, replacing or adapting its in-process bus path to the project-defined Zenoh/Protobuf ASB, rather than building another OWP server? Identify the smallest viable reuse seam and the evidence behind its feasibility.

Inspect source structure and public/exported interfaces, licensing and provenance, server startup/configuration, handshake/connection operation handling, validation, local subscription registration and group selection, outbound queues, acknowledgment timing, shutdown, and test surfaces. Trace every publication/receive path: actual Protobuf encode/Zenoh publish/subscriber/decode is mandatory, while direct OWP-to-OWP echo is forbidden. SDK-local Zenoh delivery remains acceptable.

Compare direct dependency, maintained project-owned fork/adaptation, and selective reuse only as needed to identify the recommended approach. List exact files/functions that need changes and why; distinguish necessary contract fixes from optional cleanup. Check how much of SLEET's existing protocol tests can be reused and which selected-profile coverage gaps remain, especially bidirectional OMS JSON validation, bus publication failure, subscription groups, slow-client overflow closure, readiness/outage observation, and orderly shutdown. Preserve the three-message UCI 002.5.0 scope and the chosen typed client/generator direction; do not assume this research settles lifecycle hooks or the client actor interface.

Use official OMS 2.5 / LA-CAL Rev B / OWP 1.0 and pinned UCI 002.5.0 sources for normative claims. Reference submodules are read-only. This is research/planning, not implementation or an automatic decision to adopt a fork. Source inspection must be distinguished from executed evidence; run only focused existing checks or throwaway feasibility checks when needed to establish a factual claim.

## Research context

User requested this investigation after learning SLEET already implements subscription groups. The ongoing [component-interface decision](06-rust-component-boundaries.md) is parked pending these findings; its confirmed choices remain intact. Capture one findings document under research/ and link it on resolution. Research branch/worktree details will be recorded before delegated work begins.

- Research branch: research/sleet-server-adaptation-20261003
- Worktree: /var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-sleet-server-research
- Primary reference source: /Users/dp/git/Troftu/la-cal/standards-and-references/ (read-only; use this populated checkout rather than initializing worktree submodules).
- Findings destination: .scratch/rust-la-cal-prototype/research/sleet-server-reuse-feasibility.md (draft in the research worktree, then adopted into the main map).

## Answer

Source inspection supports reuse through a project-owned adaptation of SLEET's OWP server frontend. Its unchanged binary constructs an in-process router and has no bus injection point; the server package does not export a library interface. A maintained source adaptation is the recommended candidate, not an adoption decision made by this research.

The narrow seam is publication completion: retain OWP parsing/state/authorization and local SUB/UNSUB/group registration, replace process_pub's direct local fan-out with validated Protobuf/Zenoh publication, and invoke receive-only local delivery after the startup Zenoh subscriber decodes and validates the envelope. This preserves the actual adapter-traversal and acknowledgment contracts while accepting SDK-local Zenoh delivery.

Necessary work includes bidirectional selected-profile validation, overflow-triggered connection closure, stale subscription/SID delivery protection, reliable task-abort cleanup, bounded WebSocket closing, bus-aware readiness/outage supervision, and explicit session/task shutdown. The selected shared-client interface and three-root generation also require adaptation; they are separate from server reuse. SLEET's Apache-2.0 licensing permits adaptation with retained license/attributions and modification notices; schema provenance has separate terms.

The in-memory unchanged-generator checks passed 15 existing checks and rendered the selected three roots with 26 complex and 33 simple named dependencies, with no unresolved references. This proves Python discovery/rendering only. A focused Rust router-test attempt stopped before compilation because metrics-exporter-prometheus was missing from the offline cache; the upstream Rust 1.95.0 toolchain was not installed. No Rust/bus-runtime/schema-conformance proof is claimed.

Full source citations, inspected revisions, exact commands/outcomes, required-change table and evidence plan: [SLEET server reuse with the selected Zenoh/Protobuf ASB](../research/sleet-server-reuse-feasibility.md). Findings were drafted on the recorded research branch/worktree and adopted into the main map; no commits or implementation changes were made.

This resolves feasibility research and unblocks [Choose the Rust component interfaces and dependencies](06-rust-component-boundaries.md), which must choose adoption and settle the remaining execution, validation, configuration and lifecycle-hook interfaces.
