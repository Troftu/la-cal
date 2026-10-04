# Official UCI schema-to-Protobuf generator investigation

Date: 2026-10-02

Ticket: [Investigate an official UCI schema-to-Protobuf generator](../issues/02-official-protobuf-generator.md)

## Finding

No official or officially endorsed UCI XSD-to-`.proto` generator was located in the delivered release artifacts or public official sources checked. This is a bounded search result, not proof that private, restricted, unpublished, or differently named tooling does not exist.

The official [UCI NOSC Rev1, Intro to 3.0](https://github.com/open-arsenal/uci/blob/78eb61b6112c8bffa40820c33124b57787fc5bd9/UCI-NSC-001_Rev1_Intro_To_3.0_NOSC-1.docx), section 3, describes an effort to make a Protobuf definition for UCI and warns that vendor encodings may not be compatible with the eventual consortium version. Section 4 records Consortium approval on July 30, 2026. The notice supplies neither a generator nor a mapping algorithm, repository, or UCI 2.6 compatibility statement. A definition effort does not necessarily imply an XSD generator.

## Sources and search boundary

- UCI official local/public tree: commit `78eb61b6112c8bffa40820c33124b57787fc5bd9`, [pinned distribution](https://github.com/open-arsenal/uci/tree/78eb61b6112c8bffa40820c33124b57787fc5bd9), originating at `https://gitlab.com/open-arsenal/uci/standard.git`.
- OMS official local/public tree: commit `726272bd0390982a759c91a9cf4e13b81c2b510b`, [official documents](https://github.com/open-arsenal/oms/tree/726272bd0390982a759c91a9cf4e13b81c2b510b/docs_official), originating at `https://gitlab.com/open-arsenal/oms/standard.git`.
- Official DOCX XML and ZIP members were inspected directly, rather than relying on unofficial Markdown conversions. The UCI 2.6 VDD Rev11, Standard Rev10, Schema Style and Design Specification Rev6, Normalized Interface Specification Rev2, schema and template archives, Starter Kit Tools VDD, Prisma archive, and NOSC were checked. OMS official DOCX text was searched for generation/Protobuf references.
- The UCI schema ZIP contains three XSDs, a disclaimer and VDD; no `.proto` or generator. Its XSD roots identify version `002.6.0`. Archive SHA-256: `073dafd85f0a1cf2f1668f75a73aca6a5af28895e0b4645186ed7b23014c145e`.
- The [Starter Kit Tools VDD](https://github.com/open-arsenal/uci/blob/78eb61b6112c8bffa40820c33124b57787fc5bd9/UCI%20Starter%20Kit%20-%20Tools/02_UCI_Tools_VDD.pdf), sections 3.1–3.4, lists Schema Checker, Schema Version Comparison, Message Content Validation and Prisma. The first three tool ZIPs were absent from the inspected tree; their internals were not examined. Prisma is a visualizer, not the requested generator.
- Public checks included the Open Arsenal GitHub repository inventory, complete recursive UCI/OMS/SK-CAL trees, authenticated code searches for `protobuf`, `schema2proto` and `xsd2proto` within the organization, GitLab UCI/OMS project inventories, and the standard/Prisma branch, tag, release and package endpoints. Public release/package checks found no generator asset.
- Indexed `protobuf` hits in the official example repositories concerned existing application/gRPC schemas and dependencies, not an identified UCI XSD frontend. The [Squall build](https://github.com/open-arsenal/ams-gra-hello-world-sk-sensors-squall/blob/b1015728f904c799fa0c07489fce48e78f67845f/couloir/build.rs) compiles existing `.proto` files.
- Public [SK-CAL](https://github.com/open-arsenal/oms-sk-cal/tree/d81f7768a42f8600f64cbdd0a54a7bfa59226fbf) delivers OMS 2.5 Java CAL/XML JARs; their internals were not inspected. OMS CAL API compiler and SchemaToXLS references do not establish a Protobuf backend.
- Not covered: all historical commits, restricted Consortium portals, unlisted government/vendor distributions, container/JAR internals, or unpublished deliverables. No Consortium representative was contacted.

## Generic candidates verified at their primary sources

None is identified as OMS/UCI-endorsed, and none was executed against UCI 2.6 during this investigation. Versions below are research observations, not selected dependencies.

### Entur schema2proto

- [Pinned repository](https://github.com/entur/schema2proto/tree/b7b390ae26b991591e1575992af8003fa1bfcd29), version `1.143.0`, October 2, 2026; published Maven metadata agrees. Direct Java/Maven XSD-to-proto3 converter using modified XSOM and Square Wire, with namespace separation, inheritance options and field-evolution checks.
- License: root EUPL 1.2; POM describes modifications; bundled XSOM has a separate BSD-style license. Generated-output terms were not established.
- [SchemaParser](https://github.com/entur/schema2proto/blob/b7b390ae26b991591e1575992af8003fa1bfcd29/schema2proto-lib/src/main/java/no/entur/schema2proto/generateproto/SchemaParser.java) and its expected-output fixtures show ordinary XSD choices becoming independent fields rather than exclusive alternatives.
- [ValidationRuleFactory](https://github.com/entur/schema2proto/blob/b7b390ae26b991591e1575992af8003fa1bfcd29/schema2proto-lib/src/main/java/no/entur/schema2proto/generateproto/ValidationRuleFactory.java) disables particle occurrence-rule generation and contains inherited-facet/attribute validation gaps. Complete UCI polymorphism support was not established.

### XSDFormer

- [Pinned repository](https://github.com/populationgenomics/xsd-former/tree/af7608baa51d46ea8f1562c2a6b276aaa379e2f9), version `1.4.0`, August 21, 2026. MIT; Python 3.11+, `xmlschema`, lxml and Protobuf tooling. Its README explicitly makes full XSD support a non-goal.
- [XSD processor](https://github.com/populationgenomics/xsd-former/blob/af7608baa51d46ea8f1562c2a6b276aaa379e2f9/src/xsdformer/xsd/xsd.py) has limited primitive mappings and no established complete UCI abstract-type strategy.
- [Protobuf emitter](https://github.com/populationgenomics/xsd-former/blob/af7608baa51d46ea8f1562c2a6b276aaa379e2f9/src/xsdformer/protobuf/generator.py) supports optional scalar presence and limited `oneof` emission; complex/repeated alternatives are flattened. XSD facet validation and stable numbering across schema changes are not supplied by simple traversal-based generation.

### Avrotize

- [Pinned repository](https://github.com/clemensv/avrotize/tree/0848ff37ad91aaf2b48e8f700b2cde79d2102b56), version `3.9.3`, September 21, 2026. Python 3.10+; indirect XSD-to-Avro-derived-model-to-proto3 path. Root LICENSE says Apache 2.0, while README/package metadata say MIT; the discrepancy remains unresolved.
- The inspected [XSD converter](https://github.com/clemensv/avrotize/blob/0848ff37ad91aaf2b48e8f700b2cde79d2102b56/avrotize/xsdtoavro.py) does not resolve includes/imports or complex-content extensions. Those omissions directly affect UCI's included security schema and message inheritance.
- The [Protobuf converter](https://github.com/clemensv/avrotize/blob/0848ff37ad91aaf2b48e8f700b2cde79d2102b56/avrotize/avrotoproto.py) has order-dependent numbering and configurable optionality. Its intermediate temporal mappings are not automatically lossless for UCI precision.

## Mapping implications

The official [UCI 2.6 schema archive](https://github.com/open-arsenal/uci/blob/78eb61b6112c8bffa40820c33124b57787fc5bd9/UCI%20Release%20Documentation%20-%20UCI%20Schema/03_UCI-STD-002_Rev6_UCI_Schema_v2_6-CDRL.zip) includes inheritance of message headers/security fields, abstract types, choices, optional elements, enumerations, facets and precise temporal types. The lifecycle message dependency closure must be examined, not merely the two root command/status elements.

The mapping decision must address inclusion/type resolution, root-message identity, inheritance, polymorphism, choice exclusivity, absent-versus-default values, enum lexemes/numeric identities, precision, validation and stable field numbering. A successful Protobuf decode is not evidence of UCI schema validity.

Official [Protobuf compiler documentation](https://protobuf.dev/overview/), [Rust generation documentation](https://protobuf.dev/reference/rust/rust-generated/), and [`prost-build`](https://github.com/tokio-rs/prost/blob/master/prost-build/README.md) describe generation from existing `.proto` files. They are downstream tools, not an answer to XSD translation. A Java/Python generation step need not impose that runtime on the Rust application.

## Handoff

The discovery ticket can close with these facts. Generator selection, source-subset scope, mapping rules, fidelity evaluation and release-profile selection remain decisions for the existing tickets. The official future-definition effort is a compatibility consideration, not a requirement to wait for a future release.

The full investigation, including individual API results, source locations and schema-construct counts, is an uncommitted asset in the `research/uci-protobuf-generator` worktree:

`/var/folders/_9/z94g70y52kq4v1ym86bkv_sw0000gn/T/opencode/la-cal-uci-protobuf-research/research/uci-protobuf-generator.md`

This repository-local report preserves the core findings and citations independently of that temporary worktree.
