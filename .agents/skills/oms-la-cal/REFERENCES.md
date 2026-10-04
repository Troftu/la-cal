# OMS LA-CAL Source Map

All paths are relative to the repository root. Read the branch relevant to the current question rather than loading every source.

## Release Baselines

- OMS 2.5 release inventory: `standards-and-references/oms-standard/docs_official/01_OMSC-VDD-001_RevM_VersionDescriptionDocument_DandD_v2_5.docx`
- UCI 2.6 release inventory: `standards-and-references/uci-standard/UCI Release Documentation - Standards Documents/01_UCI-VDD-001_Rev11_UCI_VersionDescriptionDocument_v2_6.docx`
- SLEET implementation overview: `standards-and-references/sleet/README.md`
- SLEET schema provenance and its pinned UCI 2.5 source: `standards-and-references/sleet/docs/schema-sourcing.md`

The UCI README has stale/conflicting metadata: its badge says 2.5, while the delivered documents and schema are 2.6. Use the delivered VDD and schema package as the release evidence.

## LA-CAL And OWP

Normative source:

- `standards-and-references/oms-standard/docs_official/20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.docx`

Searchable navigation copy:

- `standards-and-references/oms-standard/docs_markdown_unofficial/20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.md`
- Focus on section 5.1 for WebSocket and OWP 1.0 operations, and section 6.1 for OMS JSON.

Implementation references:

- OWP operation model, parser, formatter, and errors: `standards-and-references/sleet/sleet-types/src/owp.rs`
- Async client handshake, publish, subscribe, unsubscribe, and receive flow: `standards-and-references/sleet/sleet-client/src/lib.rs`
- Server WebSocket upgrade and per-connection state machine: `standards-and-references/sleet/sleet/src/server.rs`
- External contract summary: `standards-and-references/sleet/docs/contracts.md`
- Harness integration: `standards-and-references/sleet/docs/la-cal-harness-integration.md`

Verify at least subprotocol negotiation, first-operation `INIT`, `+OK`/`-ERR` behavior, `INFO`, `PUB`, `SUB`, `UNSUB`, `MSG`, identifier grammar, subscription cleanup, and OMS JSON validation against the normative specification.

## OMS Service Lifecycle

Normative source:

- `standards-and-references/oms-standard/docs_official/02_OMSC-STD-001_RevM_OMS_Standard_DandD_v2_5.docx`

Searchable navigation copy:

- `standards-and-references/oms-standard/docs_markdown_unofficial/02_OMSC-STD-001_RevM_OMS_Standard_DandD_v2_5.md`
- Focus on section 6.1.1 for Service state transition behaviors, section 6.3.1 for OMS Messaging, the Minimum Message Sets, and the lifecycle/status VCRM entries.

Related contract guidance:

- `standards-and-references/oms-standard/docs_official/14_1_OMSC-TMP-003_RevM_ServiceContractTemplate_DandD_v2_5.docx`
- `standards-and-references/oms-standard/docs_official/14_2_OMSC-INS-003_RevM_ServiceContractInstructions_DandD_v2_5.docx`

UCI 2.6 message definitions live inside:

- Archive: `standards-and-references/uci-standard/UCI Release Documentation - UCI Schema/03_UCI-STD-002_Rev6_UCI_Schema_v2_6-CDRL.zip`
- Member: `UCI_MessageDefinitions_v2_6_0.xsd`
- Relevant global elements: `ServiceLifecycleCommand`, `ServiceLifecycleCommandStatus`, `ServiceStatus`, `ServiceStatusDataRequest`, and `ServiceStatusDataRequestStatus`.

Do not equate OWP `INIT` completion with completion of OMS Service initialization. Do not infer lifecycle payloads from prose when the selected UCI schema is available.

## UCI Messages And Interaction Patterns

Normative release artifacts:

- Standard: `standards-and-references/uci-standard/UCI Release Documentation - Standards Documents/02_UCI-STD-001_Rev10_UCI_StandardDocument_v2_6.docx`
- Schema style and design: `standards-and-references/uci-standard/UCI Release Documentation - Standards Documents/04_UCI-SPC-001_Rev6_UCI_SchemaStyleAndDesignSpecification_v2_6.docx`
- Normalized interactions: `standards-and-references/uci-standard/UCI Release Documentation - Standards Documents/06_UCI-SPC-002_Rev2_UCI_NormalizedInterfaceSpecification_v2_6.docx`
- Schema archive: `standards-and-references/uci-standard/UCI Release Documentation - UCI Schema/03_UCI-STD-002_Rev6_UCI_Schema_v2_6-CDRL.zip`

