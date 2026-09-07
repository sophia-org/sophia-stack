---
id: legacy-active-0095
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation", "tooling"]
---
# 2026-08-07: Installed proof profiles require profile-specific ledgers

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3139–3166. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed release `0.1.0-ce494942fb32` reclaimed the stale Firefox profiles on
the next launch and removed its current profile during teardown. Physical run
`0042` left `/run/user/1000` at one-percent use with no `firefox-m10.*`
directories. It also retained `protocol_errors=0`, `unexpected=0`, clean layout
and renderer health, normal logout, and complete frontend and resource drain.
This closes the `GetImage` and proof-profile resource regressions.

The installed result still reported `session_verification`, but the archive
was under the ordinary XMonad run ledger and had been judged by the generic
desktop verifier. The `Sophia Firefox Proof` wrapper passed the proof argument
without selecting a Firefox attempt mode, so every physical Firefox run was
misclassified regardless of its contents. Applying the dedicated verifier to
run `0042` exposed the actual workflow failure: six action-launched Firefox
processes instead of exactly two, with incomplete Kitty retention checkpoints.

The installed Firefox entry now selects a Firefox attempt mode before invoking
the common session wrapper. That mode reserves and finalizes a schema-4
`record_kind=firefox` archive under `promotion/firefox-runs`, applies the
dedicated browser verifier, identity check, and normal-lifecycle verifier, and
emits `sophia_installed_firefox` as its result. The manual Firefox recorder
remains available for compatibility, and the aggregate verifier accepts both
legacy archives and the stricter automatic schema. A fake installed-release
regression uses evidence that deliberately fails the generic desktop verifier,
proving that a passing Firefox attempt cannot silently take that route. The
exact two-launch contract is unchanged.

<!-- END IMPORTED BODY -->
