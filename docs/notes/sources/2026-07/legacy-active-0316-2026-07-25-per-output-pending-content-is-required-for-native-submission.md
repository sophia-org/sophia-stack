---
id: legacy-active-0316
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-25: Per-Output Pending Content Is Required For Native Submission

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9897–9921. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The four-Kitty proof exposed a false clean-shutdown result. Output 2 accepted a
CPU commit after the startup terminal exited but never produced its final
page-flip callback. Logout therefore timed out and abandoned one scanout, while
the focused verifier accepted the detached runtime because its final
`native_in_flight` value was false.

The deeper error preceded logout. A primary-owned mixed Present queued content
only for the primary output but still executed the native scanout tick for
every output, producing repeated secondary submissions with `content=None`.
CPU composition also requeued the unchanged secondary marker whenever only
primary content changed. Native submission now requires explicit pending
content, primary mixed Presents service only their owning output, and typed
CPU-content reduction suppresses a frame only when the same CPU content is
already pending, submitted, or displayed. A mixed-to-CPU correction is never
suppressed by an old checksum.

Native suspension now reports a typed drained, timeout-detach, or revoked-seat
outcome. Forced detach remains a bounded liveness mechanism, but it is not
clean evidence. The four-Kitty verifier requires an exact drain, zero
abandoned scanouts, balanced per-output callbacks and retirements, and no
empty-content submission. Engine remains application-neutral; the CLI only
orchestrates and reports the Engine-owned lifecycle.

<!-- END IMPORTED BODY -->
