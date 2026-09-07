---
id: legacy-active-0483
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: a6c18979958eeba7f8f643eba1d2c3c1925599ba
committed_at: 2026-08-21T11:48:39-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# The socket the next client needed

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14695–14721. The heading has no date. Its first recorded addition is commit
`a6c18979958eeba7f8f643eba1d2c3c1925599ba` (2026-08-21T11:48:39-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The disconnect fix did not stop the cascade, and the reason is worth recording
because I stopped one step short of the failure twice.

Writing to a departed client was one of five write sites; I guarded two. The
log then showed the same `Io("Connection reset by peer")` degradation, and I
guessed the read path next -- wrote the classification, wrote a test, and the
test passed with the classification removed. That is the whole answer to
whether I was right: a unix stream reports a departed reader on the *write*
side, and a reader that closed cleanly is an ordinary end of stream, so the
read arm was unreachable code with a test that proved nothing. Both are gone.

Writing a test that reproduces the production chain found the rest of it in one
run. Settling to a client that has left retires the connection -- and then the
owner sends its next command, which met `return Err("output settlement arrived
without a client")`. The service ended, its listening socket went with it, and
the restarted policy found `Io(NotFound)` three times until the supervisor gave
up. The owner commands from its own turn and learns about a departure on the
next one, so an answer for a client that has already gone is ordinary; it is
dropped now.

Every write shares one retirement rule, because which frame happens to meet a
close is timing rather than meaning. The negative controls are the part worth
keeping: a test that passes when the code it covers is deleted is not evidence,
and running that check is what separated the real fix from the invented one.

<!-- END IMPORTED BODY -->
