---
id: t022
date: 2026-09-12
kind: investigation
tags: [protocol, ipc, conformance]
---
# Native protocol-family lifecycle audit

This audit compares `sophia_wm_v1`, `sophia_shell_v1` and
`sophia_output_v1` with the common contract in
[sophia-policy-ipc.md](../../sophia-policy-ipc.md). It records intentional
role differences rather than forcing unlike authorities into one wire shape.

| Concern | WM v1 | Shell v1 | Output v1 |
| --- | --- | --- | --- |
| Endpoint | `SOPHIA_WM_SOCKET` | `SOPHIA_SHELL_SOCKET` | `SOPHIA_OUTPUT_SOCKET` |
| Current revision | stable major 1 revision 3 | experimental major 1 revision 6 | experimental major 1 revision 1 |
| Hello capability field | requested `capabilities` | required `required_capabilities` | requested `capabilities` |
| Welcome bounds | snapshot/projection ceilings | descriptor ceilings; gated workflows publish their own bounds | head/group/mode ceilings |
| Complete facts | snapshot begin/chunk/end | workflow-specific complete snapshots; content uses facts and begin/chunk/end records | one bounded snapshot frame |
| Candidate | projection begin/chunk/end | descriptor candidate or content begin/chunk/end | one bounded proposal frame |
| Settlement | prepared/committed/rejected family outcomes | prepared/presented/rejected/superseded per workflow | validated then committed/stale/rejected/rolled-back/failed |
| Replacement | fresh connection epoch, last committed layout retained | fresh connection/content epochs, last coherent presentation retained | fresh connection epoch, last published topology retained until replacement commits |
| Declarative schema | checked-in and frozen | checked-in and additive | absent; this blocks stability |
| Independent lifecycle client | Hagia plus retained C99 client | Narthex plus independent C clients for bounded workflows | absent; required before stability |

The different outcome names are intentional authority semantics: WM commits a
layout, shell presents content, and output authority performs a physical apply
with rollback. Output's single-frame snapshot/proposal is also intentional while
its declared limits keep each legal frame below the family ceiling. It must gain
a schema, valid and malformed corpus, and independent lifecycle client before
stability; handwritten Rust codecs and socket tests are implementation evidence.

The audit corrected stale common documentation which described shell revision 3
as current and revision-5 content as unassigned. The checked shell schema is the
wire authority: revision 6, content kinds 160–180 behind bits 7–8, and indicator
kinds 181–186 behind bits 9–10. Production capability negotiation still refuses
content, so this correction grants no new authority.

`tools/check_native_protocol_family.sh` is the one contributor entry point. It
invokes each existing role gate and the output role's current codec, transport,
service and owning-session lifecycle tests. It fails rather than skipping a
missing independent Hagia or Narthex checkout. Output remains explicitly
experimental, so the family entry point does not mislabel its Rust-only evidence
as a stability proof.
