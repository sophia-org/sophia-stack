---
id: legacy-active-0202
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "rendering", "tooling"]
---
# 2026-07-25: Present Configure And Pixel Size Define Resize Readiness

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6923–6946. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical three- and four-Kitty trace disproved the earlier placement-only
resize fix. Xmonad requested 1280-by-720 and 1280-by-480 tiles, but every
retired Kitty source remained 1280-by-1440. The frontend emitted core
`ConfigureNotify` only, although Mesa's DRI3 loader selects Present
`ConfigureNotify` and uses it to update drawable dimensions. The session then
mistook the already-updated window geometry for matching pixels and committed
the layout before the client had allocated a resized DMA-BUF.

Engine-driven X11 resize now emits the standard mask-selected Present
configuration event in addition to core configure and expose delivery. Live
resize readiness resolves the actual DMA-BUF or CPU-buffer dimensions by
handle; target window geometry is no longer pixel evidence. Present submissions
for a partial multi-surface resize remain queued while the layout is pending,
and a mismatched source is rejected rather than clipped and reported as a
successful resize. Configure evidence is named delivery rather than
acknowledgement because X11 clients do not acknowledge core configure events.

The same trace exposed an independent output-service defect: the primary output
continued retiring mixed frames while output 2 retained only its startup
submission. Mixed and retained-resume frames are now queued on every active
output so each KMS head participates in the presentation lifecycle.

<!-- END IMPORTED BODY -->
