---
id: legacy-active-0168
date: 2026-07-28
recorded_date: 2026-07-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "architecture"]
---
# 2026-07-28: Software Present Optimization Keeps Ownership Boundaries Intact

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5717–5819. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first correct standalone software-vkcube result retired 487 frames in
17,755 ms, about 27.4 FPS. Its retained maxima identified two independent
costs: CPU composition reached 17 ms and native upload reached 10 ms. The hot
path also cloned the source pixmap, copied a full immutable snapshot across the
authority boundary, allocated and checksummed a 2560-by-1440 output, cloned
that output for reporting, copied it again before GBM write, and recreated
mixed EGL/GBM render targets. Correct Present feedback had exposed throughput
rather than another policy or lifecycle fault.

The optimized path keeps the architectural boundaries unchanged. X authority
retains a bounded read-only SysV mapping, resolves XFixes regions, copies only
clipped update rows into its logical-window backing, and emits a fixed-capacity
immutable patch batch with a stable handle and monotonic generation. Renderer
intake prevalidates the complete batch before applying it, so malformed suffix
data cannot expose a partially updated frame. Engine sees only ordinary buffer
generations and damage; the WM remains blind to storage and protocol details.

Output composition now owns reference-counted bytes, reclaims the allocation
when no downstream lease remains, performs three exact startup pixel proofs,
and then derives bounded evidence from display-list generations and geometry.
The same-stride native CPU upload borrows those bytes instead of making another
full-frame vector. Mixed CPU/DMA-BUF composition retains its native target and
frame surface across same-size frames. Metrics expose replacement versus patch
traffic, payload bytes, evidence mode, and target reuse so a physical result
can prove that the intended path actually ran.

`tools/benchmark_sophia_vkcube_tty3.sh` supplies a fixed 900-frame workload.
`tools/report_sophia_rendering_performance.sh` computes FPS and p95 cadence from
Present UST values and can enforce a same-provider Xorg parity gate. Physical
results are pending. A retirement-fed three-slot CPU scanout pool is deliberately
conditional: per-frame GBM allocation should be replaced only if the new
measurements show it remains material, because recycling a scanout BO before
KMS retirement would violate the existing ownership proof.

The first bounded benchmark exposed a scheduling regression before it could
collect performance data. Sophia selected the 500-by-500 PresentedBuffer and
admitted its frontend surface, but submitted an unchanged empty CPU frame
between output-baseline readiness and visual-admission commit. That page flip
retired successfully; the visual transaction itself never committed, two
layout epochs timed out, and the eight-second `no_surface` startup guard
performed a clean shutdown. There was no panic, protocol error, native submit
failure, or emergency recovery.

The cause was an overloaded checksum contract. The first three compositions
used an exact output-pixel checksum, while later compositions used bounded
generation/damage evidence. Switching metric modes therefore changed the value
for identical pixels, and native scheduling interpreted the proof-algorithm
change as new content. Scheduling identity is now always derived from immutable
buffer generations, geometry, compositor primitives, and cursor state; the
metric mode changes only how nonzero output is counted. Regressions prove
identical display lists retain one identity across the warm-up boundary and
that an immutable generation change still advances it. A new physical
benchmark result remains pending.

The first retest removed that false page flip but still reached `no_surface`.
It exposed the underlying admission-order dependency more clearly. The
PresentedBuffer candidate was selected before the blind WM staged its pending
layout; the layout correctly retained that selected transaction, then
`AdmitSurface` completed. Control completion advanced Engine from
`ControlPending` to `AwaitingPixels`, but production resolved pending layout
only while processing a later authority batch. The software client was already
blocked waiting for Present feedback, so no later batch existed and the two
sides deadlocked until the startup guard fired. In the historical successful
run, the candidate happened to arrive after control completion and authority
processing called the reducer immediately.

The owner loop now runs one shared layout-progress service after every event
class that can unlock a pending epoch. Admission-control acknowledgement
advances only Engine admission state; authority observation, WM staging, and
control completion all invoke the same idempotent reconciliation. A ready
layout remains pending when the WM-update slot is occupied and commits after
that slot drains rather than failing or overwriting the older update. Thus
candidate-before-control and control-before-candidate converge without a
synthetic authority wakeup or hidden commit side effect.

That change advanced the next physical run through visual admission, layout,
focus, and eight CPU compositions, then exposed a second ownership defect as
`no_visual_detail`. Renderer CPU intake installed the quarantined snapshot and
immediately reclaimed every buffer absent from Engine's committed surface
snapshot. Absence was intentional before admission, so the later released
transaction referenced a buffer the renderer had already discarded. The
unchanged empty output correctly produced no new native submission.

CPU update application and residency reclamation are now separate operations.
The live layout emits a sorted, bounded handle snapshot for CPU buffers
referenced by pre-admission or release-pending transaction groups. Backend-live
joins those handles with committed surfaces and the current production batch;
renderer-live retains exactly that complete root set. Staged pixels may reside
in renderer-private storage but cannot become visible until their exact Engine
transaction commits. Removal, withdrawal, supersession, or timeout drops the
root and reclaims the buffer on the next cycle. This avoids another pixel copy
and keeps X resources, application identity, and layout policy out of Engine
and renderer state.

The console's contemporaneous elogind message was not causal. Session 214 was a
valid `_greeter` session on tty7 created after Sophia's clean tty recovery; the
Sophia run had already acquired input devices, modeset both outputs, and
executed for eight seconds. Its leader and `.ref` FIFO were live during
diagnosis, and the runtime record later disappeared normally. The warning
belongs to greetd/elogind handoff diagnostics, not rendering or admission.

<!-- END IMPORTED BODY -->
