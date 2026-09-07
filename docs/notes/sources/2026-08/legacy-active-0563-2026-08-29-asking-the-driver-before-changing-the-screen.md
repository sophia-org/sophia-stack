---
id: legacy-active-0563
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-29: asking the driver before changing the screen

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17509–17539. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Direct scanout hands the plane a buffer whose format, modifier, and plane
layout the compositor did not choose. Only the driver can say whether that is
scannable, and the only safe way to ask is a commit that changes nothing.
`TEST_ONLY` existed in the flag owner but had never gated scanout -- topology
validation was its only user -- so the submit policy gains a validating mode
and the scanout path gains an entry that asks rather than performs.

The request is cloned rather than rebuilt, which matters more than it looks.
Rebuilding would mean a second framebuffer, and the driver's answer would then
be about a different object that merely resembles the one that flips. Cloning
makes "the exact framebuffer" literal, down to the framebuffer id, and it is
possible only because the underlying request type is cloneable.

The first version of the test for this passed while proving nothing. The fake
device counted commits without inspecting flags, so removing `.test_only()`
-- turning the question into a real commit that changes the screen -- left
every assertion satisfied. That is the exact defect the feature exists to
prevent, and the test could not see it. The fake now counts commits that
carried the flag separately, and the mutation fails as it should. A device
that cannot tell a question from an answer cannot test something whose whole
purpose is the difference.

A refused validating commit returns the prepared scanout rather than
consuming it, because a refusal is an answer and the resources are still owed
a submit or a cancel. Errno is not inspected: the commit layer classifies
every failure identically, so a refused modifier is indistinguishable from any
other refusal, which is why the design falls back to composition rather than
retrying with a different guess.

<!-- END IMPORTED BODY -->
