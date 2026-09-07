---
id: legacy-active-0573
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-30: xterm stopped because CopyArea never answered NoExpose

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17978–18037. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical CP-14.1 terminal run proved the new storage mechanism but
looked frozen. Its 20-second record carried 831 producer iterations into 64 CPU
updates, then only seven compositions and five flips, all near startup. The
operator's visual refusal was correct; positive update and patch totals had let
the old schema-3 reporter call startup activity a continuous workload.

An extended real-xterm trace located the earliest stopped stage. xterm reached a
successful core `CopyArea` request and then issued no next scroll. The GC's
default `graphics-exposures` value was true, but X Authority neither stored that
field nor emitted the core `NoExpose` event after a copy with no exposed region.
xterm was waiting for the server response its request required, so this was not
an Engine scheduler or backing-registry starvation defect.

`GraphicsContextState` now defaults `graphics_exposures=true`, create/change-GC
decode applies mask bit 16, and a successful `CopyArea` emits `NoExpose` only
when that flag is enabled. Wire, output-encoding, enabled/disabled behavior, and
text-scroll regressions cover the boundary. The paced real-xterm workload now
runs 80 stream rows at 20 ms and reaches 1,221 requests, 541 runtime commits, and
541 CPU buffers with `first_error=none`. That longer run also exposed a harness
error: its replay chunk of 64 left no room for fixed per-tick observations. The
chunk is now bounded below the runtime observation capacity rather than relying
on the capacity and the workload coincidentally matching.

The session no longer asks update totals to stand in for pixels reaching a
screen. A private bounded tracker starts at the exact readiness baseline and
observes post-readiness CPU intake, composition checksums, and primary native
retirement. Latest-wins intake settles every replaced pending update as
superseded; only an exact matching primary checksum presents the current update;
unchanged compositions supersede it. Completion reports accepted, presented,
superseded, pending, cadence, refresh, and largest intake-to-retirement latency
with no emitter-owned verdict.

The terminal reporter is schema 4 and owns the verdict. It refuses missing or
duplicate progress, startup-only activity, fewer than three changed primary
retirements, first or last source/display progress outside the one-second window,
one-second source or display gaps, incomplete accounting, pending or discarded
updates, and exact-retirement latency over two refresh periods plus rounding
tolerance. The prior physical failure shape is a retained negative fixture.

`ContinuousContentPresentation.tla` keeps intake, composition, submission,
kernel flip, and exact callback reduction separate. Its positive configuration
explores 646 generated states and 316 distinct states to depth 22. Four checked
negative configurations independently remove drain fairness, composition
fairness, supersession accounting, and exact retirement; the first two violate
liveness and the latter two violate their named invariants. Timing remains
empirical because a bounded state model cannot prove the physical scheduler's
one-second and refresh-relative budgets.

The first combined checker run exposed a harness boundary rather than a model
counterexample: failed TLC runs retain timestamp-named state directories, so two
negative controls started in the same second could collide before parsing the
second model. Every negative control now owns an isolated temporary directory,
and back-to-back execution reaches all four exact expected violations.

This closes the diagnosed protocol and evidence defects, not CP-14.1's physical
exit. A fresh clean signed TTY3 candidate must visibly scroll for the full
workload and pass schema 4 before the row moves.

<!-- END IMPORTED BODY -->
