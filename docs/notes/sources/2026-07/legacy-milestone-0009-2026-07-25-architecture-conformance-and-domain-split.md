---
id: legacy-milestone-0009
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-07-25 Architecture Conformance And Domain Split

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 214–242.

<!-- BEGIN IMPORTED BODY -->

- [x] Replaced application-specific session launch variants with opaque
  configuration-owned application IDs and kept Engine policy protocol-neutral.
- [x] Removed per-event input projection allocation and centralized
  authority-layer projection under Engine.
- [x] Split X authority state, service, routing, input, clipboard, wire,
  dispatch, and client output into bounded domain owners behind stable facades.
- [x] Split live-session policy, admission, WM/layout, input, presentation,
  process supervision, owner-loop state, and tick phases by ownership.
- [x] Split native scanout resource lifetime from composition execution and
  legacy-WM framing from runtime supervision.
- [x] Extracted visual diagnostics and asynchronous output servicing from the
  production runtime facade.
- [x] Moved production inline tests into integration tests and shared fixtures.
- [x] Replaced direct library printing and free-form traces with structured,
  bounded, redacted observations.
- [x] Replaced callback-owned mutation and Present delivery with explicit
  Engine projection and owner-drained queues.
- [x] Removed synthetic committed-surface seeding and client-local X
  identifiers from Engine/session routing.

The source-layout audit now prevents new unreviewed large modules, inline
production tests, and direct library printing. Reviewed exceptions remain in
`docs/source-layout-exceptions.txt`. Future splits follow domain ownership and
data flow rather than a strict line-count target.

---

<!-- END IMPORTED BODY -->
