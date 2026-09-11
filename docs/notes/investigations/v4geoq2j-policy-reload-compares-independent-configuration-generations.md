---
id: v4geoq2j
date: 2026-09-10
kind: investigation
status: awaiting-physical-acceptance
tags: [investigation, session, config]
---
# Policy reload compares independent configuration generations

## Physical trigger

During t001 acceptance, the operator pressed Ctrl+Alt+R after changing only
`outer-gap 8` to `outer-gap 9` in the active desktop profile. The desktop looked
essentially unchanged. The journal records policy activation at profile
generation 6, configuration rejection, and replacement rollback to profile
generation 1. Two attempts produced WM epochs 3–6. Output topology and layout
were preserved. Sophia PID 22902, Narthex PID 22948, Quickshell PID 22949 and
existing Kitty processes survived. Hagia ended at PID 15338.

The session is `00000001789079343163-ae587303-077c-4f55-b0b6-209bcde4edcc`.
Installed Sophia source is `171345049bf620a40b24c48d738329b1f63decaf`, binary
SHA-256 `8d079f5ff05732bbe3f2b43d2791c8655b780f00065c69f2b385273230b8295b`.
Hagia source is `3459a85d5dd7a1943efcf526fa5d4ec297d246cd`, binary SHA-256
`255f49f4ee5baf54ff8aa6df7c9153b55072fa425a50c0cd65c31fe128ae23ea`.
The component journal digest is not the executable hash; the hashes above were
read from `/proc/PID/exe`.

Evidence is retained under `.artifacts/t076-desktop-acceptance/`: immutable
original config backups, `before.json`, `core-reload-passed.json`,
`gap-applied.json`, and `events-so-far.log`. Candidate
`desktop-absolute-gap.kdl` differs from the accepted `desktop-absolute-b.kdl`
only by that gap. The 8-pixel profile was restored on disk after rejection.
No owner replacement or new binary installation was performed.

## Cause and repair

`stage_policy_configuration` passed the activated desktop-profile generation
into `resolve_public_shortcuts` as the expected action-catalog generation.
Hagia's `installConfiguration` starts the catalog at 1 on each connection.
A later profile therefore fails the resolver's equality check even though its
profile activation succeeded. The [WM contract](../../sophia-wm-api.md)
explicitly assigns these two generations separate namespaces.

The retained journal redacts the detailed rejection reason. The source finding
is independently reproduced through production staging: the external reload
fixture now sends catalog generation 1, matching Hagia, while the profile is
later than 1. Before the repair, valid staging returned `RejectedInvalid`.
The previous fixture copied the profile generation into the catalog and masked
the defect.

The repair uses the received policy-configuration generation to resolve its
catalog. Profile activation still validates the exact profile key in its own
barrier, and staging still rejects a stale connection epoch. No generation
field is repurposed or wire format changed. The fixture also verifies stale
epoch rejection before accepting the current connection, and retains the
held-key publication assertions.

## Validation and limits

The focused regression fails before the production change and passes after it.
All 21 reload/registry tests pass with `native-session` enabled; logs are
`generation-regression-before.log` and `generation-regression-after.log`.
`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` passed; the complete log
is `check.log`. It includes the retained archive checks, buffer-age equivalence
and GLX/EGL first-frame and pixmap-export pixels. Physical acceptance requires an owner containing
the repair and another policy replacement; t001 remains in
[todo.md](../../../todo.md).

The same session separately exposed picker input failures. The
[launcher input investigation](yifbnqjz-launcher-capture-loses-visible-cursor-updates-during-modal-input.md)
records the operator observations and reproduced defects; no common cause with
the policy generation mismatch is established.

## Connections

