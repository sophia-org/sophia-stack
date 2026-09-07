---
id: legacy-active-0034
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering"]
---
# 2026-08-27: a staged launch can precede its first safe pixels

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1117–1150. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The schema-5 packaged-default entry finally launched the intended installed
  release `0.1.0-50c7cb2d2d54`. Promotion attempt `0001` records the packaged
  Hagia profile at its manifest hash, one physical action, the normal Logout
  session action, zero protocol errors, clean health, and clean teardown. That
  lifecycle result is valid, but it does not override the operator's real-client
  observation: `glxgears` opened and never became visible.
- The retained stream makes the missing transition exact. GLX surface `4194306`
  produced Present transaction `589`, DMA-BUF `5`, and a selected safe 300-by-300
  visual candidate. Its public Manage response had already staged a 1278-by-1424
  placement in layout transaction `6` while the launch was pixel-silent, so the
  resize gate correctly deferred that surface. Candidate selection only queued
  recovery when no layout was pending and only primed the safe admission extent
  while reducing the earlier Manage response. The pending epoch therefore
  committed its one matching sibling, and nothing ever staged, armed, or
  presented the GLX candidate.
- Candidate selection now primes the measured admission extent at the point the
  pixels become authoritative. It queues a successor relayout whenever the live
  epoch does not own that surface at the exact measured extent; a pending epoch
  with the launch removed from its requested-size gate is no longer mistaken for
  an admission path. The existing candidate-before-layout regression now pins
  immediate extent priming, and a second regression reproduces the physical
  ordering with a 300-by-300 candidate, a deferred 1278-by-1424 standing target,
  and an unrelated in-flight settlement. A freshly packaged installed successor
  must still prove the visual admission and ordinary logout before promotion.
- The full configuration suite also exposed five old tests that implicitly
  loaded whatever desktop profile was current. The generic compiled profile now
  enables the shell, so their intentionally partial normal-session arguments
  were no longer complete configurations. Those tests now name the existing
  capability-free profile fixture explicitly. This isolates the CLI concerns
  they assert without copying personal Hagia policy into product source or
  weakening the compiled product default.

<!-- END IMPORTED BODY -->
