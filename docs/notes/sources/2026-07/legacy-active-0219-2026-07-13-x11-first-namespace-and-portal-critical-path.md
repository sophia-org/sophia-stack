---
id: legacy-active-0219
date: 2026-07-13
recorded_date: 2026-07-13
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "security"]
---
# 2026-07-13: X11-First Namespace And Portal Critical Path

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7343–7363. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sophia's next architecture work is the native X Server Frontend, not broader
Wayland protocol or DMA-BUF coverage. The two-xterm frontend already proves
bounded concurrent workers, client-attributed transactions, targeted input,
Engine composition, and KMS presentation. Its next risk is no longer basic
visibility; it is admitting clients into the correct trust domain before more
X11 semantics depend on a hardcoded listener namespace.

The chosen dependency order is session-owned namespace admission, then a portal
broker with X11 `CLIPBOARD`/`PRIMARY` as its first complete adapter, then XKB,
grabs, Engine-derived output/resize, and standard presentation semantics.
Classic shared-X intentionally retains same-namespace resource visibility.
Confined sessions use distinct namespaces and explicit capabilities; XID ranges
remain creation/cleanup ledgers rather than access-control lists.

At this stage Wayland/Smithay stayed supported under maintenance gates. The
2026-07-19 retirement decision above supersedes that status. XLibre remained
frozen historical evidence and a possible future provider only if measured
native-X gaps later justified its authority and maintenance cost.

<!-- END IMPORTED BODY -->
