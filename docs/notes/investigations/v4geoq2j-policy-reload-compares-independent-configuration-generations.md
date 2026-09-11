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
