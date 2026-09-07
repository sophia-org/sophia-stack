---
id: legacy-active-0634
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell", "architecture"]
---
# 2026-09-06 — Descriptor and content shells have distinct trust contracts

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20411–20442. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The native launcher discussion exposed a gap between the replaceable-shell
promise and today's bounded descriptor appearance. The chosen direction retains
both models: Narthex remains the maintained descriptor reference, while a future
content capability lets a shell rasterize its own interface. One admitted native
shell may combine those capabilities. The session's operator policy must grant
content explicitly at startup; choosing an executable does not grant it.

The [content-shell proposal](../../../content-shell.md) collects the behavioral contract
without allocating a revision, messages, configuration keys, or transport. Its
first workflow is the retained panel, anchored popout, and local state-changing
control. Discrete target actions suffice for that workflow. General keyboard
input and region-local pointer coordinates need separate extensions. Engine
retains authoritative placement, presented input, resource retirement, coherent
reservation/work-area/WM changes, compositor effects, and presentation timing.
Custom widgets remain shell content rather than new Engine primitives.

The trust distinction is explicit. Content permission grants no foreign pixels,
application execution, or WM authority, but arbitrary artwork can misrepresent
what a button does. Presented activation does not prove an honest label. A later
custom launcher must specify identity and activation semantics rather than
inherit the descriptor launcher's immutable-label assurance. The guides also
correct the overly broad claims that a shell reads no pixels and that descriptor
confinement guarantees phishing prevention.

This is documentation preparation. Transport feasibility, pixel semantics,
numeric limits, wire design, modeling, independent Quickshell/C conformance,
and physical acceptance remain later gates. X11 development-session readiness
and the CP-15 coherence prerequisites keep their existing order. The current
shell, application policy, and live session are unchanged.

<!-- END IMPORTED BODY -->
