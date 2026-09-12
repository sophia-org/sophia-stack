---
id: fbtlnuad
date: 2026-09-12
kind: investigation
status: accepted
tags: [investigation, shell, protocol, policy]
---
# Nothing shell-facing carries workspace state or the active output

## Question

The plan for an admitted content shell put the ironbar `WorkspaceClient`
backend after the content capability, on the assumption that workspace state
already reaches a shell and only drawing was missing. Does it?

## Evidence

No. `sophia_shell_v1` has no workspace, view, indicator, or active-output
record. Searching the schema for those terms returns one hit: the `focused`
field of `TabsGroup`.

Tab groups look like a workspace feed and are not one.

| Property | What tab groups actually do |
| --- | --- |
| Origin | `projectTabTree` in Hagia walks a tab tree; groups exist only where tree nodes do |
| Gating | Leaf groups are emitted only inside `if tree.frameStyle` — plain scroller layout emits none |
| Empty output | An output with no windows has no tree nodes, so no group |
| Meaning | A tab strip's decoration, not an enumeration of views |

`ShellTabGroup` does carry `output` and `focused`, which is what makes the
mistake easy. But a shell consuming tab groups sees nothing at all on an output
holding no windows, which is precisely the case this work exists to fix: focus
moves to an empty DP-2 and the screen says nothing.

The indicator records that *would* answer this exist and are implemented —
`ProjectionIndicator` and `ProjectionOutputStatus`, `sophia_wm_v1` revision 3,
record kinds 3 and 4. They reach the Engine, which holds them as
`PolicyIndicatorPublication`. Nothing republishes them to a shell.

## Finding

Both remaining phases are blocked on the same missing piece, and it is not the
content capability.

- **Drawing** needs the content capability, which is designed and modelled in
  [adr 6ndjwffd](../decisions/6ndjwffd-content-capability-design-for-sophia_shell_v1.md)
  and not implemented.
- **Knowing where focus is** needs a shell-facing indicator and active-output
  vocabulary, which that record explicitly scopes out and allocates no kinds
  for.

The second is the smaller of the two and is what the original complaint
actually requires. It does not depend on the content capability: a
descriptor-only shell could consume it. Sequencing it first puts the
information on the wire before anything new can draw it.

## Proposed vocabulary

Revision **6**, capability bits **9** `view_indicators` and **10**
`indicator_activation` (requiring bit 9), message kinds **181–186**. Revision 5
and bits 7–8 stay reserved for the admitted content design, which is accepted
even though it lands later; consuming another record's reservation because it
has not shipped yet would make accepted allocations meaningless.

| Kind | Message | Carries |
| --- | --- | --- |
| 181 | `IndicatorsBegin` S | epoch, snapshot generation, `active_output` with an explicit present flag, indicator and status counts |
| 182 | `IndicatorsOutputStatus` S | output, `focus_bits`, layout name — mirrors `ProjectionOutputStatus` |
| 183 | `IndicatorsEntry` S | output, indicator, action, slot, `state_bits`, label — mirrors `ProjectionIndicator` |
| 184 | `IndicatorsEnd` S | epoch, snapshot generation |
| 185 | `IndicatorActivate` C | the opaque action, for an exact snapshot, plus an event id |
| 186 | `IndicatorActivateOutcome` S | event id, status, reason |

`active_output` is **one global optional identity with an explicit present
flag**, not a per-output boolean, because per-output booleans can contradict
each other and because the motivating case is an output that is focused while
holding no window — so there is no seat focus to infer it from. It is sourced
from `LivePublicPolicyState.active_output`, which is updated on commit, and not
from `NativeOutputActivationPlan::focused_output`, which is a topology startup
fact rather than live focus.

Activation carries the opaque action the policy client authored. The shell never
names a view; it echoes what it was given for the snapshot it was presenting, so
a stale pill cannot switch a view that has since moved. That preserves the
existing issuer and epoch checks rather than letting a shell-local integer stand
in for policy authority.

## Remaining work

This note records the design, not an implementation. Drafting it directly into
`protocol/sophia-shell-v1.kdl` was reverted: the schema header would have
declared `interface-revision=6` while the Rust still serves 4 and nothing
implemented the records, and `tools/check_shell_protocol.sh` passes in that
state rather than catching it. The schema is not the place to hold an unbuilt
interface.

Implementation follows the established extension pattern — new modules beside
`packets/shell_v1.rs` and `ipc/shell_v1.rs`, capability-gated sends in
`runtime/src/shell_transport.rs`, golden frames and a generator, the independent
C client, and Narthex's independent Nim decoder — and must prove a
non-opted-in client receives no new bytes.

## Connections

`t081` in [todo](../../../todo.md) owns the visible outcome. The content
capability record owns drawing and names this vocabulary as the sibling
extension it deliberately does not specify.
