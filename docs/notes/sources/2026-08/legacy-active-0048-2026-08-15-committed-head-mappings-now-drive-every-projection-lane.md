---
id: legacy-active-0048
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: committed head mappings now drive every projection lane

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1513–1539. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The output IPC already committed `Fit`, `Cover`, or `Exact` independently on
each opaque head, but ordinary native composition still reconstructed every
`HeadRenderTarget` from one session-global `mirror_fit`. Cursor, flat fallback,
bootstrap, mixed, retained, and damage projection used the same hidden global.
An accepted runtime mapping change could therefore update authority state while
the displayed pixels continued using the startup policy.

The duplicate backend policy is removed. Initial desktop configuration is
normalized directly to protocol-neutral `OutputHeadMapping` when the native
owner is constructed; VT and hotplug reconstruction retain that initial policy.
After an output-authority commit, each live head's stored mapping is the single
source consumed by composition, damage, cursor, and fallback projection.
`HeadRenderTarget` projection also carries that head's actual target generation,
scale, refresh, and transform rather than hard-coded generation/transform facts.
Connector-neutral authority snapshots are patched transactionally from the
backend's opaque-head mapping table before publication, so missing coverage
cannot mutate a valid prefix.

Deterministic regressions prove distinct mappings and target generations on two
heads, rejection without partial snapshot mutation, and omission of disabled
heads from the render-target set. The 223-test libdrm feature suite and the
187-test CLI binary suite pass. Remaining architecture work is the native scene
bootstrap, authority-owned native-density variants for server-rendered content,
and signed physical mixed mirror-plus-extended evidence.

<!-- END IMPORTED BODY -->
