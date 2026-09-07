---
id: legacy-active-0027
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-27: an unbounded guide wait turns a stale expectation into a hang

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 924–950. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Re-running `tools/hagia-proof` unchanged would have hung rather than failed.
  Commit `edef9d3a` removed `panel 28` from `COMPILED_DESKTOP_PROFILE`, and that
  gate's session runs with `--no-config`, so it loads the compiled profile. With
  no panel, `desktop_profile_shell_panel_thickness` returns `None`, no
  reservation is ever registered, and the switcher step's `reservation_presented`
  and `reservation_reduced` waits could not be satisfied by any session. The
  guide looped on them forever. Archive `0007` predates the removal and remains
  valid evidence.
- Every wait in `hagia_physical_guide.sh` is now bounded, using the
  `wait_for_shell_line_bounded` idiom the file already had for one browser step.
  Operator steps get a long bound because a person is reading a screen; session
  steps get a shorter one because nobody is. The aborted run now names the
  expectation the profile could not satisfy.
- The same wrapper also exported `SOPHIA_HAGIA_PROFILE_MODE` and the digest of
  Hagia's `default.kdl` while the session ran the compiled profile, so
  `sophia_live_desktop_profile` described a profile that never loaded. Those
  exports are gone; the profile is still checked by both `config check` calls,
  which is what it was for. The native gate binds a profile it actually passes to
  the session, and its verifier requires the loaded `root_sha256` to equal the
  digest in the run's bound identity.
- The switcher workflow itself is deliberately not re-aligned to the panel-less
  profile. It is off the critical path, and archives `0006` and `0007` remain its
  retained evidence. What was fixed is that a stale expectation now fails
  legibly.

<!-- END IMPORTED BODY -->
