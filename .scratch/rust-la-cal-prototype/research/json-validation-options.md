# OMS JSON validation options

Date: 2026-10-03
Context: [Choose the Rust component interfaces and dependencies](../issues/06-rust-component-boundaries.md)
Status: source-inspected findings; no engine compilation or validation execution

## Pinned schema artifacts

The UCI 002.5.0 files exist in local Git objects of standards-and-references/uci-standard at revision 73a286fc4b881d9f328dbd64148525606a92c33a, not its current 2.6 checkout:

- 03_OAC-STD-002_RevE_UCI_Schema_v2_5/UCI_MessageDefinitions_v2_5_0.xsd
- 03_OAC-STD-002_RevE_UCI_Schema_v2_5/UCI_SecurityMarkings_v2_5_0.xsd
- 03_OAC-STD-002_RevE_UCI_Schema_v2_5/UCI_Versioning_v2_5_0.xsd

Main includes SecurityMarkings; neither imports another schema. Versioning imports Main, but Main does not depend on Versioning. Security documentation references controlled vocabularies without schema imports. Release version 002.5.0 is not an XML Schema language-version declaration. The pinned UCI Schema Style and Design Specification, GUID SCH-000156, recommends XSD 1.0; inspected constructs did not require XSD 1.1.

Read-only graph analysis from the three supported roots found 59 named types (26 complex, 33 simple), including optional fields and inheritance. Polymorphism through CannotComplyType.AssociatedID (ID_Type) broadens a conservative derived-type closure to 341 named types. These counts are analysis inputs, not coverage proof. Retaining the complete Main and SecurityMarkings files avoids prematurely pruning derived-type branches.

## OMS JSON mapping constraints

The lookup checked the official LA-CAL OMSC-SPC-013 Rev B DOCX, section 6.1:

- Sections 6.1.1–.3: one global-root member, namespace-aware names, objects for complex values, choices, inherited fields in the same object, JSON member order independent of schema sequence order.
- Section 6.1.2: repeating declarations use arrays even for one occurrence; singleton declarations do not. Optional arrays may be omitted or empty per examples.
- Section 6.1.4: JSON primitive categories depend on the XSD type. Integer/decimal use numbers, Boolean uses Boolean, and other relevant types use strings, with the stated UUID rule.
- Section 6.1.5: validity is assessed against the associated XML Schema infoset. Table 6.1-2 sets attributes/namespaces to empty sets and requires schema-valid child ordering.

An XML-backed validator still needs schema-aware JSON shape checks, primitive-category checks, name resolution, reconstructed inherited particle ordering, and precision-preserving number handling. Forward the original JSON bytes regardless of validation representation.

The investigation flagged mapping questions for verification/decision: Table 6.1-2 array wording versus conventional repeated-element mapping; $type versus xsi:type with the table's empty attribute set; number-token lexical forms for integer/decimal. Float Infinity/INF differences were identified but floats were outside the analyzed closure. Do not treat a generic JSON-to-XML utility as proven OMS JSON support.

## SLEET implementation evidence

Inspected revision e3c2d798a229766571ed7be1f5b28d2dcdb51868 under standards-and-references/sleet:

- sleet/src/schema/{parser,model}.rs: custom quick-xml schema parsing/model.
- sleet/src/{validator,facets}.rs: direct JSON validation, shape/occurrence/choice checks, inheritance, selected facets, integer ranges and $type derivation.
- scripts/generate_uci_types.py: Serde helper generation, not JSON Schema generation. Its default roots omit the two lifecycle-command roots.

Coverage gaps found by inspection: xs:whiteSpace collapse omitted from the model despite included security-marking types using it; dateTime/duration treated as strings without complete built-in lexical validation; namespaces reduced to local-name lookups; unsupported or failed patterns skipped with warnings; numeric facet comparisons use f64. SLEET is implementation evidence, not normative authority or complete validation proof.

## Candidate tooling

- [libxml 0.3.21](https://docs.rs/libxml/0.3.21/libxml/schemas/index.html): libxml2 XSD assessment after a faithful mapping. Requires Rust 1.88, native libxml2 headers/library, and bindgen/libclang per inspected build metadata. SchemaValidationContext is !Send/!Sync; dedicate ownership rather than moving it across async tasks. Pin the C library independently of the Rust crate. Candidate, not selected or verified.
- [exml 0.7.2](https://docs.rs/exml/0.7.2/exml/xmlschemas/context/struct.XmlSchemaValidCtxt.html): Rust libxml2 reimplementation, schema feature, Rust 1.85. README acknowledges incomplete implementation; context is !Send/!Sync and exposes unsafe operations. UCI coverage unverified.
- SLEET-style direct Rust validation: avoids XML mapping/C dependencies but needs the identified gaps closed and independent XSD assessment evidence.
- [jsonschema 0.58.5](https://docs.rs/jsonschema/0.58.5/jsonschema/): JSON Schema engine, not an XSD engine. No pinned OMS-aware JSON Schema artifact was found in inspected UCI/SLEET sources. Serde-generated schemas do not establish equivalence to UCI XSD.

## Verification still needed

Compile the complete pinned schemas with the candidate engine; establish the OMS mapping interpretation; validate all three roots including optional/security/derived-type cases; reject shape, primitive, choice, UUID, temporal, and range errors; check whitespace and namespace handling; demonstrate original-byte preservation and safe bounded validator ownership on macOS/Linux. No build, schema compilation, or executed validation was performed in this lookup.

## Follow-up: typed sending and full binding generation

Read-only inspection of scripts/generate_uci_types.py in the same SLEET revision found:

- The CLI exposes --schema, --generate-rust, --output, and --self-test, not --all or --messages. TARGET_MESSAGES is a hard-coded 14-root subset, excluding both lifecycle-command roots.
- discover_type_closure(index, target_messages=TARGET_MESSAGES) accepts arbitrary indexed global roots internally. Selecting all roots is feasible at this interface, but the current CLI uses only its default list.
- Output includes Serde structs, aliases/enums, and a root-level UciMessage wrapper. No generic message trait, profile metadata, or topic metadata is emitted.
- Polymorphic $type/derived-type dispatch is not emitted. Numeric mappings use f64/fixed-width integers; special float strings, complete facet enforcement, namespace/profile retention, and general inherited/nested content-model handling need further work.
- Read-only counts from pinned UCI 002.5.0 Main plus SecurityMarkings: 722 global roots, 4,612 named complex types, 945 named simple types. Counts do not prove complete generation, compilation, or valid encoding.

SLEET's sleet-client/src/lib.rs:221–239 already provides generic publish<T: Serialize + Sync>(topic, payload). Its checked-in 14-root UciMessage enum supplies the OMS JSON one-member root wrapper; serializing an arbitrary payload struct does not automatically supply that wrapper or schema validity.

A future generated typed-send contract needs exact global-root identity, complete OMS JSON serialization, pinned profile identity, and configured routing. XSD does not prescribe OWP topics; the current root-name/topic equivalence is this project's convention. Ordinary internal XSD types are not independently publishable global messages. Generated encoding remains distinct from schema validation and from the server's supported-message subset.

The fixed OWP operation grammar (LA-CAL Rev B section 5.1.1.1.3) is separate from UCI payload generation. Full Rust/OMS-JSON binding generation is not the deferred typed UCI Protobuf generation effort.
