---
id: legacy-active-0165
date: 2026-07-29
recorded_date: 2026-07-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-29: GLX Freeze Was a Retained-Buffer Race on the Owner Thread

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5518–5620. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The failed physical `glxgears` run completed and retired two mixed frames. On
the next repaint Sophia emitted X Present `Copy` completion for a DMA-BUF it
continued to retain and sample, then entered a production `glFinish`. That
finish blocked the session owner for 10.097 seconds. Radeonsi reported a
cancelled command stream, lost context, and guilty hard recovery; during the
same interval the owner could not service the hardware cursor, physical input,
VT control, or X traffic. The static gears, frozen pointer, and session abort
were one failure, not separate input and animation bugs.

Xserver's Present implementation confirms that `Copy` means the server has
copied the pixmap and may idle it. Sophia had used that protocol result for a
zero-copy retained compositor lease. The protocol-neutral backend contract now
uses `Retained`, `Copied`, and `Skipped`. The X authority maps `Retained` to
Present `Flip`; software snapshots alone use `Copy`. Idle remains behind exact
KMS retirement.

Production rendering no longer executes `glFinish` in composition, cache
eviction, cache clear, or full-screen DMA-BUF drawing. EGL swap and KMS
retirement carry the normal same-GPU ordering; diagnostic readback is bounded
to the initial nonzero proof.

The stronger containment boundary moves production EGL/GL/GBM work to a
bounded renderer worker after the initial modeset. The worker receives an
owned immutable frame and retains the native BO under a lease ID. The session
owner receives only duplicated DMA-BUF FDs and a descriptor, so a driver stall
cannot stop input or cursor service. A request becomes a deferred scanout while
pending, reports a soft stall at 100 ms, and is quarantined after 1 second
without fake Present feedback. Completion metrics now include worker requests,
completions, failures, stalls, release-queue failures, and maximum request age.
The remaining multi-output optimization is to share one worker among outputs
that use the same render device.

The first worker-enabled physical rerun remained static but did not reproduce
the 10-second owner-thread stall. It exposed two asynchronous-boundary defects.
While the mixed Present was rendering, a CPU repaint replaced the output's
pending content label; the resulting KMS retirement was therefore recorded as
CPU and could not satisfy the exact visual-admission transaction. A second CPU
fallback then produced a valid GEM-backed scanout buffer whose DMA-BUF FD
export was unavailable. The old synchronous submit path correctly used the GEM
descriptor in that case, but the worker had incorrectly made FD export
mandatory and rejected the frame.

The first repair preserved mixed content ownership against CPU replacement
while the GPU frame was in flight. It also treated PRIME export as optional
and retained a descriptor fallback. That transport choice was provisional;
the later lockup evidence below showed that PRIME and shared-file descriptors
must be mutually exclusive topology modes, not preferred/fallback paths.

The next physical run proved those repairs: the seat and DRM device became
active, mixed transaction 46 reached KMS, and its page flip retired. The
concurrent elogind `failed to add session ... to hash map: File exists`
message was therefore nonfatal session-manager noise, not the presentation
failure. Admission still remained `not_committed` because the Present
scheduler had only queued and submitted states. A deferred renderer-worker
export was popped from the queue as if it had failed; when KMS later accepted
and retired that exact frame, no scheduler record remained to connect the
retirement to its prepared Engine commit.

The scheduler now retains one mutually exclusive in-flight record with
`Rendering` and `Submitted` variants. The full immutable Present record moves
`queued -> rendering` when the worker accepts it, `rendering -> submitted`
only when KMS accepts the returned buffer, and leaves the scheduler only on
retirement or controlled failure. A newer client frame may remain queued, but
cannot replace the prepared transaction, displayed layer, or resource
ownership of the frame already crossing the asynchronous boundary.

The following physical run proved admission and protocol feedback through two
advancing mixed Flip retirements, then froze the graphical session. Its log
identified two deeper ownership defects. Output content and damage still had
only pending/submitted slots, so newer compositor work relabelled transaction
51's worker result as `RetainedMixed`. More seriously, the worker and KMS use
duplicated descriptors for the same DRM file, but submission preferred a PRIME
round trip. Importing that exported DMA-BUF back into the same DRM file may
return GBM's existing GEM handle; KMS resource cleanup then closes a handle
still owned by the renderer. The observed sequence was framebuffer resource
creation failure, `DmaBufImageCreateFailed`, repeated EGL target replacement,
then a submitted page flip that did not retire before emergency recovery.

Output damage and native content now carry their own `Rendering` slot beside
`Pending`, `Submitted`, and `Presented`. A worker request moves the exact
snapshot and content into that slot; worker completion promotes that same
record even if newer work is queued. Scanout transport now declares its DRM
file topology explicitly. The current shared-file production path submits GEM
descriptors directly and never PRIME-imports its own buffers. A future
independent render-node path must provide PRIME FDs and fails closed if they
are unavailable; the two modes are not runtime fallbacks for one another.
A 500 ms page-flip watchdog terminates the session so a lost DRM event cannot
leave the graphical seat waiting indefinitely.

The operator reported the resulting graphical freeze as a complete system
lock. The process-local page-flip watchdog is necessary but cannot be the only
recovery boundary because it still depends on the session owner being
scheduled. The development TTY launcher therefore accepts an independent
wall-clock deadline. Its separate shell process records the deadline, sends
TERM and then KILL to the complete Sophia session process group, and lets the
existing parent cleanup restore keyboard, console graphics mode, keyd, and the
display manager. The bounded `glxgears` proof enables that deadline at workload
duration plus five seconds. This is containment, not evidence that the
rendering defect is fixed, and it cannot recover a kernel-wide scheduler
failure.

<!-- END IMPORTED BODY -->
