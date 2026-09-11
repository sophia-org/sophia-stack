---
id: 3asecq4a
date: 2026-09-11
kind: investigation
status: awaiting-physical-acceptance
tags: [policy, validation]
---
# Maximized windows obscure keyboard navigation targets

## Report and candidate

During installed acceptance on 2026-09-11, the user reported that Super+navigation
stops working in Super+F mode. The personal profile binds Super+F to
`toggle-maximized`; Super+Shift+F is the separate fullscreen toggle. Super+Left
and Right select columns, while Up and Down select windows within a column.

The running session is
`00000001789143018118-8259844a-396f-469e-928d-0bec2bd54d00`, with installed
Sophia `0c069d2f3a232224dfdde3c20ba82e1cd78ee0fe` and Hagia
`36bbb9453f625f692369c7fa3c231c5471f0f252`. The process executable hashes are
`7ebd31ed8beacb73fe9e1de126d72665069f717071e0d36e4efb04f4386469b3`
and `92c24358b00158b46bb1df5cc32d1751ae5763c709a0a89b097b8be895ace52e`,
respectively. The profile hash remains
`6ab0a40dece42d69f00349eee0242fa90a0ff095d68d221dbc2f28c428355c4c`,
with pointer focus enabled. Hagia `9349e57` is staged on disk but has not been
reloaded; the old process still holds the replaced executable.

The checkpoint inspected after the report has four tiled windows on the large
left output, none maximized or fullscreen, and an empty right output. It is
not a capture of the reported failure. A held reproduction from the first
column was requested to distinguish a hidden focus target from an output-edge
handoff. No physical cause is inferred from the post-report checkpoint alone.

## Source finding

Hagia's column navigation includes maximized tiled windows and can select the
next ordinary column. Projection separately expands every maximized window to
the work area and places it above every ordinary window, regardless of which
window now has focus. A normal navigation target can therefore receive focus
underneath the maximized window. This is a policy projection defect; it does
not require changing Sophia's input routing or window metadata boundary.

The original stacking repair remains necessary: enlarging an earlier tile
without raising it lets a later neighbor cover it. The new repair must make
the selected window visible while retaining maximized geometry and state,
preserving parent/dialog families, and retaining fullscreen ordering. Ordinary
tile order must still return when maximization is toggled off.

## Validation scope

The initial offline regression on Hagia `9349e57` reproduced six ordinary
target ordering failures across first/middle/last maximized columns, plus a
focused dialog family below an unrelated maximized window. The artifact
`.artifacts/t004-maximized-navigation/regression-before-fix.txt` retains the
failure output. Its fullscreen exception was a test fixture missing the
fullscreen capability, not a production defect; that fixture was corrected
before the final gate.

The repair retains the expansion layers, then finds the root of the focused
window's visible parent chain with the existing family-depth bound. When a
maximized placement is present, that family moves above background maximized
families and below unrelated fullscreen roots. The family traversal still
places dialogs above their parents. The operation changes only the projected
order; canonical state, geometry, and ordinary ordering without maximization
remain unchanged.

The signed, pushed Hagia candidate is
`43cfcae0ac7481e7032962ff45be0cb45fe0d7ef`. Full `nimble verify` passed
with inherited `SOPHIA_*` and `HAGIA_*` variables removed and only the paired
Sophia checkout path set. The retained `hagia-verify.log` in the artifact
directory contains 249 passing Hagia cases, the paired Sophia tests, protocol
corpora, formatting/layout checks, eight Alloy assertions, Z3, and four TLA+
checks. The existing expanded-window session test still passes.

Six new cases cover first/middle/last maximized columns, explicit directional
focus and reversal, an ordinary parent's focused dialog, both creation orders
of two maximized parents, fullscreen families alongside a maximized window,
and normal order with maximization cleared or absent. The fixture capability
correction and stricter named-neighbor assertions were reviewed before the
final gate. No new Sophia code or wire behavior is involved.

