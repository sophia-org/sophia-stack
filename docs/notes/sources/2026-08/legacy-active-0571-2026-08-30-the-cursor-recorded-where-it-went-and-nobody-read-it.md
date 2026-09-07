---
id: legacy-active-0571
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-30: the cursor recorded where it went, and nobody read it

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17843–17895. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`sophia_live_cursor_path` and `sophia_live_cursor_plane` were emitted across
the whole cursor tranche and read by no verifier, no reporter, and no gate.
Archives `0005` and `0006` promoted the atomic plane while carrying an
unread statement of which path the session actually took.

Worse than unread, it was unreadable for the case that matters. The record was
emitted only inside `if config.atomic_cursor`, so a session that opted out
emitted nothing at all, and absence could not be told from a session that never
reached readiness. It also named only where the cursor ended up, so a card
refusing the plane and an operator asking for the ioctl produced the same line.

The record is now emitted either way, at schema 2, naming `requested=` beside
`path=`. That makes the two cases distinguishable, and only one of them is a
defect: asking for the plane and taking the ioctl is the startup probe refusing
a card, which is the fallback this row deliberately retained; asking for the
ioctl and taking the plane is the preference being ignored.

Ownership is split because absence means two different things in two places.
`verify_hagia_native_session.sh` checks consistency when the record is present
and does not require it, since it also reads archives `0001` through `0003`,
which predate the record entirely and must stay independently verifiable; that
file cannot tell old evidence from evidence that lost a line.
`hagia_native_session_gate.sh` requires it, because it just built and ran the
binary that emits it, so there absence can only mean lost. Three matcher
controls pin the rules, and a fourth pins that stripping the record keeps the
archives verifiable.

A related false claim is corrected. `the_cursor_flags_need_native_scanout` said
the remaining case -- that `--legacy-cursor` sets the preference false -- was
"checked against the release binary with `--validate-session-args`". That flag
validates an argument vector and reads no field, so the case was uncovered
while reading as covered. It cannot be covered in-process: observing the field
needs a config that parsed, which needs `--native-scanout`, which is gated on an
environment variable a test may not set without racing every other test in the
binary. It is covered by evidence instead, which is what the new gate rule buys.

`--atomic-cursor` is retained rather than retired. It selects nothing now that
the default does, but it still refuses a session that cannot honour it, so a
harness measuring the atomic path cannot quietly measure the legacy one. The
comment above the flags said the opposite of the line beneath it -- four
superseded paragraphs, two of them documenting flags they had drifted away
from -- and now says what the code does.

The physical runner no longer requires HEAD to equal the locally known
origin/master, matching the direct-scanout gate. The same precondition remains
in `run_frame_fed_output_gate_tty4.sh` and `run_current_critical_path_tty4.sh`,
which are the same class and were left alone rather than changed inside a
Milestone 14 sweep; `package_live_session.sh` keeps it on purpose, because
packaging an installed release is the publishing question this rule was wrong
about being.

<!-- END IMPORTED BODY -->
