---
id: legacy-active-0529
date: 2026-08-24
recorded_date: 2026-08-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-24: the text proof counted a key that types nothing

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16231–16252. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Two consecutive signed runs reached the final step and ended there. Both had cleared
everything the previous fixes addressed -- no protocol errors, no page-flip stall, the
browser admitted, the switcher activated twice -- and both died on the physical text
proof. The first was an ordinary typo. The second was not: the operator clipped caps
lock while reaching across the home row, and the proof treated it as a text event and
failed on the mismatch.

The exclusion list held the ordinary modifiers and omitted the locking ones. Its own
comment already gave the rule -- a transition that produces no application text sits
outside the exact sequence -- and caps lock, num lock, and scroll lock all satisfy it.
They are now excluded. Locking one on still fails the proof, and should: every
character typed after that is genuinely a different character, which is exactly what
the sequence is there to catch. What is no longer counted is the transition itself.

Whether a mismatch should reset the sequence rather than end the session is still
open. The proof asserts that an exact phrase reached the application in order, and a
reset that still requires the whole phrase contiguously would assert the same thing
while surviving a slip -- provided the reset count is recorded, so an intermittent
routing fault reads as repeated resets instead of disappearing.

<!-- END IMPORTED BODY -->
