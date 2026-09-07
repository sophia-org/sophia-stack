---
id: 6q35cl9y
date: 2026-09-07
kind: investigation
status: awaiting-physical-acceptance
tags: [investigation, session, recovery]
---
# Normal login arms recovery without a keyboard rehearsal

The user approved removing the Ctrl+Alt+Backspace arming handshake from ordinary
installed logins. The independent recovery guard remains enabled. This is a
[t013 startup and recovery change](../plans/queue-04-2-establish-the-live-session.md#t013),
not a change to application shortcuts or WM policy.

## Ownership and behavior

The installed launcher already distinguishes ordinary desktop sessions from
explicit proofs and promotion runs. Ordinary logins now request automatic
arming; a user can still select manual arming through
`SOPHIA_INPUT_GUARD_ARMING=manual`. Proof and promotion launches force manual
arming even when their environment requests automatic. Explicit TrueColor and
watchdog environment settings also select the proof path. Development launchers
and the underlying guard command retain their manual default.

The guard itself opens libinput and requires a keyboard before publishing its
armed marker. The automatic state uses the existing emergency-chord reducer:
the first complete chord triggers recovery. Manual mode still requires a full
press and release to arm. The wrapper waits for readiness in both modes and
rechecks guard liveness and pending recovery immediately before graphics
takeover. Guard failure during the session remains a supervised failure.

The startup timeout bounds readiness, not desktop lifetime. Automatic arming
proves that keyboard input opened; only an operator's chord proves the physical
recovery path. Removing the repeated rehearsal does not establish that proof.
The [operations contract](../../operations.md#emergency-recovery-and-fallback)
describes the user-visible behavior.

## Validation and remaining acceptance

Candidate base: `34128d80`, alongside the
[Kitty admission repair](ce2b55uy-blank-thunar-menus-and-frozen-brave-need-separate-pixel-and-delivery-evidence.md#2026-09-07-kitty-starts-but-never-enters-the-composed-scene).
The new public guard-entry tests refuse invalid arming modes and prove that an
input-open failure publishes neither readiness nor recovery in either mode.
Existing reducer tests cover first-chord triggering, full-release manual arming,
and rejection of partial chords and repeats. Installed-launcher fixtures cover
ordinary automatic arming, explicit manual arming, and proof/promotion selection.

`cargo xtask check` passes: workspace tests, Clippy, contract and archive checks,
and host buffer-age pixel equivalence. The installed-launcher check passes
separately after fixture isolation and cleanup were tightened. Formatting,
metadata, and diff checks pass. Private logs and exact source copies are
checksum-verified at
`~/.local/state/sophia/development-evidence/t013-guard-cd6e570277f5`; source identity
`cd6e570277f546eb424bf69413e847ea113079b75f5b749423df903e48399049`.
The existing Kitty before/after pixel evidence remains in its linked investigation.

No install or live-session replacement was performed.
After installing this candidate, accept one ordinary login with
no arming prompt and a normal logout, then separately confirm that one
Ctrl+Alt+Backspace chord returns control to greetd. These physical observations
remain part of t013; deterministic checks do not close that gate.