The [development-session plan](../plans/queue-02-cp-14-3-development-session-readiness-and-milestone-14-c.md#t001)
owns the physical acceptance gate. The
[application-command plan](../plans/1agxbuuf-application-commands-in-the-desktop-profile.md)
introduced atomic publication across command changes and policy replacement;
command-only and core reloads passed their separate launch checks in this session.

## Repaired-session acceptance in progress

The repaired release `d5acca885be5ce98bb9d2b44cb127b6a85f6454a` is installed
in session `00000001789093895563-1c5b322b-e1c9-4c19-aab1-33644a07f1c5`.
Owner PID 1575 has executable SHA-256
`90e11f83bd0c48050b04a64d6baa34e4302838f67bf29b7af105f2d23d5150a3`.
The baseline has Hagia 1580, Narthex 1624, Quickshell 1625, Kitty 1729,
Ghostty 10904 and Kitty 20639. Fresh configuration backups, a process snapshot
and a guarded installer are retained in
`.artifacts/t076-desktop-acceptance/repaired-session-baseline/`.

The same one-pixel `outer-gap 8` to `outer-gap 9` reproducer is staged in the
active desktop profile. Original SHA-256 is
`b1b7b8f1616af018d1bc16ffbff754f56d5d494a782e5ae9f004594931ecf554`;
test profile SHA-256 is
`bd3d9e989e98ccac25aee546008470eb2068e056694c0760c0291a1e6179d20b`.
The operator pressed Ctrl+Alt+R. The replacement accepted profile generation 2
at WM epoch 2 (journal sequence 48799), then published desktop and launch
generation 2 (48800). The first replacement layout committed at 48824, about
30 ms after the restart record at 48792. No rejection or rollback was observed.
Hagia changed from PID 1580 to 5601; owner 1575, Narthex 1624, Quickshell 1625
and the baseline application processes survived. This is physical evidence
that the independent-generation repair allows replacement acceptance.

The operator reported incorrect Ghostty size, position and overlap immediately
after reload; navigating windows corrected it. This is not clean t001 visual
acceptance. The existing logs omit the complete geometry and camera proposal,
and the checkpoint retained after navigation cannot establish the immediately
broken state. Checkpoint reconciliation, geometry settlement and presentation
remain possible boundaries; no new root cause or repair is claimed.

Evidence is retained as `after-gap9-reload.json`,
`gap9-reload-logs-1789094827/`, `gap9-reload-events.log`,
`hagia_policy_checkpoint-after-gap9` and `observations.txt` under the new
baseline directory. The original desktop profile is restored on disk with its
exact original hash. The running profile remains generation 2 until the
operator performs the requested restoration reload. The operator was asked to
leave any repeated distortion in place for inspection before navigating.

The restoration reload also accepted, at WM epoch 3 and desktop/launch
generation 3 with the original profile digest. Hagia became PID 23618; owner,
shell and application identities remained unchanged. The operator reported
that Ghostty was skinny and left it in place. The captured checkpoint retains
`outerGap=8`, Ghostty's normal 50-percent column, and its output-1 assignment.
Its client XID 6291460 belongs to PID 10904; read-only X geometry reports
`x=3821, y=41, width=1258, height=1390`. The checkpoint surface bounds include
the surrounding border at `x=3820, y=40, width=1260, height=1392`.

This does not show a narrow logical column. Ghostty lies beyond output 1's
right edge in the captured scrolling layout. The operator clarified that it
was a narrow slice about three-quarters of the way to the right; which monitor
showed it remains unconfirmed. That distinction is still needed before
attributing the symptom to clipping, stale presentation or policy. Evidence is retained as
`skinny-after-gap8-restore.json`, `hagia-checkpoint-skinny-gap8`,
`x-geometry-skinny-gap8.json` and `skinny-gap8-logs-1789094996/`.
Configuration restoration is complete; the visual defect remains unresolved.

## CPU frame output-routing defect

Source inspection found that `run_cpu_production_cycle` built each native head's
display list from the complete presentation order. Its CPU callback bypassed
the output-ownership filter already used by `display_list_for_output` for
ordinary mixed and retained frames. An output-1 scrolling column at Ghostty's
captured coordinates therefore entered output 2's CPU display list and could
be clipped to the part intersecting that monitor. This is a reproducible code
defect; the operator's affected monitor is not yet confirmed, so it is not
claimed as the established cause of every reported size/overlap symptom.

The repair prepares an order for each CPU output using the same shared
`surface_order_for_output` filter as ordinary presentation. Managed surfaces
stay on their policy-assigned output; frontend-positioned panels retain their
geometry-based routing. No checkpoint, application size or animation behavior
is changed.

The external regression
`cpu_frame_orders_keep_offscreen_columns_on_their_assigned_output` uses the
captured geometry and the order factory called by the CPU callback. Before
the routing repair it incorrectly contains the scrolling column on output 2;
afterward only the frontend-positioned panel remains there. This is a unit
regression kept outside production `src`; it exercises the factory used by
the CPU callback, not a real KMS frame. All 92 backend
library tests pass with `libdrm-events,gbm-probe`. Evidence is retained as
`cpu-output-routing-before.log` and `cpu-output-routing-after.log` in the
repaired-session baseline directory. Claude's independent read-only review
found no blocker in the routing change. It agreed that monitor confirmation
and a physical rerun remain necessary; neither the source review nor the unit
test establishes the complete visible symptom's cause.

The review also found two adjacent differences that this repair does not
change: CPU frames do not sample the translation timeline, and translation
target changes do not contribute to retained-frame invalidation. The first
does not explain an epoch-reset frame, whose springs are stationary; the
second's contribution to this incident is unproven. These remain boundaries to
inspect if subsequent scrolling or reload acceptance still fails.

The first full-gate attempt inherited
`SOPHIA_RUN_REAL_ATOMIC_SCANOUT_SMOKE=1`; its optional real modeset submission
failed. The next attempt exposed inherited live-session diagnostic settings:
the lifecycle child redirected its records and failed a stdout assertion.
That test passes with the inherited `SOPHIA_*`/`HAGIA_*` environment removed.
The isolated full-gate log is `cpu-output-routing-check-isolated.log`.
That gate passed: 3037 Rust tests across 260 result groups, 29 intentional
ignores, archive checks, host buffer-age equivalence, and GLX/EGL first-frame
and pixmap-export checks. The repair is based on `d5acca885be5`; its source
diff is retained as `routing-source.patch` with identity recorded in
`routing-candidate.json` in the same artifact directory.
Physical acceptance of the rendering repair remains pending. Captured
broken-state logs precede these test runs.
