---
id: legacy-active-0484
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: c2b77ba03a2a6ad9e6c500921d52d1e8c52a99a8
committed_at: 2026-08-21T13:42:14-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# The test that could not fail, and the one that could

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14722–14750. The heading has no date. Its first recorded addition is commit
`c2b77ba03a2a6ad9e6c500921d52d1e8c52a99a8` (2026-08-21T13:42:14-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Four attempts at one defect, and the difference between the failed ones and the
last was never the reasoning -- it was whether the test could tell me I was
wrong.

The reproduction that matters is an ordering: the service writes a snapshot,
the client leaves without reading it, and the *poll that follows* meets the
departure. Every earlier test dropped the client first and published after, so
the write broke the pipe and retired the connection before any read happened.
Those tests passed with the code they covered deleted, which is the definition
of proving nothing, and twice I read that pass as confirmation.

With the order corrected the test reported exactly what the physical runs had:
`Failed { message: "Io(\\"Connection reset by peer (os error 104)\\")" }` where
`Disconnected` belonged. A unix stream has two readers here -- a blocking
`read_exact` for negotiation and proposals, and a non-blocking drain for the
poll loop -- and only the second is on this path. I had classified the first,
deleted the arm for the second as unreachable, and been wrong about which.

A departed peer reaches a socket three ways: a write to it breaks the pipe, a
read with nothing left ends unexpectedly, and a close that discarded frames
still queued for it resets the connection. One predicate covers them because
they are one situation, and every reader and writer now uses it.

The rule this leaves behind: a negative control is not a formality. Deleting
the code under test and watching the test still pass is the cheapest way to
find out that the test and the defect never met.

<!-- END IMPORTED BODY -->
