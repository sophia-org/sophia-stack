---
id: legacy-active-0199
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "validation"]
---
# 2026-07-25: Synchronous Initial Modeset Is Startup Presentation Proof

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6790–6838. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first crash-free four-Kitty capture still failed verification because
startup forced-detached one secondary scanout after 750 ms. The trace showed
that both initial modesets had completed successfully, but startup immediately
queued an identical event-bearing framebuffer on each output and required an
asynchronous callback. The primary produced that callback; the secondary
driver did not emit an event for its redundant commit. Recovery then abandoned
a healthy buffer, recreated the DRM session, and repeated the same baseline.

Initial modeset is a synchronous KMS operation. A successful return already
proves that output's first framebuffer was committed and is not an in-flight
page flip. Native output state now records that fact explicitly. Startup
readiness accepts either this synchronous proof or an accepted asynchronous
page-flip callback, and unchanged-frame suppression no longer schedules a
redundant baseline solely to manufacture an event. Reduced per-output evidence
names `proof=synchronous_modeset`; the physical verifier requires exactly one
record for each of the two outputs. Callback requirements remain unchanged for
subsequent asynchronous commits.

The first physical run confirmed this correction: both outputs emitted
synchronous proof, startup performed no recovery or forced detach, and maximum
submit-to-page-flip latency fell from 119 ms to 36 ms. It also exposed four
previously opaque native submit failures later in the workflow. Export counts
showed three CPU attempts and 159 successful mixed exports, but only 158 total
submissions including the two initial modesets. Native submission now emits a
reduced failure record containing the output, submit stage, and generic content
class so the next capture can distinguish export, framebuffer, atomic request,
and commit failure without native handles.

The next capture recorded zero native submit failures and clean native drain,
but final completion still applied the old callback-only independence check to
the unchanged secondary output. The completion invariant now accepts exactly
two balanced forms: one successful synchronous submission with no callback or
retirement, or an asynchronous stream with equal callbacks and retirements and
one retained displayed submission. Both forms still require a nonzero export;
mixed, incomplete, or callback-imbalanced lifecycles fail closed.

The following capture completed that lifecycle cleanly. Startup reported both
outputs ready with no recovery, input queue dwell fell within budget at 91 ms,
and every native submission, callback, retirement, and cleanup balanced. The
remaining verifier failure was one 202 ms submit-to-callback interval while a
four-to-three-window close synchronously waited for two X Authority configure
delivery acknowledgements. Adjacent page flips completed in roughly 25 ms.
The kernel path is therefore not the demonstrated source of this outlier; the
owner thread stopped polling DRM events while WM control delivery blocked.
The next latency correction must make configure acknowledgement intake
incremental or otherwise keep native event servicing live during that wait.

<!-- END IMPORTED BODY -->
