---
id: legacy-active-0402
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-08-08: Practical xmonad policy remains opaque and Engine-themed

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12228–12261. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The retained personal xmonad configuration supplied the practical policy
  vocabulary and IR_Black colors, not an authority model. Sophia now registers
  distinct opaque actions for focus/swap master, swap up/down, shrink/expand,
  master-count, layout reset, floating toggle, and sink. The public WM frame
  did not change; Super chords terminate in Engine and translate to private
  Mod1 chords only inside the compatibility bridge.
- Xmonad keeps a zero-pixel border. A packaged core KDL file makes Engine the
  sole owner of the one-pixel `#ffb6b0`/`#7c7c7c` frame, while xmobar retains
  static IR_Black-derived system counters with no title, class, XID, PID, or
  namespace feed. Release manifest schema 3 binds that core file by SHA-256;
  the verifier continues to read historical schema-2 packages.
- A registered xmonad grab may legitimately leave geometry unchanged. The
  bridge accepts that quiet no-op after a dedicated 250-millisecond settling
  interval, restarts the interval after any response, still requires pointer
  gestures to produce activity, and poisons the private process after deadline
  or disconnect. Process regressions cover every private practical chord and a
  delayed layout response that must not cross into its successor. The revised
  `LegacyWmResponseBoundary` model checks the registered-grab prerequisite
  across 9,489 distinct states to depth 42.
- The protocol-neutral workspace reducer now rejects configure or render
  commands for a surface hidden from every candidate output. Clean completion
  publishes only the redacted zero invariant; it does not retain the rejected
  surface identity.
- One shared shell reducer owns long-soak counts. The installed progress view,
  normal-run recorder, raw session verifier, and archive verifier use it to
  require every practical action once, any physical workspace view and move,
  both pointer gesture modes, workload thresholds, and zero layout timeout,
  resize abort, hidden command, stale response, rejection, or pending work.
  The final redacted summary is checksummed and independently recomputed from
  the raw archive, so progress reporting cannot become a second evidence
  interpretation.

<!-- END IMPORTED BODY -->
