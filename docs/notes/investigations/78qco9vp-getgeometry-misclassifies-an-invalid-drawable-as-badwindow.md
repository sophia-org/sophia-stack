---
id: 78qco9vp
date: 2026-09-12
kind: investigation
status: closed
tags: [investigation, x11, protocol, conformance]
---
# GetGeometry misclassifies an invalid drawable as BadWindow

## Independent evidence

The mixed-owner descendant test on runtime 385282b4 now observes a genuinely
destroyed child, but GetGeometry returns error 3 (BadWindow), not error 9
(BadDrawable). Both byte orders fail the strict error assertion in
`.artifacts/x11-xfixes-accepted/report.json`; the other 96 executions pass.
This is a different result from the successful geometry reply before the
lifecycle repair. The old artifact directory name is not an acceptance claim:
the report says FAIL.

The [X11 GetGeometry contract](https://xorg.freedesktop.org/archive/X11R7.7/doc/xproto/x11protocol.html#GetGeometry)
accepts a drawable and names Drawable as its resource error. Its invalid-ID
completion must preserve the offending drawable, request sequence and opcode.

## Source and acceptance

At 385282b4, `dispatch/core/windows.rs` passes a failed `drawable_facts` lookup
through the generic runtime-error mapper, preserving its window classification.
The nearby comment explicitly describes that inherited behavior. Task t092
requires request-specific drawable classification without changing successful
geometry replies or broadly relabeling unrelated request errors.

The independent `get_geometry_errors` case requires BadDrawable for both an
unallocated ID and a destroyed window, then reads an existing window's geometry
to prove continuation. The mixed-owner selection case keeps its strict
BadDrawable assertion. Run both byte orders on a private frontend; no installed
session or display is needed.

## Resolution

Repaired in 9be53aff by changing the missing-window classification to BadDrawable
specifically in GetGeometry. Other runtime error classes are preserved. Both
standalone invalid-drawable and mixed-owner cases now pass in both byte orders;
the final gate reports 100/100 in
`.artifacts/x11-conformance/final-100/report.json`. The expanded before-run in
`.artifacts/x11-geometry-before/report.json` records 96 PASS and four failures.
Task t092 is closed for this completion error.

## Connections

- [todo.md](../../../todo.md): t092 joins the highest-priority protocol tranche.
- [Mixed-owner lifecycle](g8c2ey1f-peer-owned-child-selections-outlive-a-disconnected-parent.md):
  that repair exposed this completion error after actual destruction began.
- [Conformance evidence](wzxlxbok-independent-x11-socket-conformance-exposes-missing-client-completions.md):
  preserve both the before and repaired error classifications.
