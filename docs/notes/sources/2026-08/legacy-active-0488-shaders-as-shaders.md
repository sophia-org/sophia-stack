---
id: legacy-active-0488
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: 930304b0ca5cbd03f678d6a1eca39c8f26d6036f
committed_at: 2026-08-21T15:13:06-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# Shaders as shaders

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14898–14934. The heading has no date. Its first recorded addition is commit
`930304b0ca5cbd03f678d6a1eca39c8f26d6036f` (2026-08-21T15:13:06-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The GLSL was three `const &str` literals in a Rust file. Moving it into
`.vert`/`.frag` files alongside is mostly the obvious tidying -- highlighting,
diffs, a reviewer seeing GLSL as GLSL -- and `include_str!` keeps it embedded at
compile time, so nothing is read at runtime and there is no asset to lose.

The reason worth recording is the one that is specific to this renderer. A
shader that fails to compile is not fatal here, deliberately: the pipeline logs
`status=unavailable`, falls back to the direct program, and the session keeps
running with its filtering uncorrected. That is the right behaviour when a
driver refuses something at startup on a user's machine. It is a bad property
during development, because a typo produces a working session that is quietly
wrong, and the gate would report `status=fallback` in a line nobody reads until
they go looking. With the GLSL in its own files a real front end can compile it
before it ever reaches hardware, which `tools/check_shaders.sh` now does.

Two failure modes were built into that script rather than discovered later. It
refuses to run when no validator is present instead of passing, following
`check_tla.sh`, which requires its pinned jar for the same reason. And it fails
when its search matches no shader sources at all -- a moved directory or a
renamed extension would otherwise report success having compiled nothing, which
is the shape of green that means least.

The move improved the tests it touched. They had greped the Rust file that held
both shaders and asserted `count("uniform float source_is_opaque;") == 2`, a
count standing in for "both programs declare it". Against separate files each
program is asserted directly, so a change that gave one shader the uniform twice
while the other lost it now fails, and the message names which file. All three
negative controls from the previous commit still fail correctly through the new
indirection, which is the point of running them again rather than assuming a
pure move cannot break a test.

The shader text was checked byte-for-byte identical across the move before
anything else ran. A refactor that also changes behaviour is two changes wearing
one commit message.

<!-- END IMPORTED BODY -->
