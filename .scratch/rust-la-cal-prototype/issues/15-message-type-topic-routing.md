# 15: Apply message-type topic routing and its tests

**What to build:** The project-owned SLEET-derived server implements the selected three-root topic contract with passing configuration, routing and OWP socket checks through its existing delivery path. Topic policy is established before changing the delivery mechanism.

**Blocked by:** 14 — Establish a working project-owned SLEET baseline.

**Status:** ready-for-agent

Source: [Approved prototype spec](../spec.md), especially the supported profile, topic mapping and server authorization decisions.

- [ ] Restrict the supported OWP topics to `ServiceLifecycleCommand`, `ServiceLifecycleCommandStatus` and `ServiceStatus`, each paired with its identically named UCI 002.5.0 global root. Service-ID suffixes, receiver-qualified aliases and OWP wildcard topics are outside this contract; application targeting remains in payloads and application checks.
- [ ] Project configuration and startup establish this three-root supported surface and its participant/topic permissions. Missing optional upstream bindings cannot make unsupported roots or mismatched topic/root pairs acceptable.
- [ ] PUB checks participant/topic authorization and the supported topic/payload pairing. Unsupported publication roots and topic/payload mismatches return the selected `Invalid-Message` error with useful details; authorization failures retain the applicable OWP error behavior. Known rejection never receives successful acknowledgment, and ordinary publication rejection leaves a valid connection usable.
- [ ] SUB checks authorization, supported message/topic pairing and subscription-ID availability, rejecting invalid requests with applicable OWP errors. Registration completes before successful acknowledgment; UNSUB removes its connection-owned registration before successful acknowledgment.
- [ ] Matching ordinary subscriptions receive delivery independently, including publisher self-subscriptions and repeated registrations. Different connections can reuse a subscription ID. Publications do not leak into subscriptions for other topics or message types.
- [ ] Each topic-scoped subscription group receives one matching member's delivery while matching ordinary subscriptions still receive fan-out. Independent groups and topics remain isolated; no particular member-selection algorithm is required.
- [ ] Adapt relevant copied configuration, routing and OWP socket tests to the selected contract, adding focused regression cases where needed for invalid pairings, unsupported roots, authorization, topic isolation, repeated/self subscriptions, group isolation, unsubscribe and connection-owned cleanup.
- [ ] Execute the selected tests against project-owned code and record results and coverage limits. Preserve useful baseline coverage and record topic-policy source adaptations in the modification notice.
