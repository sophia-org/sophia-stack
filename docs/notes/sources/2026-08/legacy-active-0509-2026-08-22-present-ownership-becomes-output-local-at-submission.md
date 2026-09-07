---
id: legacy-active-0509
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "architecture"]
---
# 2026-08-22: Present ownership becomes output-local at submission

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15590–15617. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed source `231847abefca878e2aa40794f902ac28468df447` produced and
independently re-verified mirror archive `0006`. The following mixed run,
`/tmp/sophia-mixed-output-centered-20260822-165948.log`, committed and settled
the three-head topology before ending with `native output submission does not
match its Present ownership`.

The frame order identifies the missing phase. Present transaction 565 reserved
frame 55 for mirrored output 1 and frame 56 for output 2. Output 2 submitted
frame 56, output 1 submitted frame 55, and output 2 then retired frame 56. While
output 1 still awaited its primary-head retirement, output 2 submitted ordinary
topology repaint frame 60. The submission check used `in_flight_frame(output)`.
That query intentionally retains every output's frame until the whole
transaction cohort retires, so it still returned frame 56 and misclassified
frame 60 as displaced Present ownership.

The scheduler now exposes the narrower fact the submission boundary needs:
`unsubmitted_frame(output)`. It returns the reserved Present frame only until
that output acquires KMS ownership. Other outputs may keep the cohort alive for
joined feedback and retirement without blocking ordinary successors on an
output that has discharged its submission obligation. Tagged Present content
still requires an exact frame and transaction match, so the correction does not
weaken protocol feedback ownership. The external scheduler regression holds one
output across submission and retirement while its peer remains unsubmitted.
This is a local executable correction; mirror archive `0006` does not promote
its successor, and the physical sequence must run again.

<!-- END IMPORTED BODY -->
