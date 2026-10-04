# Native Rust tooling lookup

Date: 2026-10-03
Context: [Choose the Rust component interfaces and dependencies](../issues/06-rust-component-boundaries.md)
Status: factual recommendation inputs; no dependency selections yet

## Workspace

No prototype Cargo manifest or toolchain pin exists. The read-only SLEET reference uses Rust 1.95.0, edition 2024, Tokio 1.50.0, and tokio-tungstenite 0.29.0 in its lockfile. Those are reference implementation choices, not prototype pins.

## Released tools checked

Crates.io metadata reports Tokio 1.53.2, tokio-tungstenite 0.30.0, Zenoh 1.10.1, and prost/prost-build 0.14.4 as latest stable non-yanked releases at lookup time. No combined dependency graph was built; crate-level minimum Rust versions do not establish the combined toolchain requirement.

- [Tokio 1.53.2 API](https://docs.rs/tokio/1.53.2/tokio/): native macOS/Linux runtime, process management, timers, bounded mpsc, oneshot, watch, and select. Async hooks must yield. Timeouts are cooperative; started spawn_blocking work cannot be aborted. Dropping a child handle does not terminate the process by default; kill/wait ownership must be explicit.
- [tokio-tungstenite 0.30.0 API](https://docs.rs/tokio-tungstenite/0.30.0/tokio_tungstenite/): async client/server WebSocket streams. Plain WS needs no TLS feature; TLS features are optional. [Tungstenite control-frame behavior](https://docs.rs/tungstenite/0.30.0/tungstenite/protocol/struct.WebSocket.html) requires continued I/O polling/flushing for queued automatic Pong/Close replies. Library capabilities alone do not remove the selected deployment's TLS exception.
- [Zenoh 1.10.1 Session API](https://docs.rs/zenoh/1.10.1/zenoh/session/struct.Session.html): one shared session, startup subscriber, publication, and explicit closure are supported. [Release artifacts](https://github.com/eclipse-zenoh/zenoh/releases/tag/1.10.1) include native macOS/Linux routers.
- [prost-build 0.14.4](https://docs.rs/prost-build/0.14.4/prost_build/): Cargo build-script generation into OUT_DIR with external protoc. Config::protoc_executable or PROTOC can select the compiler. Proto3 bytes defaults to Vec<u8>. Successful Protobuf decoding does not establish a valid JSON payload.

## Zenoh readiness and outage caveats

- [Stable routers_zid observation](https://docs.rs/zenoh/1.10.1/zenoh/session/struct.SessionInfo.html#method.routers_zid) reports connected routers. Polling can miss a disconnect/reconnect between polls.
- [Transport/link event listeners](https://docs.rs/zenoh/1.10.1/zenoh/session/struct.SessionInfo.html#method.transport_events_listener) require the unstable feature. history(true) supplies current transports, not a historical outage log.
- [Released configuration](https://github.com/eclipse-zenoh/zenoh/blob/1.10.1/DEFAULT_CONFIG.json5) controls explicit endpoints, scouting, connection deadlines, and open-return conditions. Generic open success and successful subscriber declaration must not stand in for router readiness. Session closure is distinct from router loss.
- Default transport lease is 10,000 ms; a five-second required-event wait does not automatically guarantee five-second outage detection.
- [Publication builder](https://github.com/eclipse-zenoh/zenoh/blob/1.10.1/zenoh/src/api/builders/publisher.rs) performs synchronous work before returning a ready future. Awaiting put does not establish nonblocking execution or recipient acknowledgment; a Tokio timeout around it is not a hard execution limit.
- SDK reconnect attempts do not override the selected fail-on-detected-outage contract.

## Compiler pinning caveat

[protoc 36.2](https://github.com/protocolbuffers/protobuf/releases/tag/v36.2) is the latest official release checked. [prost 0.14.4 CI](https://github.com/tokio-rs/prost/blob/v0.14.4/.github/workflows/ci.yml) declares PROTOC_VERSION 3.25.3; that is not compatibility evidence for protoc 36.2. Select and verify a compiler independently of Cargo crate pins.

## Remaining factual work

OMS JSON validation tooling and coverage of the selected UCI 002.5.0 dependency closure were deliberately left for a separate lookup. Native dependency compatibility also needs a resolved graph before treating proposed pins as verified.

## Wasm async alternatives: explanatory follow-up

These are capability distinctions, not portability selections or executed build evidence:

- Tokio drives tasks/timers/networking; tokio-tungstenite integrates the Tungstenite WebSocket protocol implementation with Tokio I/O. [Tokio's documented Wasm restrictions](https://docs.rs/tokio/1.53.2/tokio/#wasm-support) do not establish native networking support on WASIp3.
- Browser Wasm can use [wasm-bindgen-futures](https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/fn.spawn_local.html) for task scheduling and browser WebSockets through [web-sys](https://wasm-bindgen.github.io/wasm-bindgen/examples/websockets.html) or [gloo-net](https://docs.rs/gloo-net/latest/gloo_net/websocket/futures/). These need JavaScript/browser APIs, not a WASI host.
- [wstd 0.6.8](https://docs.rs/wstd/0.6.8/wstd/) documents a single-thread runtime, timers/tasks, and WASIp2 networking for wasm32-wasip2. No integrated WebSocket API or demonstrated p3 support was established. Its I/O traits differ from futures-io; generic async-tungstenite integration needs an adapter and verification.
- [wasip3 bindings](https://github.com/bytecodealliance/wasi-rs), [async TCP](https://docs.rs/wasip3/latest/wasip3/sockets/types/struct.TcpSocket.html), and [scoped spawn_local](https://docs.rs/wasip3/latest/wasip3/fn.spawn_local.html) are the direct component-aware async path to investigate for p3. Matching wit-bindgen versions, guest/host/toolchain support, and a WebSocket stream integration remain unverified. [Wasmtime 49.0.2 p3 documentation](https://docs.rs/wasmtime-wasi/49.0.2/wasmtime_wasi/p3/index.html) labels its support experimental, unstable, and incomplete.
- smol with [async-tungstenite's smol integration](https://docs.rs/async-tungstenite/0.35.0/async_tungstenite/smol/) is a native runtime alternative; these docs do not establish a WASIp3 socket solution.

The separate [WASIp3 spike](../../wasip3-service-portability/issues/01-wasip3-service-runtime-spike.md) remains the place for a build/run proof and platform-seam selection.
