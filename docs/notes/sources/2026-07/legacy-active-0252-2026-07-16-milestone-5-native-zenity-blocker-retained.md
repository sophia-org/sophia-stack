---
id: legacy-active-0252
date: 2026-07-16
recorded_date: 2026-07-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-07-16: Milestone 5 Native Zenity Blocker Retained

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8329–8343. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Two guarded X13 classic shared-X attempts reached one Engine-owned native KMS
output and then showed a blank screen because Zenity aborted before presenting
a dialog. Both retained logs report GTK thaw-update assertions followed by
`BadRequest` at serial 304, request code 139 (`XFIXES`), minor code 0
(`QueryVersion`). The confined profile never started. The second emergency
chord restored KD mode 0, the exact termios state, keyd, and all Sophia
processes; the recovery record is complete. The earlier false keyd failure was
a service-start race, so the runner now waits boundedly for keyd after `sv up`.

The retained diagnosis was incomplete because wire-parse errors discarded the extension minor opcode and always encoded minor zero. A raw-minor trace on the X13 render-provider path reproduced XFixes request 11 (`SetRegion`) immediately after `CreateRegion`; `QueryVersion` had already succeeded. Sophia now retains extension minor codes, owns namespace-scoped XFixes region lifecycle, validates Present region references, and reclaims regions with the client resource range.

The first corrected run exposed a separate sentinel bug: raw region zero was converted with generation one and compared structurally to the generation-zero `NONE` value. Validity-based optional-resource checks fixed that rejection. The exact X13 sequence now accepts CreateRegion, SetRegion, DRI3 pixmap and fence resources, and Present with `first_error=none`. The non-KMS render-provider smoke reaches an Engine transaction but has no scanout consumer, so its remaining pixel-proof failure is expected and is not session evidence. Fresh guarded classic and confined hardware captures remain required before GTK promotion.

<!-- END IMPORTED BODY -->
