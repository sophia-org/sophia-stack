---
id: legacy-active-0150
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-02: CreateWindow is not a configure transition

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4812–4848. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Three freshly built physical runs produced the same blank Firefox owner after
`Open proof dialog`, despite the transient and EWMH role reductions. The GDK
thaw diagnostic landed immediately after the real Engine admission configure.
The classification work was correct but could not repair an earlier lifecycle
violation: Sophia had emitted an unconditional core `ConfigureNotify` from
`CreateWindow`, even when no client selected a lifecycle mask. That false
configure unbalanced GTK's toplevel update bookkeeping, so the later real
configure exposed the thaw underflow and blank frame.

XLibre `dix/window.c` emits only parent-selected `CreateNotify` during
creation, returns silently from a no-op configure, and delivers map events
through structure/substructure selection. Its realized/viewable split also
keeps a mapped descendant off-screen below an unrealized ancestor. yserver's
independent Rust tables encode the same protocol states as `Unmapped`,
`Unviewable`, and `Viewable`, promote mapped descendants when an ancestor
becomes viewable, and gate visibility/exposure on that final state.

Sophia now follows those boundaries. Deferred policy admission is a separate
flag rather than a fake X map state. Ancestor map, unmap, and reparent changes
propagate viewability through mapped descendants. Create, map, configure,
visibility, and exposure events are filtered by each client's masks and
structure events are also delivered to parent substructure selectors. A
managed configure denial remains an explicitly synthetic response, while an
unchanged client-controlled configure stays silent. The Firefox-shaped
regression requires an early-mapped render child to remain Unviewable until
top-level admission, then observes ordered configure/map, subtree visibility,
and subtree exposure. A separate two-client wire regression locks down parent
and owner delivery and makes any queued create-time configure fail the next
geometry assertion.

The physical gate now rejects GDK thaw warnings, popup-era layout timeout or
WM restart, and a popup layout that precedes exact matching visual retirement.
The offline X-authority and fail-closed verifier suites pass. A fresh physical
workflow remains the acceptance boundary.

<!-- END IMPORTED BODY -->
