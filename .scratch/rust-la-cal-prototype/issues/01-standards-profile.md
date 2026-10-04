# Choose the OMS/UCI profile with a UCI 2.6 preference

Parent: ../map.md
Labels: wayfinder:grilling
Type: grilling
Status: resolved

## Question

Which exact OMS, LA-CAL/OWP, and UCI releases and artifacts form the prototype's supported profile, given the user's preference for UCI 2.6 where possible and the available OMS 2.5 documents?

Identify the relevant compatibility evidence and gaps; decide how any exceptions are recorded and which conformance claims the resulting profile supports. Do not silently substitute UCI 2.5, or claim a mixed profile is an official release combination. The resolution must give later tickets an authoritative schema baseline and precise source/version references.

## Comments

- 2026-10-02: After reviewing the official release coupling and SLEET's pinned schema, the user confirmed: “alright then, well use 2.5 if oms enforces that and sleet uses it as well”. This explicitly supersedes the initial UCI 2.6 preference.

## Answer

### Selected profile

Use the release-aligned **OMS 2.5 / UCI 002.5.0** baseline:

- OMS D&D release **002.5**: OMS Standard **OMSC-STD-001 Rev M**, generic CAL **OMSC-SPC-001 Rev L**, and LA-CAL **OMSC-SPC-013 Rev B**, inventoried in **OMSC-VDD-001 Rev M**, Table 2.1-1, dated 22 January 2026.
- **OWP 1.0**, specified in LA-CAL Rev B §5.1.1.1, using WebSocket subprotocol `owp` (Table 5.1-1).
- **OMS JSON**, as defined by LA-CAL Rev B §6.1, validated against the selected UCI schema (§6.1.5). No separate JSON release number is assigned here.
- UCI release **002.5**: UCI Standard **OAC-STD-001 Rev I**, schema **OAC-STD-002 Rev E**, Schema Style and Design **OAC-SPC-001 Rev E**, and Normalized Interface Specification **OAC-SPC-002 Rev A**. The main XSD version is **002.5.0**; use that exact value for OWP schema negotiation and schema-version configuration.

### Authority and source provenance

OMS official sources are under `standards-and-references/oms-standard/docs_official/`:

- `01_OMSC-VDD-001_RevM_VersionDescriptionDocument_DandD_v2_5.docx`: release inventory, Table 2.1-1.
- `02_OMSC-STD-001_RevM_OMS_Standard_DandD_v2_5.docx`: §6.3.1 identifies the compatible UCI schema by reference to Table 5.1-1, which lists the UCI 2.5 artifacts. Service lifecycle behavior is defined in §6.1.1 and Table 6.1-1.
- `04_OMSC-SPC-001_RevL_CAL_Specification_DandD_v2_5.docx`: generic CAL baseline.
- `20_OMSC-SPC-013_RevB_LanguageAgnostic_CAL_Specification_DandD_v2_5.docx`: §4.1 delegates UCI schema-version selection to the OMS Standard; §5.1.1.1.3.1 and Table 5.1-4 define separate protocol/schema negotiation; §6.1 defines OMS JSON.

The UCI submodule's current checkout contains 2.6. The authoritative **2.5** artifacts are available in its local Git history, at commit **`73a286fc4b881d9f328dbd64148525606a92c33a`** in `standards-and-references/uci-standard` (upstream `https://gitlab.com/open-arsenal/uci/standard.git`). Paths below refer to that commit, not the current working tree:

- `01_OAC-VDD-001_RevJ_UCI_VersionDescriptionDocument_v2_5.docx`: cover and Table 2.0-1 establish release 002.5 and the 22 January 2026 artifact inventory.
- `02_OAC-STD-001_RevI_UCI_StandardDocument_v2_5.docx`.
- `03_OAC-STD-002_RevE_UCI_Schema_v2_5/UCI_MessageDefinitions_v2_5_0.xsd`.
- `03_OAC-STD-002_RevE_UCI_Schema_v2_5/UCI_SecurityMarkings_v2_5_0.xsd`.
- `03_OAC-STD-002_RevE_UCI_Schema_v2_5/UCI_Versioning_v2_5_0.xsd`.
- `04_OAC-SPC-001_RevE_UCI_SchemaStyleAndDesignSpecification_v2_5.docx`.
- `06_OAC-SPC-002_RevA_UCI_NormalizedInterfaceSpecification_v2_5.docx`.

Use the main XSD and its included SecurityMarkings XSD as the authoritative input for message mapping and generation. Preserve the pinned source revision; do not substitute the current 2.6 checkout. The later tooling decision must specify reproducible extraction and generation. No submodule checkout was changed to resolve this decision.

### Rationale and compatibility boundary

The selected pairing follows the OMS 2.5 Standard's explicit schema reference and matches SLEET's default runtime schema and generated UCI helpers. SLEET's `docs/schema-sourcing.md` pins the same UCI commit and `002.5.0` schema, corroborating implementation alignment without acting as normative authority.

OWP's separate schema negotiation makes another schema technically negotiable, but does not establish an approved OMS/UCI release pairing. A focused comparison found unchanged lifecycle payload structures in UCI 2.6 but inherited security-envelope changes, so whole-message compatibility cannot be assumed. Selecting 2.5 avoids that release-substitution exception; 2.6 support is not part of this prototype's supported profile.

### Claims and downstream consequences

This selection supplies a release-aligned basis for requirement-linked OWP client and LA-CAL server conformance evidence and a precisely scoped OMS 2.5 lifecycle integration demonstration. Profile selection itself proves no implementation conformance, and the demonstration does not establish full OMS Service or Platform tier compliance.

The Zenoh/Protobuf binding remains project-defined. Its schema mapping, validation and routing require explicit decisions and evidence; selecting the official UCI schema does not make that binding an official UCI encoding. Any later requirement deviations must be recorded explicitly in the relevant decision and acceptance evidence, not hidden in a release label.

The generator discovery report investigated UCI 2.6 artifacts. Preserve it as historical evidence; generator selection must evaluate the chosen **UCI 2.5** input and account for this search-coverage difference.
