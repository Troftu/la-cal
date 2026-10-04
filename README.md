# Native Rust LA-CAL prototype

The project-owned SLEET adoption baseline (ticket 14) runs on native macOS:
an authorized OWP Client negotiates `owp` / OWP 1.0, subscribes, publishes a
validated UCI 002.5.0 `ServiceStatus`, observes `+OK` and receives the original
JSON in `MSG` through the adopted local router.

## Build and run

Run these commands from the repository root. Rustup installs the recorded
Rust 1.99.0 toolchain selected at baseline establishment.

```sh
cargo build --locked --workspace
target/debug/la-cal config/server.toml
```

The server binds `127.0.0.1:9000`. Stop it with Ctrl-C. The configuration's
schema and participant-directory paths are relative to the working directory.
`config/services.d/baseline.toml` authorizes `baseline-tester` for the
`baseline` topic, bound to `ServiceStatus`. No credentials or TLS are configured.

For a repeatable native exchange with automatic server launch and cleanup:

```sh
cargo test --locked -p la-cal --test native_baseline -- --nocapture
```

This test launches the actual `la-cal` binary on an ephemeral loopback port,
checks the negotiated WebSocket subprotocol and INFO identities, awaits SUB,
PUB and UNSUB acknowledgments, and independently checks original-byte MSG
delivery. Readiness and the exchange are bounded; the owned process is killed
and reaped on success or test failure. It uses
[`fixtures/service-status.json`](fixtures/service-status.json), constructed
from the pinned schema rather than a generated serialization expectation.

## Checks and regeneration

```sh
cargo check --locked --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets
python3 -B scripts/generate_uci_types.py --self-test
SLEET_SCHEMA_PATH="$PWD/spec/002.5.0/UCI_MessageDefinitions_v2_5_0.xsd" cargo test --locked --workspace
```

Set `SLEET_SCHEMA_PATH` for the full-schema parser check; the adopted upstream
test otherwise prints a skip and returns successfully. This legacy variable
name is retained for the baseline. Ordinary builds use checked-in bindings.
Explicit regeneration and separate UCI terms are documented in
[`spec/002.5.0/PROVENANCE.md`](spec/002.5.0/PROVENANCE.md).

## Workspace and milestone

| Package | Purpose at this milestone |
| --- | --- |
| `la-cal` | Adopted OWP frontend, authorization, local routing/groups, direct Rust schema validator, admin/telemetry and copied tests |
| `owp-types` | Adopted OWP grammar/types and baseline UCI bindings/tests |
| `baseline-client` | Unmodified upstream-style client used by copied socket tests; temporary adoption support |

The final cloneable typed `la-cal-client`, message-type topic policy, real
Zenoh/Protobuf ASB and relayed STOP applications are subsequent tickets. This
baseline intentionally retains upstream publication/delivery and queue behavior.
It is source-adoption evidence, not final prototype acceptance or a conformance
claim. Plain `ws` retains the explicit RFC 6455 §10.6 TLS-capability exception.

See [`docs/baseline-evidence.md`](docs/baseline-evidence.md) for executed checks,
versions and known limitations. [`NOTICE`](NOTICE) records the selected SLEET
revision and actual modifications; [`LICENSE`](LICENSE) preserves its Apache-2.0
terms. Reference submodules are read-only.
