---
id: legacy-active-0141
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-05: composited Present owns a renderer snapshot

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4495–4528. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The retained DMA-BUF path had given X Present `Flip` semantics to an ordinary
composited frame. Sophia kept duplicated client DMA-BUF descriptors in scene
state after page-flip completion, so the client could legally reuse its pixmap
while later focus, layout, or damage repaints still sampled it. XLibre's copy
path idles the source after copying, and yserver independently sends Idle
before Complete for Copy; Flip is reserved for retaining the exact source.

The native renderer now captures each current DMA-BUF into a bounded,
same-format compositor-owned GBM image. The image is staged during rendering,
promoted only by the exact mixed-frame page flip, and rolled back on terminal
failure. Retained scene state contains image identity and geometry but no
client file descriptors. Output-target recreation may discard EGL imports
without discarding the renderer image. Replacement evicts the import before
dropping its backing store.

Page-flip retirement now reports Copy and releases the client source with Idle
before Complete. X-authority tracks the two phases independently, accepting
both Copy and future Flip ordering exactly once. Reduced snapshot metrics and
`PresentCopyOwnership.tla` cover capture, promotion, rollback, eviction, live
debt, and the rule that displayed composited content is compositor-owned.

The paired physical acceptance runs on `39f87687` passed. The bounded GLX run
remained visibly animated under continuous pointer motion, sustained 59.950
presentation FPS with a 16.685 ms p95 interval, and balanced 1,193 snapshot
captures and promotions with matching Copy, Idle, and idle-fence completion.
The four-Kitty mixed-scene run balanced 146 captures and promotions, reused
retained imports 356 times, and completed 146 Copy feedback cycles. Both runs
reported zero rollback, live snapshot or import debt, unexpected protocol
errors, and cleanup failure. This closes the snapshot correctness gate; the
conditional three-slot software scanout pool is not justified by these
results.

<!-- END IMPORTED BODY -->
