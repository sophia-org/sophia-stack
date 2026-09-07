---
id: legacy-active-0505
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "shell"]
---
# 2026-08-22: recovery geometry must leave room for chrome

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15471–15498. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed source `598fd27a76d538af682416ec1939260fd65690bd` produced and
independently re-verified mirror promotion archive `0002`. Its following mixed
run also proved the preceding fix: X Authority reported
`stale_target_retired` when a short-lived surface vanished before its metadata
rule arrived. Startup continued to the public reference WM.

That WM then failed four times with `RejectedInvalid`. The log made the
arithmetic plain. A 2560-by-1440 outer allocation became 2556 by 1436 after a
two-pixel chrome inset. Admission recovery held 2558-by-1438 client pixels, but
the layout coordinator compared them with the outer allocation and let them
stand. Restoring chrome produced a 2562-by-1442 policy rectangle, which the
outer-allocation reducer correctly refused.

Chrome conversion and constraint reconciliation had each been correct in
isolation; they disagreed about the bounds. Engine now exposes the content
rectangle for an outer allocation. Both the public-policy adapter and the
private compatibility path reconcile content against that inset rectangle,
then convert the committed result back to outer geometry. A regression retains
the dimensions from the physical failure and requires the round trip to end at
exactly 2560 by 1440. The supervisor's fallback error also refers to the policy
process rather than Hagia, since the failed peer here was the reference WM.

This is a local correction, not promotion evidence. It changes the executable,
so the signed mirror and mixed gates must run together again before the Hagia
and broker gate can begin.

<!-- END IMPORTED BODY -->
