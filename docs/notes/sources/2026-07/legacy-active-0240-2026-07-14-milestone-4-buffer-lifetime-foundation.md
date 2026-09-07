---
id: legacy-active-0240
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "tooling"]
---
# 2026-07-14: Milestone 4 Buffer Lifetime Foundation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8048–8072. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Milestone 4 now has an explicit reduced buffer boundary. Protocol-visible
DMA-BUF descriptors use opaque buffer and fence identities, admit at most four
planes, accept only bounded XRGB8888/ARGB8888 dimensions and byte ranges, and
contain no native renderer objects or file descriptors. Native plane and acquire
fence FDs enter a renderer-private registry. That registry refuses duplicate or
malformed registrations, blocks submission behind an unsignaled acquire fence,
and releases ownership only after page-flip retirement, rejection, or disconnect.

X software drawing now publishes a new immutable handle for every accepted
generation rather than patching a buffer already visible to Engine. External
tests retain the earlier generation and prove its bytes do not change. A matching
CPU lifetime reducer keeps the last committed handle through stale retirement and
rejection, releases the previous handle only after the replacement page flip,
and drains disconnect ownership exactly once.

MIT-SHM now advertises a real extension event base and encodes the standard
Completion event layout when PutImage requested notification and the request was
accepted. The completion is not emitted for rejected window updates. The
workspace all-feature test gate passes offline on X13. Standard DRI3/Present
SCM_RIGHTS transport, client-visible Present feedback, mixed GPU/CPU composition,
and retained Vulkan hardware evidence remain open; the private SOPHIA-PRESENT
prototype is not promoted by this checkpoint.

<!-- END IMPORTED BODY -->
