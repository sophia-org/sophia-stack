---
id: nsu4a0n2
date: 2026-09-11
kind: investigation
status: awaiting-physical-acceptance
tags: [policy, input, session]
---
# Optional pointer focus follows presented targets through committed policy

## Scope and acceptance

User-selected on 2026-09-11: follow niri's disabled-by-default convention and
provide a fully tested Hagia `policy { focus-follows-mouse #true }` setting.
Omission and explicit false retain click and keyboard activation. When enabled,
physical pointer motion may focus an eligible presented window or activate a
monitor with no window beneath the pointer. No input or rendering authority
moves into Hagia.

The implementation must validate configuration and independent wire encoding,
retain default behavior, resolve targets from presented root geometry, suppress
hover focus under captures and grabs, preserve input ordering, and change public
focus only after Engine commit. Configuration replacement and checkpoint recovery
must use the current setting; rejected or timed-out candidates retain prior focus.
Deterministic tests cover these boundaries and launch/picker ordering. Installed
physical acceptance remains a separate gate.

## Evidence

Sophia `58ed7f7a` with Hagia `24f3203` is installed and running (owner PID 4545).
The user confirmed app launches and picker-launched Ghostty on DP-1. Moving the
pointer to empty DP-2 and pressing Super+Enter retained DP-1, consistent with
current click/keyboard activation. This observation does not reject the prior
launch-focus fix. DP-1 is the large left monitor; DP-2 is the smaller right.

## Design and validation

Implemented with Hagia `d5eb25cd3ba606192a470849e27236120747198a` and the
paired Sophia change based on `58ed7f7a`. The policy setting belongs to Hagia;
Sophia reduces presented pointer targets into bounded opaque policy causes.
Negotiated capability bit 13 gates cause kind 4 at the actual socket writer,
and disabled profiles do not request the capability. Raw input, coordinates,
device identity and application metadata stay outside Hagia's policy interface.

A bounded ordered input queue holds later shortcuts behind a hover settlement;
only adjacent hover observations coalesce. Replacement clears queued work.
Eligibility comes from the pointer output's retired projection, with no primary
fallback for an unpresented output. Captures, grabs and popups retain their
existing input ownership. Rejected observations may be retried by fresh motion.
Hagia checkpoint version 13 carries the setting; older checkpoints migrate off,
and a replacement profile overrides the restored setting.

Retained evidence is in `.artifacts/t078-pointer-focus/`:

- `check.log`: complete Sophia `cargo xtask check`, exit 0; 3,053 Rust tests,
  29 intentional ignores, archive checks and host pixel proofs passed.
- Two additional queue/input-order tests passed in the final eight-test focused
  pointer suite after that full run. The strengthened launcher capture test also
  passed with a nonzero presentation epoch, so capture itself is tested.
- `hagia-verify-final.log`: final `nimble verify`, exit 0, including 207 Hagia
  cases, formatting/data-layout checks, Alloy/Z3/TLA+ checks, real pregraphics
  admission, and both paired pointer-focus tests. The gate verifies test names
  before execution so a filter matching zero tests cannot pass silently.
- `hagia-paired-final.log`: both compiled-Hagia socket cases passed. They cover
  enabled/disabled negotiation, real empty-output and window-target requests,
  Engine stage/commit, timeout discard and retry, and an old server refusing the
  enabled setting before configuration with an explicit diagnostic.
- `clippy-final.log`: final affected-crate all-feature/all-target check, exit 0.

The protocol generator had a pre-existing shell revision-3 assertion while the
schema already described revision 4. Updating that check and its expected nine
revision-4 messages was necessary to regenerate the new WM capability. Schema,
Rust and C constants are synchronized.

Claude implemented and tested Hagia through the user-authorized Herdr helper and
reviewed Sophia. Review confirmed launcher interception precedes hover emission,
configuration is mandatory in this live policy owner, and complete projections
remain bounded. No unresolved blocking review finding remains.

## Remaining physical gate

At implementation handoff the running desktop used Sophia `58ed7f7a` and Hagia
`24f3203`; installation and personal-profile activation were still pending.
The physical gate requires default-off click/keyboard behavior and enabled
hover between windows and onto empty DP-2; immediate terminal/browser/picker
shortcuts must use the committed active output. Confirm captures and window
movement remain usable, then reload with the setting off and confirm it stops
following the pointer. Keep t078 open until these installed checks are accepted.
This is separate from the remaining reload-geometry and expanded-window checks.

## 2026-09-11 installed acceptance in progress

The user installed Sophia `3085149116ef418d0aefe563a1b1d9b69d9fe25f`
with Hagia `36bbb9453f625f692369c7fa3c231c5471f0f252` and entered session
`00000001789138761895-b8f5b4e5-9487-4ec5-9919-6783dae91c45`.
Live owner PID 16982, Hagia PID 16987, and Narthex PID 17026 match the
packaged binaries. Hagia SHA256 is
`92c24358b00158b46bb1df5cc32d1751ae5763c709a0a89b097b8be895ace52e`;
Narthex SHA256 is
`1a22f1212667de6849d82a63acf5967f1a61afc485d60bd93d7d8034302074f1`.
Baseline evidence is retained in `.artifacts/installed-acceptance-30851491/`.

With the setting omitted, the user clicked a left-monitor terminal, moved the
pointer to the empty right monitor without clicking, and pressed Super+Enter.
Kitty remained on the left, accepting this default-off case. The second
click/window test could not proceed because the right workspace was empty.
The checkpoint confirms that both outputs already have three independent views:
left views 1–3, right views 4–6, with local workspace slots 1–3 on each.
The personal profile lacked monitor-focus bindings; background clicks without
a surface currently do not activate an output.

The next test profile enables `focus-follows-mouse #true` and binds
Super+Ctrl+Alt+Left/Right to `focus-output-prev/next`. Both Sophia's desktop
envelope check and Hagia's extracted-policy check pass. The existing
Super+Alt+Left/Right column-movement bindings are preserved. Enabled behavior,
captures, and disabling through reload still require user observation.

The user reloaded the enabled profile and confirmed that moving onto empty
DP-2 followed by Super+Enter opens Kitty on DP-2, and that focus follows the
mouse. The committed checkpoint has `focusFollowsMouse=true` and window 7
assigned to logical output 2. Identity records 100880–100881 show WM epoch 2,
profile generation 2 activation, and desktop/launch generation 2 publication.
The enabled personal-profile SHA256 is
`6ab0a40dece42d69f00349eee0242fa90a0ff095d68d221dbc2f28c428355c4c`.
The `hover-enabled-*` snapshots and `hover-enable-reload-events.log` retain
this evidence beside the baseline. Picker capture, workspace independence,
reload geometry, drag behavior, and disabling remain to be checked.

The user subsequently accepted picker capture across outputs: the picker opened
on the right, retained typing after the pointer moved left, and launched btop
on the right. Super+2 then Super+1 on the right changed only that monitor's
workspace and restored Kitty. The user also confirmed that reload preserved
window geometry without glitches or duplicate startup windows. This supplies
the separate t001 reload acceptance; browser placement/input, floating-window
drag/resize, and disabling through reload remain for t078.

## Related repair

The preceding [launch and pointer repair](v4geoq2j-policy-reload-compares-independent-configuration-generations.md)
records t001's independently accepted reload result. Expanded-window task t004
still needs its own physical acceptance.
