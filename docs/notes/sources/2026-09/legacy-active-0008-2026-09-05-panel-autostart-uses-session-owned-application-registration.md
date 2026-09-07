---
id: legacy-active-0008
date: 2026-09-05
recorded_date: 2026-09-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "shell"]
---
# 2026-09-05: panel autostart uses session-owned application registration

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 265–280. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The operator requested panel startup from the personal Hagia desktop profile.
The installed launcher always supplied `--session-start=terminal`, overriding
that profile, and the typed profile accepted only one startup selector. The
session candidate now admits an ordered, bounded list of registered identities;
normal Hagia startup supplies a fallback terminal selection instead of an
explicit override. Proofs keep explicit CLI selections. Executable registration
and renderer environment remain in Sophia's trusted core config, with no new
WM process authority or Quickshell-specific production policy.

The installer failure at Just recipe line 104 was the packaging prerequisite
that Hagia HEAD equal the locally known `origin/master`. Existing signed commit
`875c8c2` was local only; it is now published, and Hagia was rebuilt before
packaging. The source-signature and published-source requirements remain intact.

<!-- END IMPORTED BODY -->
