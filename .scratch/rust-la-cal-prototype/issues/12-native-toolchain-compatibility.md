# Verify the native Rust and Protobuf toolchain compatibility

Parent: ../map.md
Labels: wayfinder:research
Type: research
Status: resolved
Blocked by: 01, 04, 07

## Question

Which exact released Rust, Tokio, tokio-tungstenite, Zenoh, prost/prost-build, and protoc versions form a compatible native toolchain for the selected prototype, and what evidence supports recommending those pins?

Verify the previously proposed dependency versions against primary sources rather than treating lookup notes as established compatibility. Resolve a combined dependency graph and compile a small throwaway probe for the APIs needed by the client/server and Zenoh/Protobuf adapter, including plain WebSockets, explicit local-router configuration, startup subscriber/publication/closure, and generation of the single-field bytes envelope. Exercise the installed protoc 36.2 with the chosen prost-build if feasible. Distinguish host compilation, cross-platform support documented by upstream, API inspection, and executed runtime checks. Report network/toolchain/cache blockers precisely; do not implement the prototype or claim unexecuted validation/macOS/Linux compatibility.

JSON validator coverage and the application's failure response policy are separate remaining interface decisions. Include only tooling facts needed to expose their dependencies or constraints.

## Context

- See [Choose the Rust component interfaces and dependencies](06-rust-component-boundaries.md), [Define the OMS JSON-in-Protobuf encoding contract](04-schema-protobuf-mapping.md#answer), and [Define the Zenoh ASB routing and delivery contract](05-zenoh-asb-contract.md#answer).
- Existing inputs: [Native Rust tooling lookup](../research/native-tooling-notes.md). These are source-inspected candidates, not a combined build.
- Host tools observed on 2026-10-04: rustc/cargo 1.91.0 and protoc 36.2. Prefer evidence over assuming those tools are sufficient.
- Isolated throwaway branch: `research/native-toolchain-compatibility`; worktree: `/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-native-toolchain-research`.
- Capture primary-source citations and execution results in `research/native-toolchain-compatibility.md` under this effort on the research branch/worktree; link the resulting note on resolution. No commit is required or authorized.

## Comments

- 2026-10-04: Created and claimed for AFK tooling verification while continuing the component-interface effort. The application/API decisions already confirmed remain intact; research findings inform dependency choices rather than selecting them for the user.

## Answer

The [verified native toolchain candidates and evidence](../research/native-toolchain-compatibility.md) establish a combined macOS arm64 build using the previously proposed Tokio/WebSocket/Zenoh/prost versions with Rust 1.91.0 and installed protoc 36.2. Generated envelope byte fidelity, plain loopback WebSocket behavior, and Tokio/configuration checks passed. Zenoh integration APIs compiled; actual router behavior and Linux execution remain untested. No compatibility blocker was found.

The note records exact feature recommendations, primary-source citations, execution limits, and the isolated research branch/probe pointer. Version/feature adoption remains the user's interface decision; stable polling and synchronous publication caveats still need adapter policy decisions. Research does not select or prove validator coverage or final application dependency compatibility.
