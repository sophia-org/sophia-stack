---
id: legacy-active-0469
date: 2026-08-18
recorded_date: 2026-08-18
date_basis: first-heading-commit
date_commit: f6cf1ee29b24ada4b5d0800f037e25dbd8d2844e
committed_at: 2026-08-18T22:03:02-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# A proof that was never asked for is not a proof that failed

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14238–14261. The heading has no date. Its first recorded addition is commit
`f6cf1ee29b24ada4b5d0800f037e25dbd8d2844e` (2026-08-18T22:03:02-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The mixed gate reported `input_pixel_change=false input_text_match=false` on a
run where typing plainly worked -- twenty key events routed and a directory
listing on the extended screen. Neither field was lying, and neither was
answering the question a reader asks. They are results of the physical input
proof, which arms only when a session is given an expected text sequence, and
the mixed gate gives none. So `false` meant "never attempted" here and means
"attempted and failed" elsewhere, in the same field of the same line.

The pointer side already draws that line: `pointer_proof=enabled|disabled` sits
beside `pointer_pixel_change` for exactly this reason. The input side had no
equivalent, so the session now emits `sophia_live_session_input_proof schema=1
status=enabled|disabled` at startup, next to where the running line is written.

It is a separate line rather than a field on the completion line deliberately.
Adding a field there means bumping `sophia_live_session schema=16`, and ten
tools match that schema literally while a verifier gates a dozen checks on its
value -- a cost worth paying when the meaning of an existing field changes, and
not worth paying to add a fact that nothing else reports. A new line states the
missing fact once, duplicates nothing, and breaks no reader. The completion
site now carries a comment pointing at it, so the next person reading two false
booleans does not have to re-derive why.

<!-- END IMPORTED BODY -->
