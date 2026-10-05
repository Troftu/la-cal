---
name: oms-la-cal
description: Consult the project-local OMS, UCI, LA-CAL, OWP, SLEET, and LA-CAL harness sources when designing, implementing, debugging, reviewing, or documenting service-lifecycle messaging, WebSocket protocol behavior, OMS JSON, ASB integration, Zenoh, or Protobuf translation.
---

# OMS LA-CAL

Ground work in the standards and reference code under `standards-and-references/`. Treat those Git submodules as read-only. If their directories are empty, initialize them with `git submodule update --init --recursive`.

## Authority

Use this order when sources disagree:

1. The official OMS documents under `standards-and-references/oms-standard/docs_official/`.
2. The released UCI documents and schema package under `standards-and-references/uci-standard/`.
3. The unofficial OMS Markdown conversions for search and navigation only; verify normative wording, tables, and diagrams in the matching official document.
4. SLEET and the external LA-CAL harness as implementation/test evidence, never as the authority for a compliance claim.

Keep versions explicit in every design or compliance claim. The checked-in OMS release and SLEET implementation target OMS/UCI 2.5, while the checked-in UCI standards submodule contains the UCI 2.6 schema (`002.6.0`). Never combine their message definitions silently. First choose and record the project's supported OMS/UCI profile.

## Boundaries

Keep these separate:

- The OWP connection state machine establishes a CAL connection through WebSocket `INIT`/`INFO`.
- The OMS Service lifecycle governs `INITIALIZING`, `NORMAL`, `DEGRADED`, `PAUSED`, and `INOPERABLE` behavior.
- The CAL-to-ASB adapter maps OWP topics and OMS JSON to the platform's internal messaging technology.
- UCI defines message structures and interaction patterns, not the transport, topic names, delivery guarantees, or a Protobuf encoding.

The supplied references do not define a Zenoh/Protobuf ASB binding. Treat OMS JSON to Protobuf conversion, Zenoh key expressions, correlation, delivery semantics, and acknowledgment timing as project decisions behind an adapter seam.

SLEET does not provide that adapter: it uses an in-process, best-effort router and has no Zenoh or application-level Protobuf integration. It also routes lifecycle messages without implementing the OMS Service lifecycle.

## Working Method

1. Open only the relevant branch of [`REFERENCES.md`](REFERENCES.md).
2. Read the normative source before deriving behavior from SLEET or harness assertions.
3. Record the exact standard version, document, section, and requirement or certification identifier behind compliance-sensitive behavior.
4. Use SLEET and the harness to find edge cases and executable examples, then verify them against the selected standard profile. For harness reuse or report interpretation, follow the external-harness branch of the source map.
5. Surface gaps as decisions. Do not fill unspecified transport, security, reliability, lifecycle, or encoding behavior by assumption.

When claiming compliance, distinguish OWP client conformance, LA-CAL server conformance, OMS Service tier compliance, and OMS Platform tier compliance. None implies the others.
