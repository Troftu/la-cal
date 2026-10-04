# 16: Replace local publication delivery with Zenoh/Protobuf

**What to build:** The already-tested OWP topic exchange now traverses the real ASB adapter: validated original OMS JSON bytes are Protobuf-encoded, published through Zenoh, received by the shared subscriber, decoded, validated and forwarded to local OWP subscriptions. Successful publication acknowledgment reflects adapter operation completion.

**Blocked by:** 15 — Apply message-type topic routing and its tests.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md), especially ASB encoding, routing, readiness and acknowledgment decisions.

- [ ] Check in the project-defined proto3 `OmsJsonEnvelope` containing only the bytes field `oms_json`, permanently assigned field number 1. Carry complete OMS JSON payload bytes without OWP framing or duplicated type/profile metadata. Generate envelope Rust during Cargo builds using prost-build and latest-stable upstream protoc on PATH; record the actual versions.
- [ ] Map the three OWP topic/root names to the exact unsuffixed Zenoh keys under `la-cal/uci/002.5.0/`, with each message name as the final segment.
- [ ] Open one Zenoh session with an explicit loopback router connection and external discovery disabled. Establish router readiness using bounded stable router-ZID observation and one startup subscriber on `la-cal/uci/002.5.0/*` before admitting OWP clients.
- [ ] Expose an ordinary asynchronous adapter publication operation that internally owns synchronous-work offload and its configurable operation deadline. Connection I/O remains independent of that work. Complete verbose PUB +OK only after authorization, topic/root validation, schema assessment, original-byte envelope encoding and successful Zenoh publication completion. Errors remain visible in both verbosity modes.
- [ ] Incoming samples pass through envelope decoding, exact-key/root checks and the selected direct Rust payload validation before local delivery. Accepted original UTF-8 OMS JSON bytes remain unchanged through both directions; invalid input is reported and discarded rather than forwarded as valid.
- [ ] Delivery to all ordinary and selected group registrations originates from the actual Zenoh subscriber path. OWP PUB has no direct local recipient echo, and inbound ASB delivery is not republished. SDK-local delivery is allowed.
- [ ] SUB/UNSUB change local routing entries only. They do not create/destroy Zenoh listeners; the shared startup subscriber remains until server shutdown.
- [ ] Execute a native macOS OWP exchange through actual envelope encode, Zenoh publication/subscriber handling, decode, validation and forwarding, with observers registered before publication. Adapt relevant existing delivery tests to exercise this path and distinguish publication acknowledgment from observed delivery without assuming their arrival order.
- [ ] Provide repeatable local router/server configuration and launch instructions, bounded partial-startup/shutdown handling, and recorded dependency/protoc/router versions and source-adaptation notices. Remaining queue/connectivity failure behavior is completed in ticket 19.
