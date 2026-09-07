---
id: legacy-roadmap-0025
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Secondary Development Tooling

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 2671–2728.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

<!-- BEGIN IMPORTED BODY -->

Interactive QEMU is useful for reproduction but is not a physical daily-driver
blocker. Work on it only when it shortens an active milestone.

- [ ] Fix the load-sensitive flake in `sophia-x-authority`'s `x11_wire` suite.
  Diagnosed, not fixed. It is **not** a timeout: under a parallel build,
  `routed_service_confines_input_and_control_to_two_workers_and_drains` fails at
  `socket_observation.rs:710` with `BadWindow` (3) where `BadAccess` (10) is
  expected, and `routed_lifecycle_events_follow_structure_and_substructure_masks`
  and `configured_present_child_receives_xlibre_ordered_geometry_notification`
  fail the same way intermittently. The shape is a cross-client race: the first
  client writes four requests and never reads, so nothing establishes that the
  server processed its `CreateWindow` before a second client refers to that window.
  The obvious fix is wrong. Adding a round-trip barrier on the first client — a
  request against an absent resource, whose error reply proves everything earlier
  was processed — makes the test fail **deterministically** rather than fixing it.
  So per-connection request ordering is not the whole mechanism, and the routed
  two-worker path or the confined-namespace boundary between the two clients is
  involved. That is where the next attempt should start, and it is worth more than
  the failed patch, which was reverted.
  A second mechanism is now recorded, and it rules out the tempting fix.
  `configured_present_child_receives_xlibre_ordered_geometry_notification` fails
  under full-workspace load inside `read_x_reply`: the reply's 32-byte prefix
  arrives and the body never does, until the **10-second** `SOCKET_IO_TIMEOUT`
  expires. Raising timeouts is therefore not the answer — ten seconds is already
  generous, and a reply that is half-sent for ten seconds is a server withholding a
  body, not a machine that was briefly busy. Both mechanisms point at the same
  place: what the routed workers do when more than one client is live.
  Note also that a failure here truncates the workspace run, because cargo stops
  before the remaining test binaries. A full-suite total that drops by roughly
  thirty-six tests is this flake, not a missing suite.
  A third attempt narrowed the mechanism to `read_x_reply`
  (`tests/x11_wire/support_extensions.rs`) and then failed too, which is the most
  useful thing recorded here. That helper reads 32 bytes and derives a body length
  from bytes 4..8 whatever the record is. Only a reply has a body: an error's bytes
  4..8 are its offending resource id, and an event has none at all. So a non-reply
  record yields a nonsense length and a read that blocks for the full timeout.
  What makes it stubborn is that the two failing tests **depend on that mis-parse**.
  Instrumenting the helper to reject non-replies showed both reading Sophia Present
  **event type 35** through `read_x_reply` on *every* run, not only under load — one
  call site even names the result `present`. The mis-parse is load-bearing: those
  events carry zero in bytes 4..8, so the bogus length is zero and the helper
  happens to return the event intact. Returning non-reply records whole, which is
  what the wire actually says, also broke both tests deterministically, so they rely
  on more than the zero-length coincidence.
  Two conclusions. The fix is **not local to the helper** — those two tests must be
  rewritten alongside it, which needs someone to work out what they intend to assert
  about Present events versus replies. And raising timeouts remains wrong: the
  records arrive promptly, they are simply parsed as the wrong kind.
  Both attempted fixes were reverted. Baseline is 178 passing.
  A suite that fails for non-reasons erodes every other claim in this file, so this
  is worth closing even though it is not on the critical path.
- [ ] Complete one human-visible `xmonad-interactive` capture proving pointer
  movement, terminal launch, typed text, focus change, application close, and
  clean manual shutdown. The fail-closed verifier, mutations, and RFB capture
  already pass.

<!-- END IMPORTED BODY -->
