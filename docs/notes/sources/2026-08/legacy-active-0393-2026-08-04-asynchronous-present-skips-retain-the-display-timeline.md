---
id: legacy-active-0393
date: 2026-08-04
recorded_date: 2026-08-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-04: asynchronous Present skips retain the display timeline

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11931–11956. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first default `vkcube --wsi xcb` run after the GLX cursor-cadence repair
  produced a visible but static cube. The 500-by-500 surface submitted one
  Present while its manage transaction held a resize epoch. That epoch timed
  out, aborted the queued Present, retained its pixels through the coherent CPU
  recovery snapshot, and later committed those pixels without receiving a
  second Vulkan frame.
- The abort did route Complete/Skip and Idle exactly once, so resource release
  was not missing. It instead stamped the asynchronous completion with
  `UST=0, MSC=0`. XLibre executes a skipped pixmap Present against the current
  CRTC UST/MSC. Yserver independently replaced zero-clock Present completions
  after real clients rejected them as invalid. Sophia's successful native
  completion path already used the kernel page-flip clock; only policy-driven
  rejection reset the client-visible timeline.
- The protocol-neutral live feedback coordinator now retains the most recent
  successful display sample. Scheduler rejection, supersession, layout
  rollback, native detach, and shutdown skips reuse that sample rather than
  fabricating a new origin. Page-flip success continues to refresh the sample,
  and early startup retains the existing zero fallback until a real display
  sample exists.
- A crate-boundary regression completes one frame at a nonzero kernel sample,
  asynchronously skips its successor, and requires Complete/Skip plus Idle at
  the retained UST/MSC with all presentation resources retired. The packaged
  physical vkcube rerun remains the acceptance boundary.

<!-- END IMPORTED BODY -->