The release build and current desktop/extracted-policy validation passed. The
binary SHA256 is
`58df6b1d160efb44435a10b731ce6c7efba191a7c0c526f5812a74d5cd3d220d`.
It was atomically staged at `~/.local/state/sophia/bin/hagia`; `deployment.json`,
`hagia-release.log`, the new binary, and its predecessor are retained in
`.artifacts/t004-maximized-navigation/`. This replaces the previously staged
`9349e57` binary and includes that commit's empty-output navigation fix. The
session was not reloaded, and the personal profile remains unchanged.

Deterministic reproduction, regression checks, and physical acceptance are
separate evidence. The installed Super+F navigation report is not yet resolved
by a physical retest. The owning acceptance task is
[t004](../plans/queue-02-cp-14-3-development-session-readiness-and-milestone-14-c.md#t004).
After loading the corrected restart binding with Ctrl+Alt+R and invoking
Ctrl+Alt+Shift+R, verify the live executable identity and test
Super+Home, Super+F, Super+Right, then Super+Left with the pointer still. The
neighbor must become visible and the maximized window must return when focus
does. Dialog and separate fullscreen acceptance remain part of t004's gate.

## Activation correction after the reported failed retest

The user reported that the neighbor remained invisible and clarified that the
camera did not follow keyboard focus after Super+F. Inspection still found
Hagia PID 30410 holding the old executable hash `92c24358...ace52e`, while the
configured binary path held the new `58df6b1d...3d220d` build. The retained
`failed-live-retest.checkpoint` shows window 1 maximized and focused after the
navigation/reversal sequence; it does not capture the intermediate target.
This retest therefore does not establish a failure of `43cfcae`.

The agent's activation instruction was wrong. Ctrl+Alt+R invokes
`session:reload-profile`, and an unchanged profile did not replace the running
process. The existing Ctrl+Alt+F5 binding named `session:restart-wm`, which
explicitly replaces Hagia using the staged executable. The user was told to
use that binding and navigate right once. This second instruction also proved
incorrect: the reserved virtual-terminal chord preempts the binding, as below.
No second production change is justified by the old-binary retest. Verify the
replacement process hash before attributing further results to the repair.

## Reserved restart chord switched away from the desktop

The user reported a session crash after Ctrl+Alt+F5. The owner and Hagia were
both still alive as PIDs 30405 and 30410, and the journal continued recording
resource samples. Records 306218–306227 instead show a queued virtual-terminal
switch, a drained renderer handoff, then seat suspension. No owner fatal or
session exit is recorded. The desktop remains on `/dev/tty7`, login session 14;
the user was on tty2 at inspection. Ctrl+Alt+F7 was given as the return path.

Sophia handles Ctrl+Alt+F1–F12 before configurable shortcuts in physical input
routing. The personal `Ctrl+Alt+f5` restart binding was therefore unreachable.
The agent should have checked reserved input handling before recommending it.
Evidence is retained under `.artifacts/t004-maximized-navigation/vt-switch/`.
This is not evidence of a restart crash or a failure in Hagia `43cfcae`; neither
restart attempt had loaded that binary.

The active profile and its chezmoi source now bind `session:restart-wm` to
`Ctrl+Alt+Shift+r` and explain the VT reservation. Only that binding and its
comment changed in each file. Both desktop-profile validation and Hagia's
extracted-policy validation pass. The active profile SHA256 is
`359f19803a75745cf40d589102514f25de0cd6ba9a5240d2b0394742927f6d2f`.
The chezmoi change is signed and pushed as `1c93c79`.

The corrected shortcut must first be loaded with Ctrl+Alt+R, then invoked with
Ctrl+Alt+Shift+R. The binding change belongs to session shortcut configuration;
its policy fragment is byte-identical, so profile reload alone need not replace
Hagia. Recovery to the existing desktop and a verified new Hagia process remain
required before retrying maximized navigation. No process was restarted by the
agent during this investigation.

The preceding [pointer focus investigation](nsu4a0n2-optional-pointer-focus-follows-presented-targets-through-committed-policy.md)
records the independent empty-output arrow trap and installed drag repair.
The [original stacking incident](../sources/2026-09/legacy-active-0637-2026-09-06--maximized-stacking-and-gtk-startup-in-the-replacement-session.md)
retains the evidence behind the initial expansion layering rule.
