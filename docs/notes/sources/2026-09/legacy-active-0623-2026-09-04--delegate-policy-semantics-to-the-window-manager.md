---
id: legacy-active-0623
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "validation"]
---
# 2026-09-04 — Delegate policy semantics to the window manager

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20026–20073. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Hagia's current default added `scratchpad-size` and `column-width-presets`;
its optional `floating-size`, `view-name`, `view-layout`, and `dwindle` layout
also exposed Sophia's duplicated policy grammar. The operator chose delegation
to the WM over extending Sophia's allowlist. Sophia now admits an ordered,
bounded Policy payload, retaining the envelope, reserved Engine controls,
authority partitioning, and activation identity. It does not interpret WM
setting names, values, or duplicate identities. Full profiles and staged
fragments preserve repeated record names; Hagia resolves workspace slots and
rejects actual duplicates. Other authorities keep their existing validation.
No WM or shell wire change is required.

Inspection found that Hagia constructed its policy model after acknowledging
activation. Its grammar loader checked the new settings, but model-only checks
such as the maximum gap could still fail after that acknowledgement. Hagia now
constructs the candidate before the handshake, retains the prepared adapter for
normal operation, and closes the connection on failure. Its offline `config
check` invokes the same construction. A socket regression queues Prepare and
Activate for an invalid gap and requires no completion frame. Cross-repository
startup tests cover successful admission of all five settings and rejection of
invalid geometry, unknown vocabulary, and duplicate workspace identities before
Sophia opens its graphical gate.

Sophia's desktop-profile check reports `policy_validation=delegated`. The Hagia
TTY adapter checks the selected Sophia/Hagia executables and explicit profile
before display-manager takeover, building first when requested so validation
uses the candidate that will run. The paired checks have a ten-second bound per
owner. A disposable-PTY regression proves WM rejection precedes any TTY-mode
query or privileged handoff. Packaging already runs both owners' config checks.
Actual runtime still gives Hagia only its staged Policy fragment.

Verification: Sophia's isolated `cargo xtask check` passed 2,353 test executions,
archive checks (5/5, 9/9, 6/6), and render-node buffer-age equivalence. Hagia's
`nimble verify` passed 140 named tests, the cross-repository conformance suite,
Alloy/Z3 foundation checks, and the lifecycle/trace models. One old Sophia test
still expected Engine rejection of a WM view count; it was updated to assert
delegation while retaining reserved-control rejection. Both optimized builds
succeeded. Paired checks accept the unmodified canonical default and personal
profile with matching digests. No physical session was launched or installed;
CP-14.3 stage 2 continues with the new candidate and stage 1 evidence retained.

Optimized Sophia/Hagia executables, unchanged Narthex, both profile inputs,
source identities/patches, verification logs, and checksums are retained in
`.artifacts/diagnostics/policy-parser-delegation-20260905T012858Z/`. The new
startup rejection test lives under `tests/support`; the optimized Hagia build
also passes both local pregraphics admission tests after that relocation.

<!-- END IMPORTED BODY -->
