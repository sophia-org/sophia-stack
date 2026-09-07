---
id: legacy-active-0160
date: 2026-07-30
recorded_date: 2026-07-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "architecture"]
---
# 2026-07-30: CPU patch residency is validated after transaction reduction

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5190–5246. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first commit-pinned Milestone 9 semantic rerun exposed a frontend-timing
race during two-xterm startup. Renderer residency was derived from committed,
staged, and current transaction buffers. A replacement or patch carried in an
update-only intake was not itself a residency root, so the replacement could
be installed and reclaimed before a later patch arrived. A superseded patch
whose base had already been reclaimed then terminated frame composition with
`MissingPatchBase`.

Current replacements and patches refresh a 16-handle renderer-private recent
update set. Production joins that bounded working set with committed, staged,
and incoming transaction roots, bridging update-to-transaction queue gaps
without granting scene visibility or retaining an unbounded X resource cache.
Production discards a late patch only when its base is absent; the strict
renderer registry still rejects missing bases for direct callers. After Engine
transaction reduction and residency reconciliation, production counts
committed CPU surfaces without buffers and fails the cycle before composition
if the count is nonzero. Thus superseded traffic cannot kill the session,
while a relevant missing buffer still fails closed instead of producing absent
or mismatched pixels.

Regressions cover consecutive replacement/patch intakes, the bounded recent
update set, late unrooted patch disposal, and the post-reduction
missing-committed-buffer check. The exact M7 two-xterm QEMU acceptance then
completed startup, both
resize epochs, pointer click/drag focus, output-edge reversal, workspace
projection, WM restart, launch/close, and clean logout.

The first commit-pinned M9 rerun subsequently exposed nondeterminism in the
QEMU pointer-focus harness: it reset only the horizontal coordinate, so an
inherited `y=0` placed the scripted click on compositor chrome and correctly
produced `button_suppressed reason=no_target`. Focus click and drag setup now
reset both axes, move 32 units horizontally, and use eight separate 16-unit
vertical steps before sending a separate gesture command. This avoids relying
on one acceleration-sensitive relative movement to leave the top edge.
The same rerun reached Firefox's refocus proof and showed that back-to-back
focus chords could race the X11 handoff, while a fixed two-chord pair could
start and end on the terminal. The proof now cycles one surface at a time,
waits for `focus_applied`, and sends an `r` probe accepted only by the page's
refocus stage. This proves an acknowledged focus change plus delivery to the
returned browser without depending solely on Firefox surfacing a DOM focus
event under headless QEMU.

The first physical input-latency sample then completed its exact key and pixel
proof but raced renderer teardown: KMS was drained while one asynchronous
renderer frame was still finishing, so image cleanup returned `WorkerPending`.
Renderer maintenance now settles and discards an unsubmitted worker result
within the existing one-second maintenance boundary before clearing cached
images. A failed or stalled worker remains a hard teardown error.

The following commit-pinned M8 rerun exposed a second restart ordering window:
the compatibility bridge could exit after policy-event polling but before a
request submit or completion poll. Those request-channel disconnects now enter
the same supervised, layout-preserving restart path as policy-channel
disconnects instead of terminating the live session.

<!-- END IMPORTED BODY -->
