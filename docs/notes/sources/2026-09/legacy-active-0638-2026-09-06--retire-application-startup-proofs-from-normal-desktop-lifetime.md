---
id: legacy-active-0638
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-09-06 — Retire application startup proofs from normal desktop lifetime

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20569–20621. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next login still used installed `f323323d`, now with the panel-only desktop
profile. Its session began at 23:02:06 UTC. Super+Enter committed launch
transaction 12 at 23:02:13, but the process never spawned. The owner ended the
session at the eight-second startup deadline with `stage=not_focused`; native
cleanup drained with no abandoned scanouts or cleanup errors. This was not a
new Kitty protocol failure.

The launch queue waited for application startup readiness, while readiness
required a focused application. The installed launcher supplied that proof
deadline to ordinary login. The source-only launcher edit had not been
installed, and removing the deadline alone would have left normal completion
and native rendering activation tied to the same proof record.

The user confirmed that this check belonged to early development. Normal
desktop sessions now have no overall application-startup deadline and no
replacement desktop timer. Explicit proof launchers retain the existing
`--startup-ready-timeout-ms` option and exact-surface evidence. Authority
activation, bounded queues, WM replies, application admission, routed input,
page flips, and shutdown keep their own checks and deadlines.

`SessionLaunchQueue::begin_next` now accepts only admission-pipeline readiness;
it no longer reads an application-proof flag. Normal startup launches every
configured app without waiting for the first app's frame, contains spawn
failures, and initializes the empty runtime even headlessly. Removed the
synthetic blank-session proof success. Normal completion uses schema 17 and
`startup_ready_msec=not_requested`, with a separate startup proof status record;
explicit proof completion retains schema 16 and its required numeric timing.
The normal installed-session verifier accepts empty/panel-only desktop
evidence without requiring an automatically opened terminal.

Native scanout and atomic cursor admission now depend on completed output
presentation and cleared output quarantine. Proof sessions additionally keep
their exact-surface composition barrier. Native owner replacement resets this
activation state. Normal CPU visual accounting starts at owner-loop admission,
so removing the proof cannot silently turn off update accounting or drain
obligations; its timing origin is session admission, not application proof.

Headless CLI regression cases cover empty startup, a background-only session
lasting 8.5 seconds, unsuccessful process exit, failed spawn followed by another
startup app, clean completion, and an explicit proof that still times out.
No DRM or physical input is acquired by these tests. Hagia's isolated-config
conformance gate and the normal-desktop verifier fixture pass. Physical
acceptance awaits a matching rebuilt binary and launcher; the current installed
release has not been modified or restarted.

Final verification passes: `cargo xtask check` ran 2,502 test executions,
Clippy, source-layout checks, the normal-desktop verifier fixture, retained
archives (5/5 Hagia, 9/9 mirror, 6/6 direct scanout), and host buffer-age pixel
equivalence. The separate Hagia conformance gate passes with isolated test
configuration. Release packaging follows these checks; physical acceptance is
still an operator step.
<!-- END IMPORTED BODY -->
