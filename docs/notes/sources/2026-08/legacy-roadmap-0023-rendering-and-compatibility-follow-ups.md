---
id: legacy-roadmap-0023
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Rendering And Compatibility Follow-Ups

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 2602–2642.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0017-post-promotion-capability-roadmap.md).

<!-- BEGIN IMPORTED BODY -->

- [ ] Fix stable-relayout silence in the xmonad compatibility bridge. Signed
  source `c681f762` reached one through four retained synthetic windows, but
  unchanged `SceneChanged` cycles waited for ConfigureRequest replies that real
  xmonad did not emit and exhausted the outer supervisor. Add a quiet-boundary
  regression for stable retained geometry while preserving strict manage and
  resize response fences. This is retained compatibility work, not a native
  Sophia WM protocol, shell protocol, or Milestone 14 promotion blocker.
- [ ] Retain the bounded physical `glxgears` proof with visible animation,
  advancing Present/KMS cadence, matching reference provider, clean retirement,
  and zero protocol or renderer debt.
- [ ] Obtain an unredirected Xorg/XLibre `Flip` reference only if end-to-end
  presentation-latency parity is needed. Keep composited `Copy` results labeled
  as client-cadence evidence.
- [ ] Complete the two-output concurrent-producer workload after the shared
  renderer-worker prerequisite in Milestone 13. Require bounded inter-output
  service skew and no producer starvation.
- [ ] Replace per-frame CPU GBM allocation with an output-scoped,
  retirement-fed three-slot pool only if measured software fallback remains
  outside its parity gate.
- [ ] Run the deterministic Firefox pointer/keyboard/wheel fixture in Chromium
  as an independent native-X consumer after Chromium is installed.
- [ ] Add client-selected classic X11 cursor images or further toolkit,
  extension, font, color, and WM behavior only when a retained workflow exposes
  the missing protocol fact.
- [ ] Add opportunistic scanout cloning for equal-mode mirror heads after the
  per-head composition path is promotion-proven. Eligibility is plan-record
  equivalence (geometry, mapping, density, generations; never the content
  checksum) per the normative design in
  [Multi-Monitor Per-Head Composition](../../../multi-monitor-composition.md#target);
  the decision stays backend-private and switches only through topology
  transactions. Exit gate: an equal-mode physical mirror run whose evidence
  shows one composition per frame and one framebuffer identity behind both
  opaque heads' plane assignments with joined retirement; a dual-render audit
  phase proving the cloned and per-head compositions byte-identical for one
  committed scene; a forced mid-run divergence through `sophia_output_v1` that
  demotes to per-head with no visual discontinuity or leaked framebuffer; and
  re-promotion only after a passing atomic `TEST_ONLY` probe. Cross-card
  mirrors and unequal modes remain render copy permanently.

<!-- END IMPORTED BODY -->
