---
id: legacy-active-0131
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-08-05: black client content cannot authorize native recovery

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4255–4277. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first commit-pinned Kitty fallback attempt exited at its eight-second
readiness deadline. KMS ownership, synchronous modesets, page-flip callback,
Present retirement, focus delivery, and VT restoration had all succeeded.
Kitty's first mixed frame was valid but entirely black, and another primary
Present was already queued. The owner nevertheless treated the absence of
nonzero pixels after 1.5 seconds as a native transport failure.

That transition was invalid. The recovery drained and replaced the active KMS
and renderer owner while the runtime still retained the first Present's
renderer-image identity. The replacement worker could not resolve an image
owned by the discarded worker, reported `InvalidTarget`, and left the queued
client work unable to satisfy startup. A valid black frame is application
readiness evidence; it is not evidence that page-flip transport has stalled.

Startup native recovery is now admitted only by an objectively missing output
callback after the bounded 750 ms transport threshold. Valid black content
remains owned by the normal eight-second readiness deadline, allowing queued
client Presents to advance without destroying renderer state. A reducer
regression proves that elapsed time alone cannot authorize recovery and that
the missing-callback threshold retains its exact boundary.

<!-- END IMPORTED BODY -->
