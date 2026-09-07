---
id: legacy-active-0455
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-16: the output proof waited for a scene echo that need not exist

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13675–13699. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next mixed run showed both proof windows tiled on the large output while
  both smaller outputs remained black. This was the unchanged three-output
  startup topology, not a partial modeset: the log contained no output-role
  connection or proposal. All three startup heads presented and retired
  normally.
- Cause: after committing the proposal that first placed both surfaces, the
  reference policy decided whether to start its output role from the preceding
  scene snapshot. That snapshot still described one placed surface. No further
  policy cause was required after the second commit, so waiting for another
  snapshot blocked until the four-second policy socket timeout. The committed
  proposal already carried both authoritative placements.
- The resulting restart exposed a second ordering bug. The output service could
  already be blocked in `accept` when the owner spawned a replacement and
  queued its new PID. A fast replacement connected first and was correctly
  rejected against the old assignee identity, then the dual-role process exited
  again.
- Decision: the proof start barrier counts distinct surfaces in the committed
  proposal itself. Separately, output-service acceptance now has a synchronous
  pre-spawn pause. The owner closes the old connection and waits for that pause,
  spawns the replacement, then installs its exact PID; only processing that PID
  command resumes negotiation. This preserves exclusive-role authentication
  without relying on scheduler timing or weakening the peer check.

<!-- END IMPORTED BODY -->
