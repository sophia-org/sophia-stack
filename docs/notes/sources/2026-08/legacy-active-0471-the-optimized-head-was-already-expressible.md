---
id: legacy-active-0471
date: 2026-08-19
recorded_date: 2026-08-19
date_basis: first-heading-commit
date_commit: b216921282b1622cf27c1ce572734e5525846599
committed_at: 2026-08-19T07:04:29-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# The optimized head was already expressible

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14296–14324. The heading has no date. Its first recorded addition is commit
`b216921282b1622cf27c1ce572734e5525846599` (2026-08-19T07:04:29-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Implementing the macOS model turned out to be mostly a matter of noticing that
the protocol already carries it. A mirror group is proposed as one logical rect
plus a mapping per member -- `Exact` or `Fit` -- so "optimize for this head"
means sizing the group to that head's mode and marking it exact while the other
members fit themselves to it. The compositor needs no change at all: head plans
are already built against the group's logical size, which is why head 3 has been
reporting `mapping=fit downsampled=1` all along.

So the work was in the reference policy, which hard-coded the primary as exact
and the member as fit, and in the gate, which now exposes the choice as
`SOPHIA_MIXED_OPTIMIZE_FOR_LABEL` and names it in the operator prompt. The
default is the primary, which is what every run so far did without saying so.

One behavioural note worth recording: the group is now sized by the optimized
head's current mode rather than by whatever logical rect the group already had.
Those coincide on this rig, and the former is the meaningful rule -- a group
optimized for a head that is not its size would be a contradiction. It also
keeps the extended output's origin honest, since it starts where the mirror
group now ends.

What this does not do is re-mode anything. Choosing the smaller member means
the larger panel keeps its own mode and receives an upscaled image, which is
macOS's trade. Windows and X instead restrict the desktop to a mode every
member supports so nothing resamples; that remains a separate roadmap item,
because it reaches into topology candidates and rollback rather than into one
policy proposal.

<!-- END IMPORTED BODY -->
