# Verified native Rust and Protobuf toolchain candidates

Date: 2026-10-04
Context: [Verify the native Rust and Protobuf toolchain compatibility](../issues/12-native-toolchain-compatibility.md)
Status: executed throwaway compatibility probe plus latest-stable/compiler-sourcing lookup; the old probe baseline is not the selected toolchain

## Findings

The combined probe compiled and ran successfully on macOS arm64 using these exact versions:

| Component | Verified candidate |
| --- | --- |
| Rust / Cargo | 1.91.0 |
| Tokio | 1.53.2 |
| tokio-tungstenite | 0.30.0 |
| Zenoh | 1.10.1 |
| prost / prost-build | 0.14.4 |
| protoc | 36.2 |
| futures-util | 0.3.31 |

The exact-version crates.io metadata confirmed the five previously proposed crates exist and are non-yanked. No new Rust toolchain or system software was installed. These are tested available candidates, not a claim that each is the latest release.

Recommended feature shape exercised by the probe:

```toml
tokio = { version = "=1.53.2", features = ["rt-multi-thread", "macros", "net", "io-util", "sync", "time", "process", "signal"] }
tokio-tungstenite = { version = "=0.30.0", default-features = false, features = ["connect", "handshake"] }
zenoh = { version = "=1.10.1", default-features = false, features = ["transport_tcp"] }
prost = "=0.14.4"
futures-util = { version = "=0.3.31", features = ["sink"] }
# build dependency:
prost-build = "=0.14.4"
```

The probe used edition 2024, resolver 3, rust-version 1.91, an explicit version-checked protoc executable, and a generated Cargo.lock. Its target-filtered metadata graph had one Tokio version, no TLS/QUIC packages, and a highest declared external MSRV of 1.88.0; 69 packages did not declare an MSRV. Rust 1.91.0 is tested, not an established minimum. The final application/validator graph will need its own lockfile and verification.

## Executed evidence and limits

The isolated probe passed `cargo build` (59.57 seconds), `cargo run --locked --offline`, and its locked/offline metadata audit. Commands used an isolated CARGO_HOME, explicit PROTOC, and probe-local TMPDIR; the detailed record is linked below.

- Generated the selected `OmsJsonEnvelope { bytes oms_json = 1; }` using installed protoc 36.2 and prost-build 0.14.4; compiled and exercised byte-for-byte JSON encode/decode preservation, including an unknown envelope field. The JSON fixture was synthetic, not a schema-valid UCI message.
- Executed Tokio task/channel/timer/timeout/process spawn-and-wait checks.
- Executed a real loopback plain WebSocket upgrade, binary echo, and Close exchange. This did not exercise OWP negotiation or protocol operations.
- Executed Zenoh explicit-router/no-scouting configuration parsing. Zenoh open, router-ZID observation, startup wildcard subscription, put/receive, undeclare, and close APIs compiled but were deliberately not run against a router.
- Tested macOS arm64 only. Upstream supports Linux, but this combined feature/lockfile set was not executed there.

## Remaining adapter decisions

Stable `session.info().routers_zid()` provides a current connectivity snapshot, so bounded polling can serve readiness and detected-outage checks. It can miss a disconnect/reconnect between polls. Zenoh transport/link event listeners require its `unstable` feature and were not exercised. Existing fail-on-detected-outage scope does not imply every transient outage is observed.

Zenoh `put` constructs a ready future by doing synchronous work first. A Tokio timeout cannot preempt that work, and successful put neither proves router presence nor subscriber receipt. Offloading can protect I/O responsiveness, but does not make started blocking work cancellable. The default transport lease is 10 seconds; five-second application event waits are not a promise of five-second loss detection. Adapter execution/cleanup policy and actual router/outage integration remain open.

## Primary sources

