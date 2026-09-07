---
id: legacy-active-0134
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-05: installed fallback attempts are automatic and fail closed

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4324–4348. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The Kitty baseline previously rotated live logs but had no immutable attempt
boundary. A successful-looking later run could overwrite the evidence for a
failed fallback, and the operator had no repository-independent command that
bound the reduced session to its installed release identity.

Normal and fallback logins now share one profile-parameterized attempt ledger.
Each installed Kitty launch reserves a numbered directory before graphics
takeover and finalizes it after display-manager handoff. A crash remains
pending; a nonzero or unverifiable run remains failed. The archive contains
checksummed session, guard, recovery, lifecycle, launch-identity, runtime-
identity, and release records, while normal and fallback attempts remain in
separate ledgers and cannot be relabeled across profiles.

The fallback verifier admits only the bounded one-Kitty, WM-disabled profile.
It requires two-output startup and visible retirement within eight seconds,
positive routed physical keys, clean protocol, presentation, application, and
lifecycle shutdown, an armed but untriggered guard, and exact Kitty-profile KD
and termios restoration. Mutation fixtures reject missing Kitty exit,
one-output or slow startup, missing retirement, absent physical input, external
WM policy, emergency recovery, a wrong recovery profile, a failed latest
attempt, and archive modification. Packaging, installation, status, and the
operator runbook expose the same contract without a source checkout.

<!-- END IMPORTED BODY -->
