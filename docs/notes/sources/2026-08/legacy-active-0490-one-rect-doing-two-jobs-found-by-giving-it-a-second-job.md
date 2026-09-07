---
id: legacy-active-0490
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: eae6775e52b4ecb0623da74d521fc416e329ca12
committed_at: 2026-08-21T17:14:55-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# One rect doing two jobs, found by giving it a second job

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14980–15023. The heading has no date. Its first recorded addition is commit
`eae6775e52b4ecb0623da74d521fc416e329ca12` (2026-08-21T17:14:55-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Centre-unscaled landed and the scaling was right. The window borders were not:
bands 1440 tall in a 1080-tall logical space, bands at x=4160 on a 2560-wide
framebuffer, and the same border appearing twice at coordinates 320 apart --
2238 against 1918, which is exactly the letterbox offset.

The cause is older than the policy. `project_border` never clipped at all, while
surfaces got a `native_clip` and the cursor got a `clip_to_target`. That much was
a plain omission. The deeper half is that the two things which *were* clipped
were clipped to `target.native_size`, the whole framebuffer, rather than to the
region the scene occupies on it. Every policy until this one projected the scene
across the entire head, so those were the same rectangle and nothing could tell
them apart. Putting a smaller scene inside a larger head separated them for the
first time, and content bounded by the framebuffer began painting into the margin
that exists to hold background alone.

So the border bug was three bugs wearing one symptom: borders unclipped,
surfaces clipped to the wrong bound, and the cursor likewise. Each was confirmed
load-bearing by reverting it alone and watching the test fail.

The part I got wrong and had to correct: bands must be clipped individually
rather than by clipping the `outer` and `inner` rects they are subtracted from,
and I wrote a comment asserting that clipping those first would invent a band
along the clip edge -- a border down the side of a window merely running off the
screen. Enumerating the cases showed that does not happen. Both rects clip to the
same boundary, so the subtraction goes to zero and the band simply vanishes,
which is correct. What actually goes wrong is the opposite: a window lying
entirely outside the scene clips to two degenerate rects whose difference is
still positive, and the band survives at its original off-screen coordinates. The
approach is wrong, the reason I gave was not, and a test written against the
reason I gave passed with the wrong approach in place.

That is the second time this session a test agreed with a claim without
testing it. The first was a mirror of a shader standing in for the shader. This
one was a case that could not distinguish the two implementations, and the way I
found it was by running the wrong implementation against the test rather than by
reasoning about whether it would pass. The case that does distinguish them is in
the test now, and both wrong approaches fail it.

Worth noting what the new policy actually did here: it did not introduce these
defects, it made them observable. A latent conflation stays latent while nothing
in the system can produce the input that separates the two meanings.

<!-- END IMPORTED BODY -->