The UCI 2.6 main schema is `UCI_MessageDefinitions_v2_6_0.xsd`, version `002.6.0`, and includes `UCI_SecurityMarkings_v2_6_0.xsd`. Preserve the common `SecurityInformation`, `MessageHeader`, and message-specific `MessageData` structure when designing another encoding.

Use UCI interaction roles rather than assuming HTTP semantics:

- `ServiceStatus`: Status-1 producer/consumer, fire-and-forget.
- `ServiceStatusDataRequest` and `ServiceStatusDataRequestStatus`: DataRequest-2 requester/requestee, correlated by `RequestID`.
- `ServiceLifecycleCommand` and `ServiceLifecycleCommandStatus`: Command-2 commander/commandee, correlated by `CommandID`.

UCI does not prescribe transport, guaranteed delivery, routing, JSON, or Protobuf. A non-XML encoding needs an explicit, testable mapping profile.

## CAL And ASB

Normative sources:

- OMS ASB and CAL architecture: `standards-and-references/oms-standard/docs_official/02_OMSC-STD-001_RevM_OMS_Standard_DandD_v2_5.docx`
- Generic CAL behavior: `standards-and-references/oms-standard/docs_official/04_OMSC-SPC-001_RevL_CAL_Specification_DandD_v2_5.docx`

Searchable CAL copy:

- `standards-and-references/oms-standard/docs_markdown_unofficial/04_OMSC-SPC-001_RevL_CAL_Specification_DandD_v2_5.md`
- Focus on CAL initialization, topics, publish/subscribe behavior, and the ASB connection-state model.

SLEET comparison points:

- In-memory topic/message/group router: `standards-and-references/sleet/sleet/src/router.rs`
- Static service/topic authorization: `standards-and-references/sleet/sleet/src/config.rs`
- SLEET architecture: `standards-and-references/sleet/docs/architecture.md`

SLEET's router is process-local and best-effort. It has no durability, replay, federation, end-to-end acknowledgment, Zenoh adapter, or Protobuf application schema.

## OMS JSON And Validation

Normative source:

- OMS JSON mapping in section 6.1 of `standards-and-references/oms-standard/docs_official/20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.docx`

Implementation references:

- XSD parser: `standards-and-references/sleet/sleet/src/schema/parser.rs`
- Schema model: `standards-and-references/sleet/sleet/src/schema/model.rs`
- JSON validator: `standards-and-references/sleet/sleet/src/validator.rs`
- Facet validation: `standards-and-references/sleet/sleet/src/facets.rs`
- UCI helper generator: `standards-and-references/sleet/scripts/generate_uci_types.py`

SLEET's validator is custom, not a complete general XSD processor. Its behavior is test evidence, not proof that every UCI construct is supported.

## Compliance Evidence

Official OMS artifacts:

- Platform checklist: `standards-and-references/oms-standard/docs_official/16_1_OMSC-CHK-003_RevM_PlatformChecklist_DandD_v2_5.xlsx`
- Platform checklist instructions: `standards-and-references/oms-standard/docs_official/16_2_OMSC-INS-009_RevM_PlatformChecklistInstructions_DandD_v2_5.docx`
- Service checklist: `standards-and-references/oms-standard/docs_official/18_1_OMSC-CHK-005_RevM_ServiceChecklist_DandD_v2_5.xlsx`
- Service checklist instructions: `standards-and-references/oms-standard/docs_official/18_2_OMSC-INS-011_RevM_ServiceChecklistInstructions_DandD_v2_5.docx`
- Platform Description Document template and instructions: `standards-and-references/oms-standard/docs_official/12_1_OMSC-TMP-001_RevM_PDDTemplate_DandD_v2_5.docx` and `standards-and-references/oms-standard/docs_official/12_2_OMSC-INS-001_RevM_PDDInstructions_DandD_v2_5.docx`

SLEET evidence and self-assessment:

- `standards-and-references/sleet/docs/compliance/la-cal-compliance.md`
- `standards-and-references/sleet/docs/compliance/la-cal-harness-assessment.md`
- `standards-and-references/sleet/docs/compliance/oms-platform-checklist.md`

Treat SLEET's compliance documents as claims to verify. Passing an OWP harness does not establish OMS Service or full OMS Platform compliance.
