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

The running desktop still uses Sophia `58ed7f7a` and Hagia `24f3203`. No live
reload or personal profile change was performed for this addition. Install the
paired candidate, then confirm default-off click/keyboard behavior and enabled
hover between windows and onto empty DP-2; immediate terminal/browser/picker
shortcuts must use the committed active output. Confirm captures and window
movement remain usable, then reload with the setting off and confirm it stops
following the pointer. Keep t078 open until these installed checks are accepted.
This is separate from the remaining reload-geometry and expanded-window checks.

## Connections

The preceding [launch and pointer repair](v4geoq2j-policy-reload-compares-independent-configuration-generations.md)
remains awaiting its remaining physical checks. This user-selected addition does
not close desktop reload task t001 or expanded-window task t004.
