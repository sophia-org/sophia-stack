---
id: legacy-milestone-0007
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-08-03 Milestone 10 Physical Firefox Workload

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 175–195.

<!-- BEGIN IMPORTED BODY -->

- [x] Reduced attached-dialog and floating-window behavior into generic opaque
  policy facts, Engine-owned placement validation, and transactional
  move/resize gestures without application heuristics.
- [x] Closed focused Firefox rendering, dialog, selection, lifecycle, resize,
  focus, input, and cleanup gates before the integrated run.
- [x] Passed one integrated physical workflow with all six Firefox stages and
  six Kitty retention checkpoints around normal and WM-forced status-zero
  Firefox exits.
- [x] Retained synchronous startup presentation on both outputs, asynchronous
  retirement on the active output, real DOM navigation and wheel progress,
  exact resized pixels, and clean session, layout, application, frontend,
  namespace, and Xauthority teardown.

Repetition moved to the installed unattended and interactive soak gates. The
completed Firefox milestone does not claim Chromium compatibility or general
X11 conformance.

---

<!-- END IMPORTED BODY -->
