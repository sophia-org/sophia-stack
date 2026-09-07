---
id: legacy-active-0129
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-06: Resize storms advance only after exact pixels commit

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4207–4231. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The former `--inject-surface-resize` proof could exercise one transition but
could not distinguish a robust resize pipeline from one that happened to
recover once. Its bounded sequence extension retains one active proof at a
time and advances only after the matching transaction has delivered a client
configure, committed the resize epoch, and installed pixels at the exact
target size. This preserves the production visual-admission contract instead
of using sleeps or overlapping speculative transactions to manufacture load.

The new diskless `xmonad-resize-storm` profile continuously redraws an Xterm
through the CPU/SHM patch path while cycling 12 policy sizes across two virtual
outputs. Two consecutive production runs committed every
request→layout→resize-epoch→pixel chain without timeout, rollback, authority
drop, stale WM response, or restart. Both observed partial repaint and another
retired frame after the final resize, then shut down with balanced renderer
requests and completions, zero live snapshot/import-cache entries, and clean
application, native, input, and WM ownership.

The verifier is causal rather than count-only. Mutations remove an exact-pixel
commit, alter its dimensions, inject a layout timeout, remove post-storm frame
progress, or leave worker ownership outstanding; each must fail. This closes
the software-present resize workload. It deliberately does not stand in for
the remaining multi-producer DMA-BUF contention proof.

<!-- END IMPORTED BODY -->
