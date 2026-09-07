---
id: legacy-active-0555
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-08-28: the shared-worker model, and two things it refused first

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17213–17257. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Milestone 14's next row coalesces every output of a DRM device group onto one
renderer worker, and the milestone requires the model before the semantics
change. `validation/tla/SharedWorkerService.tla` is that predicate: one
worker, one FIFO queue, a latest-wins pending cell and an at-most-one-in-flight
gate per output, explicit routing of each result to the output that asked, and
a pass-over count that states skew structurally. 5,095 generated states,
2,135 distinct, depth 16.

The row's three named guarantees are all things a per-head worker got for
free by position. With one request outstanding, the next message on the
channel was necessarily its answer and a mismatched request id was a fault;
one output could not occupy a thread it did not share; and no head could be
starved by another because none competed. Sharing the thread removes the
position argument, and each guarantee has to be stated instead.

Two drafts failed before the model held, both usefully. `ServiceSkewBounded`
first compared service counts directly, and TLC refuted it in fifteen steps
with a trace where one output simply had nothing to draw: an idle head falls
as far behind as the run is long, and calling that skew fires the invariant
on healthy behaviour. Skew is only meaningful against an output that is
actually waiting, so it now counts pass-overs. The environment was wrong the
other way: `Compose` refused to offer a generation while that output had a
render in flight, which is not what the code does -- `replace_pending_frame`
fills the pending cell without consulting the worker, and that is precisely
why supersession exists. The too-strict environment made the submission gate
unreachable, so its negative control could not fail. Correcting it doubled
the state space and made the gate load-bearing.

Three controls hold the result: routing every result to one fixed output
violates `ResponsesRouteToTheirOutput` at depth 5, dropping the per-output
in-flight gate violates `OneInFlightPerOutput` at depth 5, and letting the
worker take any queued entry rather than the head violates
`ServiceSkewBounded` at depth 11. The third is the one to keep in mind while
implementing: nothing about sharing a worker forces fair service, and a
scheduler that picked its next render by scanning outputs in a fixed order
would starve the second screen while satisfying every other invariant.

The pinned jar was on this host all along, at `~/tmp/tla2tools-1.7.4.jar`;
the copy under `~/src/Specula/lib` that `tools/check_tla.sh` first refused is
a different build, which is what the checksum pin exists to catch. The full
harness then ran all twenty-nine models under the pinned TLC 2.19 with no
error, and `SharedWorkerService` reproduced exactly the figures above.

<!-- END IMPORTED BODY -->
