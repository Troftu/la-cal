# Bound the prototype's deployment and failure behavior

Parent: ../map.md
Labels: wayfinder:grilling
Type: grilling
Status: resolved

## Topology update

Use [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer) for the current application/command contract. Native deployment, loopback endpoints, TLS exception, visible failure, and configurable five-second waits remain. START/process-launch and reusable runtime assumptions in earlier comments are historical. WASIp3 remains a deferred follow-up.

## Question

Under what concrete deployment and operating assumptions must this prototype work, and what behavior is required when a client, server, or Zenoh connection fails?

Settle local versus distributed processes, supported hosts, process/service multiplicity, configuration and identity setup, connection protection and authorization expectations, reconnection/recovery scope, and whether performance limits are acceptance conditions. Choose explicit prototype bounds rather than assuming production requirements. If the supported profile imposes requirements within those bounds, account for them in the conformance evidence.

## Comments

- 2026-10-02, Q1: User selected one-machine deployment with macOS and Linux support. The proposed topology uses separate example Service/OWP client, LA-CAL server, and Zenoh router processes.
- 2026-10-02, Q2: User said reconnection/recovery is unnecessary for the demonstrator. No automatic reconnect, retries, or replay are required; the proposed failure policy is visible failure with bounded waits and a fresh run.
- 2026-10-02, Q3: User selected no performance targets for acceptance.
- 2026-10-02, Q4: User chose local unencrypted connections unless the specification requires WebSocket encryption. The proposed bounds are loopback-only endpoints, explicit local Zenoh router connections, no external discovery, and no credentials. TLS support versus use remains to be clarified against the normative source.
- 2026-10-02, Q5: User accepted one server, one router, one example Service acting as commander/commandee, and a test observer; predefined Service/Mission UUIDs with environment overrides and consistent Service identity.
- 2026-10-02, Q6: User accepted visible run failure on invalid configuration, initialization failure, detected disconnect, or wait timeout, without invented success or recovery lifecycle. User selected a configurable default deadline of five seconds per required wait, replacing the proposed thirty seconds.
- 2026-10-02, TLS source check: Official LA-CAL Rev B §5.1 invokes RFC 6455 and CERT LACAL-000002 explicitly invokes its server-side requirements. RFC 6455 §3 defines ws and wss; §4.2.2 requires TLS handshake for HTTPS-port connections; §10.6 says implementations MUST support TLS and SHOULD employ it. Plain ws is therefore a defined transport, but omitting TLS capability requires an explicit conformance exception rather than assuming encryption is entirely optional.
- 2026-10-02, Q7: User explicitly deferred TLS capability for the first draft. Use loopback ws and unencrypted local Zenoh transport; record the RFC 6455 §10.6 TLS-support exception in acceptance evidence and do not claim complete WebSocket conformance.
- 2026-10-02, final confirmation: User confirmed the complete operating-boundary summary.
- 2026-10-03, subsequent ASB clarification: The user accepted SDK-managed reconnect attempts provided detected loss still fails the run, and accepted Zenoh session-local delivery as first-draft adapter evidence. Physical router traversal with two Services/two LA-CAL Servers is a second phase. Details live in the [ASB routing decision](05-zenoh-asb-contract.md#answer).
- 2026-10-03, topology/runtime resolution: User confirmed the START-only two-Service flow with one shared CAL server, external driver, OS process creation as command success, and simple process ownership/cleanup. Both Services use the reusable lifecycle runtime. la-cal and orchestrator are native; Wasm capability of other Services and the shared crates is deferred to a separate spike, not required for this native first demo.

## Answer

### Deployment and configuration

Support native macOS and Linux deployment on one machine. Run one Zenoh router, one shared `la-cal` server, `orchestrator`, `commandee-service`, and `demo-driver`. Both applications are running before the driver initiates the command exchange. The two Services and driver use independent OWP connections. Distributed deployment and additional application Services are outside first-draft scope. WASIp3 remains deferred to the [portability spike](../../wasip3-service-portability/issues/01-wasip3-service-runtime-spike.md).

Bind communication endpoints to loopback. Use plain ws WebSocket and unencrypted Zenoh TCP, with an explicit connection to the local router and no discovery beyond the machine. No credentials or TLS capability are required in the first draft. Keep ordinary OWP negotiation, schema/Service support checks, and message validation; absence of credentials is not permission to bypass protocol checks.

Provide distinct predefined demonstration Service identities and a configured Mission UUID, with environment overrides. Keep each participant's OWP/sender identity consistent. Use the effecting/affected and processor identities in [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer). Exact variables, endpoint defaults, driver/server identities, and launch/build details belong to the component-interface and acceptance decisions.

The commandee exits after the selected STOP reporting sequence. Harness launch ownership, process-exit observation, and remaining cleanup must be settled in the component-interface and acceptance decisions. Do not assume the orchestrator launched or owns commandee merely because it issues the downstream STOP. No extra process-management framework is required.

### Failures and deadlines

Invalid configuration, application initialization failure, detected communications disconnection, required-operation failure, or an expired required wait makes the demonstration fail visibly. Report the cause, never fabricate a successful lifecycle reply, and require a fresh run. No application-managed reconnect, command retries, replay, or lifecycle recovery is required. SDK-managed reconnect attempts are acceptable, but they do not resume a run after detected loss; see the subsequent [ASB clarification](05-zenoh-asb-contract.md#delivery-bounds-failures-and-cleanup). A timeout is failure of the demonstration, not evidence of a particular Service lifecycle transition or proof that a command was never executed.

Use the STOP completion milestones in [Choose a client-focused relayed STOP demonstration](11-client-focused-stop-demo.md#answer). Do not fabricate upstream success from a publication acknowledgment alone. Downstream rejection, timeout, and shutdown-failure response policy remains an interface/acceptance decision. Expected commandee closure after final reporting is normal completion, not an unexpected-disconnect failure; distinguish it from premature connection loss.

Use a configurable default deadline of **five seconds per required wait**, not five seconds for the entire run. Required waits include connection/readiness and expected demonstration events; these are bounded event waits, not fixed sleeps. No throughput benchmark or real-time latency guarantee is an acceptance condition. Operation-specific error surfaces, ASB readiness, acknowledgment timing, and orderly cleanup remain for the ASB and component decisions.

### Standards scope and TLS exception

Official LA-CAL OMSC-SPC-013 Rev B §5.1 uses RFC 6455; CERT LACAL-000002 explicitly requires its server-side requirements. RFC 6455 §3 defines ws and wss, §4.2.2 requires the TLS handshake for HTTPS-port connections, and [§10.6](https://www.rfc-editor.org/rfc/rfc6455.html#section-10.6) says implementations MUST support TLS and SHOULD employ it.

The user explicitly deferred TLS capability for this first draft. Record that exception in requirement-linked acceptance evidence and do not claim complete WebSocket conformance. The fixed local unencrypted deployment is an intentional prototype boundary, not an assertion that the specification has no TLS requirement. The map continues to target scoped OWP/LA-CAL evidence and lifecycle integration, not full OMS Service/Platform compliance.
