---
id: ksbt5d8f
date: 2026-09-12
kind: investigation
status: investigating
tags: [investigation, x11, protocol, conformance]
---
# The window destroy family is incomplete beyond DestroyNotify

## Question

`DestroyNotify` was missing entirely and is now emitted and delivered
(`fa23b570`, `b3941c04`). That repair covered one window destroyed by one
request. The acceptance matrix it was written against named several cases it did
not cover, and those were left unclaimed rather than assumed working. Which are
real gaps?

Source inspection at `b3941c04`. Nothing here is an execution result.

## What the repair does cover

A successful `DestroyWindow` on a single window now produces both addressed
forms — `event == window` for `StructureNotify` selectors, `event == parent` for
`SubstructureNotify` selectors on the immediate parent — delivered over a real
socket, asserted from a second connection, and failing when the producer is
removed. A rejected destroy produces an error and no event, because the handler
emits only on the success arm.

## Confirmed gaps

**`DestroySubwindows` (opcode 5) is not implemented at all.** It does not appear
anywhere in `crates/sophia-x-authority`: not as a decoded request, not in the
dispatch match, not in the wire tables. A client issuing it takes whatever path
an unrecognised core opcode takes. This is a missing request rather than a
missing notification, so it is larger than the `DestroyNotify` repair was.

**Destroying a window does not destroy its descendants.**
`runtime/windows.rs:471-500` removes exactly the named window: resources, hints,
preferences, buffers, raster, pictures, shapes, background and visual entries,
all keyed on that one id. Nothing walks children. `direct_children` exists and is
used elsewhere (`:362`, `:372`, `:749`) but not on this path.

The protocol requires a destroy to destroy the whole subtree and to notify for
each window, descendants before ancestors. Sophia currently leaves descendants
alive with a destroyed parent, which is both a lifecycle defect and the reason no
descendant notification can be emitted — there is nothing to report because
nothing was destroyed. Adding descendant notifications without the lifecycle
change would announce destructions that did not happen.

## Unverified

**Connection-close destruction.** `dispatch.rs:2195` retires core event
subscriptions when a connection closes, but whether windows owned by that
connection produce `DestroyNotify` to *other* clients selecting
`SubstructureNotify` on their parents was not established. A window manager
learning that a client's window went away on disconnect is the ordinary case, so
this is worth an explicit test either way.

**Unmapped destruction.** Destroying a window that was never mapped should still
notify. Not tested.

**XID reuse with stale subscriptions.** The repair retires subscription and
hierarchy entries after routing specifically so a reused id cannot inherit them
(`b3941c04`). That ordering is exercised, but a test that reuses an id and
asserts the previous window's subscribers receive nothing does not exist.

## Why this is recorded rather than repaired

The descendant gap changes window lifecycle semantics and `DestroySubwindows` is
a new request. Both are larger than the notification repair and sit in the same
tranche as the other confirmed protocol gaps rather than trailing behind them as
cleanup.

The shape is worth noting: four gaps in this protocol family were found in a
single independent gate run, and `DestroyNotify` had been absent without anything
noticing. These are evidence about coverage, not four unrelated bugs.

## Connections

Repaired in `fa23b570` and `b3941c04`. `t087` owns the remaining lifecycle
coverage and repair described above; this note is its evidence and should not be
duplicated into it. The independent conformance gate that found the adjacent
`UnmapNotify`, `NoOperation` and `ListExtensions` gaps is the mechanism that
should catch this class; see `t057`.

One correction from coordination: explicit destruction of an unmapped window is
already covered by the gate, so the "unverified" entry above overstates that
case. The remaining unverified items stand.
