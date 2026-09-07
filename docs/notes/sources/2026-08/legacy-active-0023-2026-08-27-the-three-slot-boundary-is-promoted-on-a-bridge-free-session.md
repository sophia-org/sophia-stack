---
id: legacy-active-0023
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "architecture"]
---
# 2026-08-27: the three-slot boundary is promoted on a bridge-free session

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 786–823. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Signed native archive `0001` promotes Milestone 14's three native frame slots
  on signed Sophia source `05d98e44981f5086fc8d2bd3ee4580944029a952` and signed
  Hagia source `9c9a59061fd0d8e88310b764f7dd240e729fb035`, against Hagia's
  tracked default profile. It is also the first evidence of Hagia driving the
  Sophia WM and shell protocols as a product session with no xmonad
  compatibility bridge anywhere in it.
- The workflow ran in order rather than merely completing. Each of three
  terminal launches committed its layout before the next was requested and
  reached an admitted surface; `Super+J` committed focus to a surface; one
  close and a normal logout followed. Committed layouts moved 0 to 1 to 4
  surfaces and back to 3 after the close, which is the workflow's shape and not
  just its endpoints.
- The slot evidence is exact: `worker_requests=263` settled as
  `worker_completions=263` plus `frame_slot_deferrals=0`, with
  `frame_slot_stale_releases=0` and `frame_slots_leased=0` at completion. No
  request was silently dropped, no stale release was accepted, and no page flip
  retired without releasing its buffer. The aggregate watermark of 6 is three
  slots on each of two presented heads: both reached full occupancy, which is
  what the model predicts under sustained presentation.
- Supporting facts from the same archive: separate protected broker and shell
  admissions with clean ready-to-stopped lifecycles, a 34-event physical text
  proof, 2 ms session-control queue dwell and 1 ms acknowledgement against a
  100 ms budget, drained native scanout with zero abandoned scanouts, clean
  session/topology/cleanup health, zero unexpected protocol errors, and exact
  TTY recovery at `emergency=false`. Four stale policy responses were ordinary
  scene races; each re-armed and committed.
- The verifier was corrected after the run it judged, so the promotion rests on
  facts in the evidence rather than on the tooling that read them. The identity
  chain is unbroken independently of that correction: the gate wrote the
  identity line after re-verifying both signed commits and all three binary
  digests, the archiver checked those digests against the evidence's own
  identity line, and the archive verifier re-checked the whole record from the
  archive alone. What the run did not demonstrate is the gate printing its own
  pass line end to end; that is a statement about the harness, and the next
  native run earns it without a session spent on it alone.

<!-- END IMPORTED BODY -->
