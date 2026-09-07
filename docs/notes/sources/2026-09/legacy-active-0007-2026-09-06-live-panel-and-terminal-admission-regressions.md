---
id: legacy-active-0007
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell"]
---
# 2026-09-06: live panel and terminal admission regressions

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 221–264. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed Sophia `49e29e80` with Hagia `875c8c2` exposed three failures during
ordinary use. The current session log is
`~/.local/state/sophia/hagia-session/session.log`; no comparison campaign is
needed to diagnose them.

- Quickshell created hardware OpenGL contexts on the Radeon RX 7900 GRE and
  reserved 32 pixels on each output, but neither dock surface retired. The GPU
  runtime required a WM output assignment for every surface, although
  frontend-positioned docks deliberately bypass WM placement. The session now
  supplies an explicit visible frontend-positioned set. Backend composition
  and Present retirement use that set; ordinary surfaces still require a policy
  owner, and scrolling columns cannot bleed onto a neighboring output. This
  changes no public wire protocol and adds no toolkit-specific authority.
- The first Kitty was correctly resized from 1258x1408 to 1258x1390 after the
  panel reservation changed. An aborted launch's standing target then restored
  height 1408 at y=41, extending below the 1440-pixel output. Fresh explicit
  policy sizes now supersede that obligation unless they merely echo a
  temporary recovery constraint. First-frame recovery remains transactional.
- Super+Return successfully launched Kitty. Surface 8388622, for example,
  produced candidate 23649 and committed layout 21, but its exact Present was
  superseded by retained repaints and launch completion timed out. Compositor
  projection changes now coalesce until the owning Present retires. Tab bars,
  overlays, outlines and layout repaints share the guard; translation repaints
  also respect bound software Presents. A managed window overlapping the
  neighboring monitor no longer creates a retirement obligation on that head.

Focused regression coverage exercises routing withdrawal and unknown-surface
exclusion, policy ownership precedence, display-list inclusion, first-Present
render/submission/retirement fencing, and work-area target replacement across
recovery. Hardware contexts and offline regressions do not prove the repaired
pixels reached glass. Physical acceptance remains open: see both panels, open
three Kitty windows with Super+Return, check bounds and navigation, then resume
ordinary usage. The separately recorded forced-deadline control drain failure
remains open.

Validation: `cargo xtask check` passes with a fresh private `TMPDIR` and an
empty `XDG_CONFIG_HOME` (2,434 Rust test executions, Clippy, unchanged layout
debt, reader and archive checks). Without isolation, old PID-named temporary
directories collide and session parser tests read the operator's personal
startup registry. Those test-environment dependencies remain tooling debt;
no live configuration was removed or changed to make the gate pass.

<!-- END IMPORTED BODY -->
