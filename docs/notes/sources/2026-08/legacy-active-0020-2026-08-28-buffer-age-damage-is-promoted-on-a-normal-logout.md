---
id: legacy-active-0020
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering", "tooling"]
---
# 2026-08-28: buffer-age damage is promoted on a normal logout

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 683–711. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Signed native archive `0002` promotes bounded buffer-age damage history on
  signed Sophia source `03f7060867289b78494d96170616f8c058defc45` and signed
  Hagia source `9c9a59061fd0d8e88310b764f7dd240e729fb035`. The session ran the
  bounded workflow to a normal logout with zero unexpected protocol errors,
  drained native scanout, and exact TTY recovery at `emergency=false`.
- The feature demonstrably fired rather than merely being enabled: 129 partial
  repaints beside 627 full fallbacks, and 201 history records with zero
  invalidations, which is what a session with no bundle rebuilds should look
  like. The three-slot ledger stayed exact underneath it -- 201 requests
  settling as 201 completions, no deferrals, no stale release, no slot leased
  at completion, watermark 6 across two heads. No stale regions were visible
  on screen, which is the check the machine cannot make.
- The verifier's schema-8 rule earned itself here. Requiring at least one
  partial repaint is what separates a promotion run from a run that merely had
  the switch set: without it, a session where every render fell back to full
  would have produced identical health, identical slot balance, and identical
  pixels, and promoted nothing.
- The first attempt failed for a reason outside the code and is worth naming so
  it is not re-diagnosed later. The proof phrase was never typed into the guide
  terminal -- the evidence shows a single key pressed on that surface, released
  at focus handoff -- so the client never redrew, the composed checksum never
  moved, and the input proof correctly refused. Every workflow shortcut
  committed normally around it. The `skipped_present=1761` in that log is a
  transaction identity, not a count of skipped presents; it is what an
  emergency exit with one Present in flight looks like, and reading it as a
  count would have sent the diagnosis into the renderer for no reason.

<!-- END IMPORTED BODY -->
