---
id: legacy-milestone-0004
date: 2026-08-11
recorded_date: 2026-08-11
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-08-11 Milestone 12 Frozen Classical-WM Compatibility Baseline

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 75–128.

<!-- BEGIN IMPORTED BODY -->

- [x] Closed the intended xmonad desktop configuration: the packaged 0.18
  series builds without mutable `~/.config/xmonad` state; `ThreeColMid`, `Tall`,
  `Mirror Tall`, `Full`, and `Spiral` hold exact configured multi-surface
  geometry; `Tabbed`, title/class manage hooks, and xmonad-owned decorations
  stay excluded from the blind-WM contract; the configured xmonad and xmobar
  executables are immutable release inputs with recorded source revision, build
  configuration, and binary digest; and the retained personal profile's safe
  core became opaque Sophia actions behind an IR_Black-derived one-pixel Engine
  frame with xmobar static, redacted, and title-free.
- [x] Completed TrueColor semantics: the advertised 24-bit XRGB and 32-bit ARGB
  contract is internally exact, `AllocColor` converts RGB16 through the
  advertised masks, `QueryColors` recovers channel intensities instead of
  collapsing to white, `AllocNamedColor` uses a bounded deterministic table that
  returns the correct X error for unknown names, and the conventional
  1/4/8/15/16 auxiliary pixmap formats are advertised and proven separately from
  the TrueColor visuals in both byte orders. Visual IDs, colormap IDs, channel
  masks, and X color names stayed inside X Authority.
- [x] Rebuilt and re-proved the candidate across two-phase admission recovery
  and complete position-and-size geometry delivery, then passed each focused
  installed gate on immutable artifacts: xterm attempt `0003` on `7e18ea3a`,
  automatic Firefox run `0002` and the xmobar work-area gate on `4c312142`,
  physical TrueColor attempt `0003` and emergency-recovery attempt `0004` on
  `883666a2` with `reverified=0`, and ten-cycle lifecycle runs `0044` through
  `0053` on `883666a2` with two-output readiness between 288 and 324 ms.
- [x] Audited workspace and admission recovery with the commit-pinned Specula
  tool, retaining project-sized formal models and deterministic regressions
  rather than a runtime or build dependency. Preliminary soak attempt `0054`
  remains a failed immutable artifact; installed run `0055` on `a2fdf4f6` then
  held clean admission, workspace projection, two independently advancing
  animated surfaces, and zero layout timeout, resize abort, hidden-surface
  command, or WM restart.
- [x] Recorded the fixed-TrueColor row in the X11 compatibility matrix once both
  the wire regressions and the visible physical proof passed.

The installed-candidate lineage is retained failure evidence in order:
`1a7d67c3` (admission recovery), `7bd3e7db` (move feedback), `a50dfb67`
(committed-layout reseed), `53a21365` (false core `GetImage` ceiling),
`fb1c3804` (unbounded ephemeral Firefox profiles), `ce494942` and run `0042`
(resource lifecycle cleanup), `7a6be56c` (renderer-worker settlement during VT
handoff), and `4c312142` (closed that gap). Any reproduction of the historical
xmonad promotion or soak artifacts must use one of these exact immutable builds
or a verified source-identical successor.

This milestone froze xmonad as a compatibility baseline and regression corpus,
not as Sophia's promotion vehicle. Its profile-specific work stayed behind the
generic compatibility boundary and never made xmonad concepts part of Engine or
the universal WM API. The remaining practical short gate and long xmonad soaks
moved to the Classical X11 WM Compatibility section of `todo.md`; elapsed wall
time is not a promotion criterion. Hagia owns the active product path.

---

<!-- END IMPORTED BODY -->
