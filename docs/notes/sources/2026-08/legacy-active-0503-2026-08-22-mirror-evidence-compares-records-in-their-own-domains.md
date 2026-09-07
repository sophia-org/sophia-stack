---
id: legacy-active-0503
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-22: mirror evidence compares records in their own domains

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15419–15451. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical run on `d5b42c3c5132e8bf88c394856a96e68fe4e1acff`
completed cleanly and passed the operator's visual check, but the verifier
rejected it. The diagnostic archive is run `0027`; it is not promotion
evidence. Replaying that run exposed two stale assumptions in the verifier.

Composition sampling and pixel-region records describe the terminal surface,
not necessarily the full framebuffer. The verifier now pairs the primary's
exact sampling with the secondary's sharp sampling at one scene generation,
requires the same source extent and an exact 3:4 target ratio, and attributes
pixel records by their explicit output extent while retaining compatibility
with the older full-frame target field.

The bounded session's `cpu_checksum` and a head's
`logical_content_checksum` are also different hashes. The former covers the CPU
replay report; the latter covers the Engine's logical scene plan. Equality was
coincidental in the fixture and false in the real run. The verifier still
requires a nonempty CPU composition, then causally binds one nonzero logical
checksum through both heads' plans, queues, submissions, page flips, and
retirements. The saved diagnostic run passes with those corrections, but the
physical gate remains open until the corrected verifier and exact signed source
produce a new passing archive.

The next signed attempt, diagnostic run `0028`, exposed one more representation
mistake after the runtime and visual proof again passed. Its CPU checksum,
`13820492447675412724`, is a valid `u64` but exceeds Bash's signed arithmetic
range, so `(( checksum > 0 ))` treated it as negative. Positive telemetry fields
are now validated as nonzero decimal strings instead of shell integers. The
fixture suite accepts `u64::MAX` and still rejects zero. Run `0028` then passes
the verifier end to end, but remains diagnostic because the gate correctly
recorded the verifier's original rejection.

<!-- END IMPORTED BODY -->
