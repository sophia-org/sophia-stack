---
id: wjctvtsk
date: 2026-09-12
kind: investigation
status: closed
tags: [investigation, shell, protocol, architecture]
---
# Indicator delivery does not yet provide native panel presentation

## Question

`sophia_shell_v1` revision 6 now carries view indicators and the active output,
and an ironbar backend consumes them. The plan that produced both sequenced a
visible ironbar panel as the next step. Is it?

Reviewed by source inspection at `2f38ac75`, plus `~/src/ironbar` at
`e2910c7` on `feat/sophia-workspaces`. No session was launched and nothing here
is an execution result.

## Finding

> Today's admitted native shell has an indicator feed but no implemented path to
> present its own toolkit-rendered content. The existing descriptor-switcher path
> also lacks an independent persistent-panel lifecycle. The full content design in
> [adr 6ndjwffd](../decisions/6ndjwffd-content-capability-design-for-sophia_shell_v1.md)
> specifies both pixel submission and that panel lifecycle, including
> reservations. Implementing it would resolve those protocol gaps; it would not
> automatically adapt ironbar's Wayland-dependent GUI or deliver the complete
> visible outcome.

`LiveMetadataShell::start`
(`crates/sophia-session/src/live_session/metadata_shell.rs:96-140`) launches the
shell into a protection domain whose environment and mounts are built at
`crates/sophia-runtime/src/supervisor/protection.rs:342-448`, with protection
evidence required at `metadata_shell.rs:620-631`. The environment is cleared and
rebuilt from the session's explicit set plus `PATH`; the shell receives no
display connection. The generic supervisor can launch unprotected processes, but
that is not an admitted native-shell fallback.

State confinement precisely, because the loose version is wrong. A confined
process can rasterize its own pixels in memory, and descriptor clients already
cause Engine-rendered UI to appear. What is missing is presentation of arbitrary
client content. The first content transport the ADR selects needs no X or
Wayland display at all: it moves CPU pixel bytes over the existing authenticated
shell stream. That is not a claim that every toolkit supports display-free
rendering, nor that confinement inherently rules out buffer handles — ADR §1
selects bounded byte transfer, and another transport would need its own
specified ownership and synchronization.

## What the full content design resolves

Describes the design once implemented, negotiated and explicitly permitted by
the operator. None of it ships today.

| Requirement | What the full content design resolves |
| --- | --- |
| Present client-generated pixels | Bounded CPU resources, transfer, candidate submission, Engine composition, outcomes and resource release. |
| Persistent panel independent of the switcher | `ContentOutputFacts`, panel `ContentAllocationRequest`/`Result` (kinds 162–164), content candidates (172–174), and demand/permit pacing (176–177). No switcher descriptor entry is required. |
| Reserve panel work area | `Surface.reservation_extent`, allocation-specific `allowed_reservation_extent`, the reservation capability and applicable limits, with coherent content, work-area and WM presentation. Allocation permission alone does not change work area. |
| Panel button and popout interaction | The specified discrete input and dismissal lifecycle. General keyboard, pointer-motion and host-service access are not implied. |
| Know the active output and view indicators | Supplied separately by the revision 6 indicator extension. Pixel transport does not supply this state. |
| Run the current ironbar GUI | Still requires an ironbar startup and presentation adapter compatible with the selected path. A workspace-data backend is not that adapter. |
| Enforce today's descriptor reservation allowance | Remains the separate defect `t083`; new content support does not repair the old path. |

The panel explanation is anchored in adr `6ndjwffd` lines 339–363 and 397–413.
Requiring descriptor capability bit 0 for the first combined client does not make
its content candidates depend on visible switcher entries. Landing only pixel
upload would be insufficient; landing the full specified allocation, candidate,
input and presentation lifecycle addresses the native persistent-panel
limitation.

Today's narrower constraint is real and separate: `ipc/shell_v1.rs:401-440`
refuses a reservation on a non-visible candidate and refuses a visible candidate
with no entries, `runtime/src/shell_transport.rs:261-283` requires every entry to
come from the current snapshot, and `physical_input_phase.rs:595,1087` are the
only request sites, both switcher-driven. Scope that to the native descriptor
path: ordinary X11 panels already reserve through struts.

## Alternatives and independent findings

A separately confined X shell is a coherent alternative design. Sophia's resource
namespaces and OS protection domains are separate boundaries
(`docs/namespaces-and-portals.md:55-80,102-107`). But the live X policy calls
`admit(self.namespace, …)` at `live_session/x_frontend.rs:23-28`, so passing
existing display credentials or selecting `Confined` does not give a shell its own
namespace apart from the session group. Such a path needs distinct admission and
defined placement, input disclosure, grants and revocation, and it would add a
second presentation path whose contract must be compared with the content design.
Two claims to avoid: that every X connection exposes every client, which is not
true given the confined profile; and that network denial or sparse font
configuration alone proves a GTK client cannot work.

Bubblewrap is the current enforcement backend, not the architectural requirement.
[Pnut evaluation](../../pnut-evaluation.md) records the replacement seam and
holds Pnut as a candidate rather than an adopted improvement. Replacing a backend
while preserving its grants does not supply the missing presentation protocol.

An independent ironbar blocker survives all of the above:
`~/src/ironbar/src/main.rs:212-214` initializes Wayland unconditionally before
the UI, and the worker's `connect_to_env().expect(…)` at
`clients/wayland/mod.rs:260` fails without a reachable Wayland compositor, with
the main thread's synchronous roundtrip failing through the disconnected channels
at `:192-200`. "Reachable compositor" rather than "`WAYLAND_DISPLAY` unset",
since an inherited `WAYLAND_SOCKET` is another connection mechanism. Source
confirmed, not reproduced live. Repair belongs to the ironbar owner.

The descriptor reservation allowance is not enforced against the configured
depth; that is [gl2ooa99](gl2ooa99-shell-reservation-admission-ignores-the-configured-panel-depth.md)
and `t083`, already filed, and is not duplicated here.

## What this does not invalidate

The revision 6 indicator wire, its session publisher and the activation path are
sound and independently decoded by three implementations. The ironbar data
consumer is tested. The phase-ordering error was not that the indicator work came
first — it was expecting that phase alone to produce a visible panel.

## Connections

[Nothing shell-facing carries workspace state](fbtlnuad-nothing-shell-facing-carries-workspace-state-or-the-active-output.md)
asked the prior question and was answered by building the feed. `t081` in
[todo](../../../todo.md) owns the visible outcome and stays open.
