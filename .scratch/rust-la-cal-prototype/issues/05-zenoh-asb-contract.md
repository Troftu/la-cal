# Define the Zenoh ASB routing and delivery contract

Parent: ../map.md
Labels: wayfinder:grilling
Type: grilling
Status: resolved
Blocked by: 03, 04, 07

## Topology update

Use [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer) for the current two-Service command exchange. Retain one shared CAL Server, one Zenoh session, the startup wildcard listener, message-type routing, actual adapter evidence, and SDK-local delivery. Multiple CAL Servers and physical-router-hop evidence remain deferred. Earlier START comments are historical.

## Question

Apply the [operating boundary](07-prototype-operating-boundaries.md#answer): one local server/router, an orchestrator and example Service, and a driver; loopback-only explicit router connection, no application-managed reconnect/retries/replay, and visible demonstration failure with configurable five-second per-wait deadlines. Define readiness, outage observations, and cleanup within those bounds.

How does the LA-CAL server map OWP subscriptions and publications onto Zenoh and back, carrying OMS JSON inside the settled Protobuf envelope for the chosen lifecycle exchange?

Decide topic/key-expression mapping, subscription identity and cleanup, publisher self-delivery, duplicate/loop avoidance, ordering and reliability expectations, backpressure, and the point at which OWP publication acknowledgment is sent. Apply the [encoding decision](04-schema-protobuf-mapping.md#answer), including its validation boundaries; settle error reporting and delivery consequences. Define what happens when the ASB is unavailable without conflating ASB state, OWP connection state, and Service lifecycle state. The demonstration must traverse Zenoh rather than be satisfied by a local echo.

## Comments

- 2026-10-02, routing discussion Q1: User asked for routing alternatives and the pros/cons of including the receiver in the Zenoh key. Topic/key mapping remains open.
- 2026-10-02, Q2: User accepted one Zenoh subscription per active topic with fan-out to matching OWP subscriptions, including the publisher's subscription. Delivery must use the actual Zenoh path, never a direct local echo; forwarding received Zenoh data to OWP must not republish it into Zenoh. Exact subscription keys depend on the unresolved routing choice. Q3 delivery expectations remain unanswered.
- 2026-10-02, Q1 follow-up: User proposed a message-type-first key family: `la-cal/uci/002.5.0/ServiceLifecycleCommand/<id>` for scoped commands and the same base without the ID for unscoped commands, keeping the payload schema unchanged and permitting all-command or ID-scoped listening. This is a viable alternative to a `to/<id>/<message-type>` layout. Official Zenoh key-expression documentation confirms that `<base>/**` includes the base and descendants, exact `<base>` does not include descendants, and `<base>/*` matches one additional segment. The unscoped delivery meaning and OWP-facing subscription mapping remain to be settled. Source: https://zenoh.io/docs/manual/abstractions/#key-expression
- 2026-10-02, processor clarification: User asked whether the affected Service must read and act on the command. In the resolved self-RESTART exchange it does, because it is also the processor. Effecting/processing and affected roles can differ; naming an affected Service is not itself an ASB routing rule. No manager execution mechanism was selected.
- 2026-10-02, Q1 simplification: User chose to start with OWP topics named only by message type, without IDs. Do not introduce ID-scoped or `all` OWP topics. Whether to also use only unsuffixed Zenoh keys in the first draft remains to be confirmed.
- 2026-10-02, Q3: User accepted live, non-durable delivery with no stored history, replay, application-level delivery acknowledgment, or duplicate suppression. Publication acknowledgment alone is not receipt or command completion evidence; expected observations remain subject to the resolved deadlines.
- 2026-10-02, Q4: User confirmed unsuffixed Zenoh keys for the first draft, one exact key per message-type-only OWP topic. Optional-ID routing is deferred; payload identities govern application targeting.
- 2026-10-02, Q5: User confirmed publication `+OK` after OMS JSON/schema validation and successful completion of the Zenoh publication operation, not recipient receipt or command completion. Known publication failures must not receive `+OK`.
- 2026-10-02, Q6: User delegated error-policy details: follow the specification as closely as possible, and use the simplest behavior where it leaves a choice. Apply standard OWP errors for rejected publications; report and discard invalid inbound ASB samples; detected ASB failure fails the demonstration without recovery.
- 2026-10-02, Q7: User conditionally accepted a bounded outbound queue per OWP connection with visible overflow and closure of the slow connection, provided it is simple to implement. Use an ordinary bounded channel and nonblocking enqueue; exact capacity belongs to the component decision. Do not add complex backpressure machinery for the first demo.
- 2026-10-02, Q8: User requested a simpler explanation of why the proposed server uses separate publish and receive Zenoh sessions. Router-path enforcement and the proposed subscription-readiness milestone remain unconfirmed.
- 2026-10-02, Q9: User accepted SDK-managed reconnection attempts in the background, provided a detected outage still fails the demonstration and triggers cleanup. No application reconnect/retry/replay or run resumption is required. This clarifies the earlier operating boundary; it does not make recovery part of acceptance.
- 2026-10-02, router-traversal challenge: User questioned whether proving a physical router hop is necessary or merely tests Zenoh capabilities. The earlier proposal of two sessions strengthens the agreed actual Zenoh/Protobuf-path requirement into forced router traversal. Simpler candidate: one Zenoh session using actual publication/subscription callbacks and Protobuf encode/decode, accepting SDK-local delivery while retaining the configured router deployment. No direct OWP echo or lifecycle shortcut. This interpretation awaits confirmation; readiness and failure evidence must be framed accordingly.
- 2026-10-02, simplified-path confirmation: User confirmed one Zenoh session with actual adapter encode/publish/subscribe/decode behavior; SDK-local delivery is acceptable and physical router traversal is not required for first-draft evidence. User placed two Services using two LA-CAL Servers through a router in a second phase, outside this map's first-draft destination. The previously proposed two-session remote-only topology and remote-matching readiness gate are superseded. Simple subscription-readiness details remain for final confirmation.
- 2026-10-02, readiness explanation: User asked what subscription `+OK` means, then challenged whether a normal LA-CAL would behave that way. OWP's specified positive acknowledgment must be distinguished from the project-specific choice of when/how to create Zenoh subscribers. The final subscription-readiness proposal remains unconfirmed pending clarification of the objection.
- 2026-10-02, normative subscription-scope check: User asked whether `SUB` success means subscription at the server or the bus. Official LA-CAL Rev B §§5.1.1.1.1.2–.3 and 5.1.1.1.3.3/.3.7 define successful operation completion with matching-message delivery, not a pending local registration or a prescribed remote ASB confirmation. Sections 4.4–4.7 place the server within the ASB and abstract middleware/topic implementation; no separate server-versus-bus acknowledgment milestones are specified. If required infrastructure prevents fulfillment, §5.1.1.1.1.3 permits `Internal-Error` or connection failure. These passages were verified in the official DOCX. Simplest candidate remains: active Zenoh listener plus installed OWP fan-out before `+OK`, without a special router acknowledgment; this readiness policy awaits user confirmation.
- 2026-10-02, implementation-evidence check: User proposed that the acknowledgment milestone depends on whether the server relays through an existing shared receive path or creates bus resources for each subscription, and asked what the spec/examples suggest. LA-CAL leaves internal mapping open; generic CAL Rev L §4.8 explicitly permits different approaches. SLEET creates one shared in-process Router (`sleet/src/main.rs:438–447`), registers connection-owned subscriptions synchronously under a lock (`sleet/src/router.rs:106–160`), and only then sends verbose `+OK` (`sleet/src/server.rs:538–574`). Its awaited registration is not merely a queued command and is not an example of a remote broker handshake. Candidate simplification: install the three fixed Zenoh listeners at server startup and make OWP SUB/UNSUB local routing-table operations; retain listeners until server shutdown. This replaces on-first/last-subscriber listener lifetime only if confirmed by the user.
- 2026-10-02, wildcard-listener question: User asked whether the three listeners merely reflect the selected message subset and why not use a wildcard. They do reflect the subset, not a LA-CAL requirement. A single startup Zenoh subscriber on `la-cal/uci/002.5.0/*` matches each direct message-type key while preserving exact, unsuffixed publication keys. Supported-message validation and OWP allowed topics remain restricted to the settled three types; unsupported or mismatched inbound samples must not be forwarded. This simpler listener topology awaits confirmation and would supersede the earlier one-listener-per-active-topic choice.
- 2026-10-03, final confirmation: User confirmed one startup wildcard listener, explicitly scoped to the supported UCI profile. This settles shared-listener readiness and supersedes the earlier per-active-topic listener lifecycle. The answer below records the complete contract; two Services using two LA-CAL Servers through a router remain a second-phase effort.
- 2026-10-03, topology resolution: The first demo now has two Services and a driver using one shared server, explicitly exercising multiple clients. Driver → orchestrator START and orchestrator → driver status use the actual adapter. Retain SDK-local delivery and defer multiple CAL Servers/forced physical router traversal; application processor and affected identities now differ.

## Investigation context

- Official LA-CAL Rev B §§5.1.1.1.1.3 and 5.1.1.1.3.7–.8 define operation acknowledgments and permit `Internal-Error` or WebSocket connection failure for underlying-infrastructure errors. PUB/SUB/UNSUB validation and state errors are listed in §§5.1.1.1.3.2–.4. Shutdown §5.1.1.1.1.4 disposes of every subscription on the closed connection. These passages were checked in the official DOCX; the standard does not define a Zenoh acknowledgment or readiness milestone.
- Zenoh 1.10.1 can deliver publications to subscribers in the same session without router traversal. Separate publish/receive sessions and remote-only filters were investigated as a way to force router traversal, then explicitly dropped. The selected contract accepts session-local delivery. Source: https://docs.rs/zenoh/1.10.1/zenoh/sample/enum.Locality.html . This verifies capability, not the version selection reserved for the component ticket.
- A publisher's matching status/listener reports whether at least one matching subscriber exists, not delivery or individual subscriber identity. A remote-matching readiness gate was investigated and dropped with the forced-router topology; it is not required by the selected startup-listener design. Source: https://docs.rs/zenoh/1.10.1/zenoh/pubsub/struct.Publisher.html#method.matching_status .
- Zenoh 1.10.1 can attempt transport reconnection internally; disabling startup connection retries is insufficient to prohibit those attempts. The user accepted SDK-managed attempts, while retaining visible run failure on detected outages. Source: https://github.com/eclipse-zenoh/zenoh/blob/1.10.1/zenoh/src/net/runtime/orchestrator.rs .

## Answer

### Namespace, routing, and listener ownership

Use the fixed [OMS 2.5 / UCI 002.5.0 profile](01-standards-profile.md#answer), with exact OWP topics and exact Zenoh publication keys:

| OWP topic and message type | Zenoh publication key |
| --- | --- |
| `ServiceLifecycleCommand` | `la-cal/uci/002.5.0/ServiceLifecycleCommand` |
| `ServiceLifecycleCommandStatus` | `la-cal/uci/002.5.0/ServiceLifecycleCommandStatus` |
| `ServiceStatus` | `la-cal/uci/002.5.0/ServiceStatus` |

No Service-ID suffixes, receiver-qualified topics, or OWP wildcard topics are introduced. Targeting, sender identity, and command correlation remain in the JSON and application checks. Apply the two STOP interactions' distinct processor/affected identities from [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer). Naming an affected Service does not make it the processor or introduce ID-based bus routing.

The LA-CAL Server opens one Zenoh session connected explicitly to the local loopback router, within the existing no-external-discovery deployment bounds. At startup, establish one shared subscriber with key expression:

```text
la-cal/uci/002.5.0/*
```

The wildcard matches one message-type segment within the supported UCI version namespace, not every profile or an ID-scoped descendant. It does not extend payload support beyond the three types and their required schema dependency closure. Check the actual received key against the allowed mapping and check its message type against the JSON. Report and discard unsupported keys/types or mismatches.

Keep this subscriber for the server's lifetime, even when there are no OWP subscriptions. OWP SUB/UNSUB change local connection-owned routing entries, not Zenoh connections or subscriber lifetimes. This supersedes the initial one-Zenoh-listener-per-active-topic proposal.

### Readiness and acknowledgments

Before accepting OWP clients, establish the configured Zenoh session/router connection and startup listener. Failure or expiry of the configured readiness wait fails the run. No remote subscription-matching gate, probe message, or special router acknowledgment is required.

- **SUB:** validate the OWP request, allowed topic/message pairing, and subscription-ID availability; install the local routing entry before successful acknowledgment. `+OK` confirms completed subscription setup against the existing receive path, not a pending registration request.
- **UNSUB:** remove the local subscription before successful acknowledgment.
- **PUB:** validate the OMS JSON against the selected schema/subset and topic/type mapping, encode the original JSON bytes in the settled envelope, and complete the Zenoh publication operation before successful acknowledgment. Known validation or operation failures must not receive `+OK`.
- Successful operations send `+OK` only on verbose connections, as OWP specifies; errors are reported on both verbose and non-verbose connections.

Publication `+OK` is not a router receipt, subscriber-delivery receipt, command acceptance, or lifecycle completion. Do not assume `+OK` arrives before a matching `MSG`. The test observer's subscriptions must be successfully installed before the Service emits its first state report; retain the lifecycle decision's bounded event waits rather than fixed sleeps.

### Delivery, validation, and loop prevention

Use the real adapter path:

```text
OWP PUB → JSON/schema validation → Protobuf encode → Zenoh publish
  → Zenoh subscriber → Protobuf decode → JSON/schema validation → OWP MSG
```

Preserve accepted JSON bytes exactly in both directions, following the [encoding contract](04-schema-protobuf-mapping.md#answer). Fan out each valid received sample to every matching ordinary OWP subscription, including the publishing connection's subscriptions. Subscription IDs are local to the OWP connection; different connections may reuse an ID. A connection with multiple matching subscriptions receives a corresponding MSG for each one.

Never forward OWP publications directly to OWP recipients or call the lifecycle handler as a publication shortcut. Conversely, forwarding a received Zenoh sample to OWP must not republish it to Zenoh. No duplicate-message cache is required; distinguish forwarding to multiple subscriptions from retransmitting a sample into the ASB.

Zenoh's own session-local delivery is acceptable. Protobuf encode/decode and actual Zenoh publication/subscriber handling must still execute for both STOP commands, both replies, and both Services' reports. The first draft uses two Services with one shared CAL Server, not a physical-router-hop success criterion. Separate CAL Servers communicating through the router remain later work.

### Delivery bounds, failures, and cleanup

Delivery is live and non-durable: no retained history, replay, application retries, subscriber receipt protocol, exactly-once guarantee, or duplicate-execution guarantee. There is no total arrival-order guarantee across message types; establish lifecycle order from local transition events and verify that the corresponding reports arrive, as already decided.

Use the standard OWP error names for malformed fields, disallowed topics, duplicate/inactive subscription IDs, and invalid publications. Rejected publications do not by themselves close an otherwise valid connection. Treat unsupported payload types and topic/type mismatches as `Invalid-Message` with explanatory details under the project's configured-format acceptance policy. Report and discard malformed envelopes, invalid JSON/schema payloads, and unsupported/mismatched inbound Zenoh samples; never forward them as valid messages.

An infrastructure failure that prevents operation fulfillment produces `Internal-Error` or connection failure under the standard's provisions; detected ASB loss fails the demonstration and triggers cleanup. SDK-managed reconnect attempts may occur, but the application does not retry/replay or resume a failed run. OWP connection status, ASB availability, and OMS lifecycle state remain distinct; do not invent lifecycle transitions or fabricate a successful command reply for an outage. A missed required event fails by deadline even if publication previously received `+OK`.

Use a simple bounded outbound queue per OWP connection. Nonblocking enqueue overflow is reported and closes the slow connection rather than silently dropping messages or blocking every client. Dispose of that connection's subscriptions. Queue capacity and execution/timeout mechanics belong to the [component-interface decision](06-rust-component-boundaries.md); avoid elaborate backpressure machinery.

UNSUB removes only the referenced connection-owned entry. Connection closure removes all entries for that connection. Neither tears down the shared startup Zenoh subscriber. Server shutdown disposes of its OWP connections, shared subscriber, and Zenoh session. Required waits retain the configurable five-second default; library connection health and successful `put` must not be mistaken for end-to-end success.

### Evidence and source grounding

The [acceptance decision](08-conformance-acceptance-evidence.md) must include actual adapter-boundary encoding/decoding and publication/subscription observations, JSON-byte fidelity, correlated command/status receipt, shared-listener/local-registration behavior, subscription cleanup, invalid-input rejection, and visible required-operation/deadline failures. Disable the adapter's publication or receive/forwarding path to show that the demonstration cannot succeed through a hidden direct echo. Physical router traversal is not the first-draft success criterion; router-outage handling is separate failure evidence.

Normative passages were verified in official `20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.docx`:

- §§4.4–4.7: server/ASB relationship and configurable middleware/topic mappings.
- §§5.1.1.1.1.2–.4: subscription delivery, successful-operation acknowledgments, infrastructure errors, and connection-shutdown cleanup.
- §§5.1.1.1.3.2–.4, CERT LACAL-000008/000009/000010: PUB/SUB/UNSUB behavior and operation errors.
- §§5.1.1.1.3.6–.8, CERT LACAL-000012/000013/000014: MSG, +OK, and -ERR.
- §6.1.5: OMS JSON validity against the applicable XML Schema.

These specify functional subscription success, not a per-client underlying bus connection or remote broker acknowledgment. The single startup wildcard listener and publication acknowledgment milestone are explicit project choices.

SLEET is implementation evidence only: `sleet/src/main.rs:438–447` creates a shared in-process router; `sleet/src/router.rs:106–160` completes connection-owned subscription installation before returning; `sleet/src/server.rs:538–574` sends verbose +OK afterward. Its middleware and best-effort queue behavior are not adopted as normative requirements. Zenoh's key-expression semantics are documented at https://zenoh.io/docs/manual/abstractions/#key-expression . Exact Rust dependency versions remain for the component-interface decision.
