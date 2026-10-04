# Investigate an official UCI schema-to-Protobuf generator

Parent: ../map.md
Labels: wayfinder:research
Type: research
Status: resolved
Assignee: OpenCode research agent
Research branch: research/uci-protobuf-generator
Research worktree: /var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-uci-protobuf-research
Research asset: ../research/uci-protobuf-generator.md

## Question

Do the official OMS/UCI sources provide or endorse a generator from the released UCI XSD to Protobuf definitions, especially for UCI 2.6? If not located, what primary-source evidence establishes the limits of that search, and what maintained generic XSD-to-Protobuf tools could be evaluated instead?

Check delivered local release artifacts and official public distribution/repository/documentation sources. Distinguish official OMS/UCI tooling, first-party Protobuf tooling, and third-party generators. Record version support, provenance, licensing, availability, and material mapping limitations where verifiable. Do not treat `protoc` generating Rust from an existing `.proto` as an XSD-to-Protobuf generator.

This research discovers facts and options. Choosing the generator and defining the mapping are left to the mapping decision. A failure to locate a public official generator is not proof that no private or restricted tool exists.

## Answer

Resolved 2026-10-02 after primary-source investigation by the research agent.

No official or officially endorsed UCI XSD-to-Protobuf generator was located in the delivered UCI 2.6 artifacts or public official distribution sources checked. An official UCI NOSC describes a Consortium Protobuf-definition effort, but delivers no generator or confirmed UCI 2.6 mapping. Private/restricted availability is unknown.

Entur schema2proto, XSDFormer and Avrotize are maintained generic candidates, not official UCI tools. Their source-level choice, validation, type-resolution, presence, precision and evolution limitations are documented; none was executed against UCI 2.6. `protoc` and Rust binding generators take existing `.proto` definitions and do not resolve XSD translation.

See the [primary-source findings and bounded search record](../research/uci-protobuf-generator.md). Generator selection and the exact mapping remain with [Choose generation tooling and define the UCI-to-Protobuf mapping](04-schema-protobuf-mapping.md).