- Exact released crate metadata: [Tokio 1.53.2](https://crates.io/api/v1/crates/tokio/1.53.2), [tokio-tungstenite 0.30.0](https://crates.io/api/v1/crates/tokio-tungstenite/0.30.0), [Zenoh 1.10.1](https://crates.io/api/v1/crates/zenoh/1.10.1), [prost 0.14.4](https://crates.io/api/v1/crates/prost/0.14.4), [prost-build 0.14.4](https://crates.io/api/v1/crates/prost-build/0.14.4).
- [Rust 1.91.0 release](https://blog.rust-lang.org/2025/10/30/Rust-1.91.0/), [protoc 36.2 release](https://github.com/protocolbuffers/protobuf/releases/tag/v36.2), [prost-build configuration](https://github.com/tokio-rs/prost/blob/v0.14.4/prost-build/src/config.rs).
- [Cargo Rust-version-aware resolution](https://doc.rust-lang.org/cargo/reference/resolver.html#rust-version).
- Zenoh 1.10.1: [configuration](https://github.com/eclipse-zenoh/zenoh/blob/1.10.1/DEFAULT_CONFIG.json5), [stable info and unstable event APIs](https://github.com/eclipse-zenoh/zenoh/blob/1.10.1/zenoh/src/api/info.rs), [publication builder](https://github.com/eclipse-zenoh/zenoh/blob/1.10.1/zenoh/src/api/builders/publisher.rs), [publication/session implementation](https://github.com/eclipse-zenoh/zenoh/blob/1.10.1/zenoh/src/api/session.rs).
- [Tokio timeout](https://docs.rs/tokio/1.53.2/tokio/time/fn.timeout.html), [spawn_blocking](https://docs.rs/tokio/1.53.2/tokio/task/fn.spawn_blocking.html).
- Upstream platform evidence: [Tokio supported platforms](https://github.com/tokio-rs/tokio/blob/tokio-1.53.2/tokio/src/lib.rs), [Zenoh CI](https://github.com/eclipse-zenoh/zenoh/blob/1.10.1/.github/workflows/ci.yml), [prost CI](https://github.com/tokio-rs/prost/blob/v0.14.4/.github/workflows/ci.yml).

## Detailed record and reproducible probe

Research branch: `research/native-toolchain-compatibility`. No commits were made. The detailed source citations, full executed command record, hashes, and probe assets live in the isolated temporary worktree:

- [Detailed research note](/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-native-toolchain-research/.scratch/rust-la-cal-prototype/research/native-toolchain-compatibility.md)
- [Probe manifest](/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-native-toolchain-research/native-toolchain-probe/Cargo.toml) and [source](/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-native-toolchain-research/native-toolchain-probe/src/main.rs).

No prototype implementation, UCI validation proof, full generated-binding check, actual Zenoh traversal, or OMS/OWP conformance claim follows from this tooling probe.

## Latest-stable and compiler-sourcing addendum

After the probe, the user directed use of latest stable tooling. A fresh 2026-10-04 primary-source lookup found Rust 1.99.0 (official stable manifest dated 2026-10-01), Tokio 1.53.2, tokio-tungstenite 0.30.0, Zenoh 1.10.1, prost/prost-build 0.14.4, futures-util 0.3.34, and upstream protoc 36.2. Crate metadata was filtered to non-yanked non-prerelease releases. The initial probe remains valid evidence for its exact tested set, not proof of a complete Rust 1.99.0/futures-util 0.3.34 build. Rust 1.99.0 was not installed during research.

Current [prost-build compiler-sourcing documentation](https://docs.rs/prost-build/0.14.4/prost_build/#sourcing-protoc) says it needs protoc through PATH, PROTOC, or Config::protoc_executable. It does not bundle the compiler. The separately maintained [protoc-bin-vendored 3.2.0](https://docs.rs/protoc-bin-vendored/3.2.0/protoc_bin_vendored/) provides prebuilt host binaries, while protobuf-src is a separate source-build option.

The latest protoc-bin-vendored package is 3.2.0, but its [released version file](https://docs.rs/crate/protoc-bin-vendored/3.2.0/source/version.txt) says v31.1. Executing the packaged host compiler confirmed `libprotoc 31.1`, which lags latest upstream 36.2. A separate build-only probe with that helper and prost/prost-build 0.14.4 generated and compiled the selected envelope successfully in 7.19 seconds. This proves the helper path works for that envelope, not compliance with the user's latest-upstream-compiler preference. No compiler-sourcing option was selected by research.

Latest-release sources: [official Rust stable manifest](https://static.rust-lang.org/dist/channel-rust-stable.toml), [official protobuf latest release](https://api.github.com/repos/protocolbuffers/protobuf/releases/latest), crates.io current metadata for [Tokio](https://crates.io/api/v1/crates/tokio), [tokio-tungstenite](https://crates.io/api/v1/crates/tokio-tungstenite), [Zenoh](https://crates.io/api/v1/crates/zenoh), [prost](https://crates.io/api/v1/crates/prost), [prost-build](https://crates.io/api/v1/crates/prost-build), [futures-util](https://crates.io/api/v1/crates/futures-util), and [protoc-bin-vendored](https://crates.io/api/v1/crates/protoc-bin-vendored).

The [detailed research note](/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-native-toolchain-research/.scratch/rust-la-cal-prototype/research/native-toolchain-compatibility.md) contains the addendum's exact command record. Its separate probe assets live under `vendored-protoc-probe/` in that worktree; the original combined probe is preserved.
