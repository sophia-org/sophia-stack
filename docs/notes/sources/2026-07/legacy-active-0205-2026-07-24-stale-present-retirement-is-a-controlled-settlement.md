---
id: legacy-active-0205
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-24: Stale Present Retirement Is A Controlled Settlement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6992–7017. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

An xmonad physical run exposed a normal client-exit race: Kitty exited after
Present transaction 778 entered KMS but before its page-flip callback. The
surface-removal batch advanced Engine state, so retirement correctly rejected
the prepared candidate as `RejectedStaleSurface`; the live backend incorrectly
promoted that controlled result into a fatal session error.

Prepared retirement now remains ordered through the production coordinator:
Engine revalidates first, then the backend maps a committed result to
Present `Flip` and a rejected result to `Skip`, followed by `Idle` and exact
resource release. A rejected retirement never becomes a stable or focusable
surface and the current Engine snapshot is projected unchanged. Missing or
duplicate presentation resources remain fatal because they indicate broken
ownership rather than an ordinary asynchronous race.

CPU-frame preservation now follows the post-batch active transaction set
instead of a removed pre-batch DMA-BUF surface. Removing the last GPU surface
therefore queues the current CPU snapshot behind the in-flight frame, allowing
the asynchronous service to replace exited-client pixels after retirement.
Focused regressions reproduce the removal-before-retirement ordering without
physical hardware and require `Skip`, `Idle`, unchanged Engine state, and
exactly-once resource cleanup. This supersedes the earlier wording that stale
prepared retirement invokes no feedback callback: it invokes no successful
`Flip`, but it must still settle backend and protocol lifetimes.

<!-- END IMPORTED BODY -->
