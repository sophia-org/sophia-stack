---
id: bwffe5lv
date: 2026-09-11
kind: investigation
status: investigating
tags: [investigation, policy, session]
---
# Triad floating regressions and child launch origins

## Approved scope

User approved implementation on 2026-09-11 under t041. Triad baseline:
`fb8fb27ec294e0fe2361375de0b2fa8c08be0ca9`; initial Sophia `da9dda47`,
Hagia `d5eb25c`. Triad has unrelated removed lockfiles; its source is unchanged.

Port authority-neutral regression cases for offset-output floating geometry,
late/nested parents, same-candidate parent centering, manual geometry survival,
stacking, and family focus restoration. Dialogs remain visible on unrelated
focus; they hide with minimized, unprojected or wholly offscreen parents.
Automatic versus manual intent survives checkpoint and scratchpad restoration.

New child-app origin attribution belongs to Sophia's authenticated X admission.
Hagia advertises opaque immutable placement bookmarks in committed projections;
Sophia captures one at child connection and echoes it at first top-level admission.
No PID, XID, title, class, namespace or executable crosses to Hagia. No swallowing
or existing-instance forwarding. Missing, ambiguous or obsolete origin evidence
falls back to normal placement without failing a policy cycle. Background origin
placement does not activate its monitor/workspace or steal focus.

## Validation and evidence

Sophia implementation adds optional capability bit 14 and paired 24-byte
extension records `0xff05`/`0xff06`, committed-context publication, authenticated
socket-peer process ancestry, first-top-level origin retention, and process-bound
registered-launch classification. No application metadata enters the wire.

The full Sophia contributor gate passed with inherited `SOPHIA_*`/`HAGIA_*`
variables removed: `.artifacts/t041-launch-origin/sophia-check-final.log`.
This includes workspace tests, Clippy, generated protocol agreement, archived
proof checks, and local render-node buffer-age/GLX/EGL checks. A later focused
regression against production projection settlement also passed:
`.artifacts/t041-launch-origin/settlement-test.log`. Real X socket admission
passes with a child process paused between connection and mapping. The paired
Hagia fixture adds four combinations of switching before/after connection and
visible/hidden originating workspace, each with rejection and retry.

The first full-gate attempt inherited the installed session's real atomic-scanout
smoke flag and failed its card submission; the clean offline run supersedes that
attempt. A descriptor-count regression initially raced the new socket fixture;
serializing those two tests fixes process-wide measurement interference. Golden
record coverage now includes both new extension records.

The real-X/Hagia fixture passed all four origin scenarios after correcting its
advertised action set: the fixture offers pure policy actions, so it must not
advertise session actions without matching session-operation records. Hagia's
strict snapshot validation correctly rejected that earlier fixture.

Hagia's reviewed changes include automatic/manual floating intent (checkpoint
14), final parent geometry before dialog derivation and visibility, family
stacking/focus fallback, and immutable per-epoch launch tokens with protected
current destinations during eviction. Tests cover 1100 historical destinations,
dormant outputs, malformed origins, class precedence, legacy checkpoint geometry,
manual placement, nested dialogs and inactive parent tabs. The fullscreen panel
fixture supplies a real positive work rectangle; an offset alone would have
silently used its full-output fallback and failed to exercise the reservation.
The full reviewed Hagia gate passed: `.artifacts/t041-launch-origin/hagia-verify.log`.
It includes 161 policy-model tests, the paired X/Hagia scenarios, shared wire and
restart corpora, formatting/layout checks, and the existing Alloy/Z3/TLA+ gates.
The release binary is `.artifacts/t041-launch-origin/hagia`; its build completed
successfully. Sophia's additional all-target session Clippy pass is recorded in
`.artifacts/t041-launch-origin/session-clippy.log`.

Paired Hagia candidate: `36bbb9453f625f692369c7fa3c231c5471f0f252` (signed). The Sophia candidate is the signed
commit containing this note; the release package manifest records both exact
source commits and binary hashes. Physical acceptance and t078 remain separate pending
gates. No live-session installation or reload has been performed for this scope.

## Installed acceptance still required

Use the signed packaged pair in a new installed session. From Kitty on the large
left monitor, launch a new child Kitty with a delay, switch to the right monitor
before it connects, and verify the child remains on the originating left view
without moving active focus. Repeat after switching the left monitor to another
workspace; returning to the original workspace must reveal both separate windows.
An already-running application that forwards a request is intentionally outside
this origin guarantee.

Exercise a real parented dialog while scrolling, maximizing/fullscreening its
parent, moving/resizing the dialog manually, and closing sibling/nested dialogs.
Verify geometry, stacking and focus on both outputs. Preserve the separate t078
pointer-focus/reload acceptance record; offline tests do not close either gate.

## Connections

The [t041 plan](../plans/queue-14-native-wm-and-shell-product.md#t041) owns this
promoted workflow. Triad test provenance is retained in Hagia's provenance record.
