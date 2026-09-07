---
id: legacy-active-0567
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-30: the direct-scanout archives, recorded after the fact

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17674–17714. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The log stopped at "asking the driver before changing the screen" while three
direct-scanout archives and six cursor archives were promoted. That breaks the
rule in `docs/development-tooling.md` that architecture, the roadmap, and this
log agree, so the tranche is recorded here retrospectively. The narrative form
lives in `docs/roadmap-archive-2026-08-30.md`; what follows is what each archive
proved and what getting there found.

Archive `0001` promoted eligibility: thirty-eight client buffers reached the
plane from one validating commit, with no test rejections, no proof
disagreements, no unsupported formats, and no fallbacks.

Archive `0002` promoted the return to composition. An overlay opened through
the same `set_descriptor_overlay` entry the shell uses, a composed successor was
built from the client's still-held planes and retired inside the window with the
displaced buffer's snapshot promoted before its release, the overlay withdrew, a
second validating commit passed, and flips resumed. Three boundary defects
surfaced that no offline suite could reach: the retained requeue sourced a
renderer image that had never been imported, the lowering carried an eligibility
verdict that stopped being true the moment it substituted the snapshot, and the
conformance readers anchored records to the line start, so the episode-order
rules had never actually run against decorated hardware evidence.

Archive `0003` answered what a direct frame costs. On one head in one session,
the client's buffer was offered to the plane in 17 microseconds at the median
against 35 for a composed frame, and at p99 in 22 microseconds against 12,883.
The tail is the finding: a direct frame's cost is nearly constant because
nothing is drawn, while a composed frame waits on a renderer that occasionally
takes twelve milliseconds. Submit-to-flip was measured beside it only to check
that the display engine does not care how a buffer arrived -- 7,972 against
15,099 microseconds at the median, both dominated by this host's chronic DCN32
stalls rather than by anything Sophia does.

One instrumentation defect surfaced only under real evidence. The classifier
asked the flip counter whether the export it had just timed was direct, but a
flip happens later, at submit, so every export answered no and the direct
population was left with no offer samples at all. The emitter then dropped the
half-measured population rather than reporting it, so the only symptom was a
comparison claiming no direct frames existed.

<!-- END IMPORTED BODY -->
