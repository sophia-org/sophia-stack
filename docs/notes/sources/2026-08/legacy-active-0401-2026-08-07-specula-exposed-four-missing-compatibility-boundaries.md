---
id: legacy-active-0401
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-07: Specula exposed four missing compatibility boundaries

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12181–12227. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- A commit-pinned, development-only Specula audit examined complete legacy-WM
  projection, delayed Configure/Focus responses, restart/reseed, and safe-pixel
  admission. It used a clean clone of Sophia commit `ef918108` and Specula
  commit `3946f892`; eleven focused configurations found four implementation
  defects. Specula remains outside Cargo, packaging, and the installed session.
- An unmodified legacy WM cannot attach Sophia transaction identity to its X
  requests. A delivered-channel drain therefore cannot prove that a
  socket-buffered or scheduled reply belongs to the next request. Successful
  collection now requires a final quiet boundary; reaching the hard deadline
  is failure. Any request error poisons that private runtime, causing the
  existing supervisor to replace and reseed it before later Engine work.
- Complete workspace packets now replace cached membership exactly while
  preserving stable synthetic XIDs. Direct `AssignWorkspace` mutates that same
  unique membership before returning the Engine command, and workspace
  activation derives mapping from it. Omitted or moved surfaces are unmapped,
  so their delayed Configure and Focus requests remain private.
- A surface may have presentation intent without any complete pixel extent.
  The first such admission timeout is now an expected bounded state: it keeps
  the owner loop and standing target alive, records one retry, and bypasses
  fixed-extent recovery until safe pixels exist. Persistent silence still
  follows the ordinary retry/withdrawal policy.
- Deterministic Rust regressions preserve all four counterexamples. The revised
  `LegacyWmProjection` model checks 2,106 distinct states to depth 11;
  `LegacyWmResponseBoundary` checks 6,417 to depth 39; and
  `PixelSilentAdmission` checks 11 to depth 5 with its liveness properties.
  The pinned project checker passes all models.
- Restart/reseed remained clean in five exhaustive Specula configurations:
  302,541,189 distinct states to depth 40, 4,159 to depth 20, 595 to depth 18,
  and two 90,181-state searches to depth 28. Two initially stronger generated
  invariants were corrected rather than imposed on Sophia: retained fallback
  pixels may differ from a later exact successor, and only expected
  Configure/Focus replies have a current-request obligation.
- Final candidate-identity and ownership-exclusivity simulations each reached
  the full 30-minute watchdog without a violation. They checked 701,155,271
  states across 44,868,415 traces and 707,143,607 states across 50,238,084
  traces, respectively.
- Specula's optional post-validation agent confirmation was stopped after five
  provider-policy false positives on the first benign local X11 reproducer. It
  is not part of the cited evidence; validated traces, exhaustive checks, and
  checked-in deterministic Rust regressions own these conclusions.
- Source control retains the corrected small models, regressions, this result,
  and a clean-clone installer/runner under `validation/specula` and `tools`.
  Generated source copies, patches, transcripts, raw traces, model-checker
  databases, and large logs remain local audit evidence.

<!-- END IMPORTED BODY -->
