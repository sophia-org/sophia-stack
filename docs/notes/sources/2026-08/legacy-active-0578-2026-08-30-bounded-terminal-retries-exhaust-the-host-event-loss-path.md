---
id: legacy-active-0578
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-30: bounded terminal retries exhaust the host event-loss path

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18139–18176. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The full bounded terminal run `20260830T235422Z` on signed commit
`d63d0970d8854de0832879d74f3aae1f6d31fb24` exhausted its eight allowed retries:
nine physical sessions, nine attributed hard stalls, and no schema-4 or visual-
confirmation opportunity. The stalled head and completed retirement counts
were 1/85, 1/415, 1/440, 2/10, 1/48, 2/2, 1/118, 2/325, and 2/316. Movement
between heads and the wide count range are inconsistent with one fixed
retirement index as the trigger.

Every attempt carried exactly the same reduced event attribution:
`poller_pending=0 poller_routes=2 poller_last_read=WouldBlock
poller_last_decoded=0 poller_last_rejected=0`. At the hard-stall boundary Sophia
had two installed routes but no queued callback, no decoded callback, and no
rejected callback. The peer CRTC had continued retiring before the affected
head stopped. The complete kernel delta was empty. This is the known below-
process DRM/KMS event-loss signature, not evidence of an Engine-owned queue,
decoder, or routing loss.

The gate therefore ended with `attempts=9`, `stall-retries=8`, failure
`page_flip_stall_retry_budget`, and no performance or visual verdict. Every
session used bounded forced detach, restored termios without emergency
recovery, and handed control back to the display manager. CP-14.1 remains open:
continuous source-to-physical retirement has still not been proved, but
repeating the same gate against unchanged local kernel/driver state is no
longer a productive experiment. The next physical run is admitted only after
the local DRM/KMS event-delivery state is repaired or materially changed.

The repetition also explained the perceived pause after `Restoring greetd...`.
xterm's command child can outlive the session application's process group when
the X server disappears. Its nested 20-second producer retained the terminal
gate's logging descriptor, so `tee` could not observe EOF even though greetd
was already restored. The bounded xterm probe now owns a parent-death watchdog
and signal cleanup for that nested timer. A no-X regression deliberately
orphans the fake xterm command child and requires the 20-second probe to return
failure in under five seconds. This fixes retry handoff latency; it does not
alter the page-flip classification or promote the failed run.

<!-- END IMPORTED BODY -->
