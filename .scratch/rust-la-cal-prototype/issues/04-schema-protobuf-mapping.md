# Define the OMS JSON-in-Protobuf encoding contract

Parent: ../map.md
Labels: wayfinder:grilling
Type: grilling
Status: resolved
Blocked by: 01, 02, 03

## Question

How will the prototype preserve OMS JSON payloads inside a project-defined Protobuf envelope over Zenoh, under the selected OMS 2.5 / UCI 002.5.0 profile?

Define envelope fields and their stable numbers, payload fidelity, profile/version identification, validation boundaries, and unknown/unsupported data behavior. Support ServiceLifecycleCommand, ServiceLifecycleCommandStatus, and ServiceStatus with their required schema dependencies. Topic/key routing and delivery behavior belong to the ASB contract ticket. The UCI payload remains OMS JSON; this is not a typed UCI Protobuf mapping.

Specify meaningful envelope and payload round-trip evidence, and the `.proto`-to-Rust generation step. This ticket settles design; it does not implement the encoding.

## Comments

- 2026-10-02, Q1: User chose the necessary subset: ServiceLifecycleCommand, ServiceLifecycleCommandStatus, and ServiceStatus, with their complete required schema dependency closure. Generator strategy remains open; the user proposed reusing the OWP JSON payloads as a first draft.
- 2026-10-02, Q2: User confirmed JSON inside Protobuf over Zenoh as the prototype destination. Typed schema-derived UCI Protobuf generation and generic-generator evaluation are deferred to a separate effort. The map destination and notes now reflect this choice; envelope details remain open.
- 2026-10-02, Q3: User chose validation followed by byte-for-byte preservation of the original OMS JSON UTF-8 payload. Parsing for validation must not replace the original payload with a reserialized representation.
- 2026-10-02, Q4 discussion: User challenged the need for envelope metadata when relevant message data is already in the JSON. The proposed minimal alternative is a single bytes field, with the supported profile fixed by deployment configuration rather than extra envelope fields; confirmation remains pending.
- 2026-10-02, Q4 confirmation: User chose the simplest first-draft envelope: proto3 OmsJsonEnvelope with only bytes oms_json = 1. No duplicated type, profile, or version fields. The selected profile is fixed for the deployment; the JSON remains the source of message content and identities.
- 2026-10-02, Q6: User selected prost and prost-build for the project-owned envelope's Rust bindings.
- 2026-10-02, Q5: After reviewing validation responsibilities, user chose validation on receiving and before sending for the server/adapter path. Accepted payloads retain their original bytes. Whether the example Service also performs schema validation is the remaining discussion.
- 2026-10-02, Q7: User chose no runtime schema validation in the example OWP client/Service for now. The Service still parses received messages and enforces the resolved lifecycle contract's application checks; schema validation remains on the server/adapter path.
- 2026-10-02, final confirmation: User confirmed the complete encoding contract, including round-trip and invalid-input evidence, ordinary unknown-field handling, and pinned Rust generation tooling.

## Context

The [generator investigation](02-official-protobuf-generator.md) found no official generator in the checked sources. Its [primary-source report](../research/uci-protobuf-generator.md) remains historical context. The user subsequently chose JSON-in-Protobuf instead of typed UCI generation, so candidate evaluation is outside this effort. The [profile decision](01-standards-profile.md) records authoritative UCI 002.5.0 provenance. This project-defined wrapper does not claim compatibility with a future Consortium Protobuf encoding.

## Answer

### Encoding and profile

Carry complete OMS JSON payloads inside a project-owned Protobuf envelope over Zenoh:

```proto
syntax = "proto3";

message OmsJsonEnvelope {
  bytes oms_json = 1;
}
```

Field number 1 remains permanently assigned to oms_json. No envelope message-type, profile, or version fields are required. Use the fixed deployment profile from the profile decision: OMS 2.5 / UCI 002.5.0 / LA-CAL Rev B / OWP 1.0. This is Protobuf framing of JSON, not typed UCI Protobuf. XSD-to-Protobuf generation is deferred to a separate effort.

Support ServiceLifecycleCommand, ServiceLifecycleCommandStatus, and ServiceStatus with their complete required schema dependency closure, including headers and security markings. The JSON remains the source of message identity, Service identities, and command correlation. Routing belongs to the ASB contract, not extra envelope fields.

### Fidelity and validation

Preserve accepted OMS JSON UTF-8 payload bytes exactly; parsing for validation must not replace them with reserialized JSON. Wrap the payload only, not OWP PUB/MSG framing.

- OWP to Zenoh: validate incoming publications, then wrap and send the original payload bytes.
- Zenoh to OWP: decode the envelope and validate the extracted JSON before forwarding through OWP.
- Reject malformed Protobuf, empty payloads, invalid UTF-8/JSON, unsupported message types, and schema-invalid payloads. Ignore unknown Protobuf envelope fields using ordinary decoder behavior; they do not bypass payload validation.
- The example client/Service performs no runtime schema validation for this first draft. It still parses received messages and performs the application checks required by the lifecycle decision, including targeting, supported commands, state eligibility, and reply correlation. Validate its generated payloads against the selected schema in tests.

Error surfaces and delivery consequences remain for the ASB and component-interface decisions. No new retry, ordering, or delivery guarantees are implied by this encoding.

Official LA-CAL OMSC-SPC-013 Rev B §5.1.1.1.3.2 (PUB, CERT LACAL-000008) describes delivery of valid publications and Invalid-Message errors for nonconforming message formats. Section 6.1.5 defines OMS JSON validity by XML Schema assessment. Those passages were checked in the official DOCX. Bidirectional adapter validation and the fixed supported subset are explicit project choices; the envelope itself supplies no UCI validation.

### Generation and evidence

Keep the project-owned .proto as source of truth. Generate Rust bindings with prost and prost-build; pin their versions and the required Protobuf compiler toolchain. Exact versions and build integration belong to the component-interface ticket.

Acceptance evidence must include byte-for-byte JSON → Protobuf → JSON round trips for all three supported messages, generated Service payload schema validity, and meaningful negative cases for each rejected input category. Verify unknown envelope fields are tolerated without allowing an absent or invalid payload. Retain the lifecycle decision's proof of actual Zenoh traversal; an envelope-only unit round trip cannot establish that integration.
