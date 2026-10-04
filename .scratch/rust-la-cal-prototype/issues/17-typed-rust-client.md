# 17: Provide the selected typed Rust client

**What to build:** Rust applications can use one cloneable client to send and independently receive all three selected typed UCI messages through the adapted server, with inferred wire mapping and persistent application-owned subscription streams.

**Blocked by:** 16 — Replace local publication delivery with Zenoh/Protobuf.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md) and the [approved client/component interface](06-rust-component-boundaries.md#approved-public-client-surface), subject to the spec's final acceptance narrowing.

- [ ] Provide the approved asynchronous connect, generic send-by-reference, generic subscribe and explicit client/subscription close surface. A cloneable Client shares one OWP connection; connect returns only after successful WebSocket/OWP INIT/INFO and selected-profile checks within its configured connection deadline. Plain loopback WebSockets and verbose acknowledgments use the fixed supported profile.
- [ ] Explicitly generate and check in versioned UCI 002.5.0 bindings for `ServiceLifecycleCommand`, `ServiceLifecycleCommandStatus` and `ServiceStatus`, with complete required type/value dependencies, SecurityInformation, MessageHeader and OMS JSON root wrappers. Generate root/topic/profile encoding and decoding metadata so callers do not repeat wire strings; internal XSD types are not independently publishable roots.
- [ ] Record pinned schema and adapted-generator provenance, preserve artifact terms and source-adaptation notices, and document explicit regeneration. Ordinary Cargo builds compile checked-in UCI Rust without invoking Python.
- [ ] Concurrent application calls are permitted while PUB/SUB/UNSUB acknowledgment-tracked operations are internally serialized to one transmitted pending response per connection. Incoming MSG and WebSocket controls are dispatched independently while an operation awaits acknowledgment; application stream consumption cannot block acknowledgment processing.
- [ ] Typed send returns success on successful OWP Publication Acknowledgment, preserving server rejection details on failure. It does not promise subscriber receipt or application completion.
- [ ] Each subscribe owns an independent connection-local OWP subscription ID and a persistent stream of decoded messages or receive errors, including repeated subscriptions to the same type. Applications own processing and response matching; no callback execution, reply waiter, raw-message public API or grouped client API is introduced.
- [ ] Message-specific typed decoding errors are visible nonterminal stream items, allowing subsequent valid messages. Malformed OWP framing/control remains a protocol failure, and unexpected connection loss yields a visible terminal receive error before stream end.
- [ ] Explicit asynchronous subscription close consumes the stream, awaits UNSUB completion and leaves other registrations active. Drop ends local delivery and requests best-effort cleanup. Client clone drop leaves other handles usable; explicit close through a clone closes the shared connection. Detailed cancellation/deadline/drain behavior is completed in ticket 18.
- [ ] Verify typed send/receive for all three roots through the deployed server/adapter, independent repeated subscriptions, an interleaved MSG while awaiting acknowledgment, and relevant decoding/cleanup behavior at existing public client or OWP peer boundaries. Carry over useful existing tests and fix defects that obstruct selected runs or corrupt exercised data.
