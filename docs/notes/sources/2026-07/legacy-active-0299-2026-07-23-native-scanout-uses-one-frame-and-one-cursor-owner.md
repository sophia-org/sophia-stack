---
id: legacy-active-0299
date: 2026-07-23
recorded_date: 2026-07-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-23: Native Scanout Uses One Frame And One Cursor Owner

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9294–9318. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical Kitty-only run proved that accepted KMS commits and routed
input were not sufficient visual evidence. Kitty created a 2540x1390 window,
completed DRI3/Present, and received keyboard and pointer routing, while the
operator still saw a frozen pointer beside the moving hardware cursor and no
Kitty content.

The exporter had independent pending CPU, DMA-BUF, and mixed-frame slots.
Consequently a mixed Kitty frame could export first while an older CPU frame
remained eligible for the idle service, and later CPU-only authority work could
replace retained DMA-BUF content. Native CPU composition also continued to
receive the pointer position even though the atomic cursor plane owned cursor
presentation.

Native pending scanout is now a single latest-wins slot per output. A mixed
frame supersedes its unsubmitted CPU precursor, and authority work preserves
the last GPU scanout whenever a committed or pending DMA-BUF layer cannot be
recomposed safely. Physical native sessions select hardware cursor ownership
explicitly and never bake cursor pixels into CPU backgrounds. Mixed submission
resolves the registered primary output by identity, and startup becomes ready
only when the exact Present transaction is the stable displayed content with
no newer primary frame queued or submitted. The Kitty-only physical capture
remains required before restoring xmonad.

<!-- END IMPORTED BODY -->
