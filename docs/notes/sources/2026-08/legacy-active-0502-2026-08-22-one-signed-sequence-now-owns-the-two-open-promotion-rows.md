---
id: legacy-active-0502
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-22: one signed sequence now owns the two open promotion rows

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15384–15418. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The mixed-output gate no longer leaves its only passing artifact in `/tmp`.
  A successful visual and telemetry result is archived with the exact Sophia
  and reference-WM digests, signed source commit, and copies of the committed
  core configuration and desktop profile. A standalone verifier checks the
  archive checksum, commit signature, binary identity records, committed file
  contents, visible-pixel acceptance, and the raw topology proof. Negative
  fixtures reject a substituted WM digest and a profile that is not from the
  named signed commit.
- The Hagia physical gate now binds two repositories rather than naming only
  Sophia implicitly. Its current-checkout launcher requires clean signed heads
  equal to the locally known `origin/master`, builds both exact binaries, and
  carries their four identities through the session log and checksummed archive.
  Hagia's three prepared signed commits through `074e374c537b316b6bdf196ac8f3727004ba6549`
  are published on `origin/master`.
- Real-session broker promotion is part of that same proof. The live broker
  emits a clean terminal lifecycle record only after its transport disconnects
  and supervised process terminates. The Hagia verifier requires one protected
  revision-1 admission, at least one redacted descriptor commit, and one later
  clean stop, and refuses either a broker failure or a missing identity. Its
  negative fixtures exercise every required lifecycle record and an unknown
  Hagia archive commit.
- `tools/run_current_critical_path_tty4.sh` is the operator boundary. It prompts
  for the current rig's two-head, three-head, and two-head cable states; runs the
  mirror, centered mixed, and Hagia/broker gates in that order; and stops on the
  first identity change or failed gate. This is implementation readiness, not
  physical promotion. The three archives still have to pass on the current
  signed candidate before Tier-0 indicator work begins.
- Formatting, diff hygiene, offline metadata, the two negative fixture suites,
  and the full all-feature Rust suite pass. The aggregate atomic-local wrapper
  still stops at its pre-existing source-layout audit: the unreviewed oversized
  and inline-test files it reports predate this tranche and none is touched
  here.

<!-- END IMPORTED BODY -->
