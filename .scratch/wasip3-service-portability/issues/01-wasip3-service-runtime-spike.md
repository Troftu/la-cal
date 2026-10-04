# Spike WASIp3 support for Service lifecycle and LA-CAL clients

Status: ready-for-agent
Labels: spike
Priority: deferred follow-up

## Question

**Re-scope before execution:** [Choose a client-focused relayed STOP demonstration](../../rust-la-cal-prototype/issues/11-client-focused-stop-demo.md#answer) drops `service-lifecycle`, renames `example-service` to `commandee-service`, and defers START/process launch. The original investigation below is historical input, not authority to restore those requirements. This remains a separate deferred follow-up; its revised proof should target application-owned lifecycle behavior and `la-cal-client` after native interfaces are concrete.

Can `example-service`, `service-lifecycle`, and `la-cal-client` compile and execute as a WASI Preview 3 component while preserving the agreed lifecycle behavior and OWP communication with native `la-cal`? Identify the smallest working dependency/adapter combination and the concrete changes needed to the native interfaces.

## Context and timing

The user confirmed the [reusable lifecycle runtime design](../../rust-la-cal-prototype/lifecycle-runtime.md) and [native START demo](../../rust-la-cal-prototype/issues/09-demo-participant-topology.md#answer), then explicitly deferred WASIp3 work to this spike. This issue is outside the [native prototype decision map](../../rust-la-cal-prototype/map.md) and does not block its component-interface/dependency or acceptance-evidence decisions. Resume it as a separate follow-up.

`la-cal` and `orchestrator` remain native. Other Service applications should ideally be Wasm-capable, so their shared lifecycle and LA-CAL client crates need that capability. The driver may stay native. Preserve the crate names: `orchestrator`, `example-service`, `la-cal`, `la-cal-client`, `service-lifecycle`, `demo-driver`.

The native orchestrator can launch a Wasm Service through a runtime process. No Wasm orchestrator, guest process-spawning API, or custom host Service-launch interface is required by this spike. Successful OS process creation remains the START command-success milestone; actual component loading, connection, and initialization are separate demo observations.

## Investigation

1. Recheck current released Rust support for the exact `wasm32-wasip3` target, distributed std availability, component artifact format, runtime support, and networking permissions. Pin the tested compiler, WASI bindings, and host versions; distinguish released capability from main/nightly documentation.
2. Verify the shared crates' actual dependency graph and build features. Identify native-only runtime/net/process/fs/thread assumptions, WebSocket dependencies, randomness, and any code-generation/native-build prerequisites. Do not infer compatibility from wasm32-unknown-unknown or WASIp2 support.
3. Evaluate an executor, clock waits/deadlines, and channels/state observation for the lifecycle supervisor. Exercise asynchronous application hooks without stopping periodic reporting or control reception. Determine whether direct WASIp3 bindings or a constrained existing executor is simpler.
4. Establish OWP/WebSocket transport over real WASIp3 sockets to native `la-cal`. Evaluate generic WebSocket stream integration versus target-specific transport support. Preserve WebSocket/OWP negotiation and acknowledgment semantics rather than substituting host calls for messaging.
5. Determine the smallest actual platform seam needed by `service-lifecycle` and `la-cal-client`. A supervisor future, injected clock, target-specific transport, and relaxed Send bounds were discussed as candidates, not selected designs. Avoid a broad platform framework.

## Bounded proof

Use a throwaway implementation/prototype or the existing native baseline when available. Do not silently replace the native demo or expand it to full lifecycle support.

- Build an example Service using both shared crates for native and `wasm32-wasip3` targets.
- Run the Wasm component against native `la-cal` and its real Zenoh/Protobuf adapter with loopback networking enabled explicitly.
- Complete OWP INIT/INFO and subscription setup; publish INITIALIZING before initialization completes, then NORMAL; observe periodic reports with the selected one-second default.
- Exercise a health-driven transition and a pending asynchronous action to demonstrate that reporting continues. If PAUSE/RESUME is used for the proof, demonstrate actual suspension/application acknowledgment and restored previous state, not just enum changes.
- Verify event waits/timers wake an otherwise idle supervisor and that application waits do not depend on unavailable OS threads or blocking pools.
- Launch the component runtime process through the native orchestrator or a representative native launcher; distinguish successful process creation from successful guest startup. Verify failure reporting and termination/reaping.

This is a capability proof, not the native map's complete conformance suite or a new TLS/recovery requirement.

## Resolution / acceptance

Produce one findings document linked from this issue, with:

- Exact tested versions, build/run commands, host flags/permissions, and a reproducible artifact or prototype link.
- Working native and WASIp3 observations for OWP, lifecycle initialization, reporting/timer behavior, and application hooks, or a precise blocker with reproduction.
- The actual dependency compatibility results; separate source-inspected capability from executed evidence.
- A small proposed change list for public interfaces and target-specific implementations, including unresolved trade-offs and limitations.
- A recommendation on when Wasm execution should become an acceptance requirement. That requirement was deferred, not already selected.

Preserve OMS 2.5 / UCI 002.5.0, OWP, message-type routing, JSON-byte preservation, and the existing command/status distinctions. Consult `oms-la-cal` and the pinned profile when assessing messaging/lifecycle behavior.

## Primary-source starting points

Recheck these sources at execution time; their current contents are not a pinned toolchain decision or a demonstrated build:

- [Rust wasm32-wasip3 target documentation](https://doc.rust-lang.org/nightly/rustc/platform-support/wasm32-wasip3.html).
- [WASI 0.3.0 socket interfaces](https://github.com/WebAssembly/WASI/blob/v0.3.0/proposals/sockets/wit/types.wit) and [clock interfaces](https://github.com/WebAssembly/WASI/blob/v0.3.0/proposals/clocks/wit/monotonic-clock.wit).
- [Rust WASI bindings and crate-selection guidance](https://github.com/bytecodealliance/wasi-rs).
- [Tokio Wasm support](https://docs.rs/tokio/1.53.2/tokio/#wasm-support).
- [Wasmtime 49.0.2 p3 implementation documentation](https://docs.rs/wasmtime-wasi/49.0.2/wasmtime_wasi/p3/index.html).
- [tokio-tungstenite transport interfaces and features](https://github.com/snapview/tokio-tungstenite).

## Comments

- 2026-10-03: Created at the user's request to defer WASIp3 feasibility and platform-adapter decisions while persisting the agreed reusable lifecycle runtime. No Wasm build/run proof or dependency selection was performed in that conversation.
