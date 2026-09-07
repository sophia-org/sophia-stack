---
id: knjco01f
date: 2026-09-06
kind: investigation
status: awaiting-physical-acceptance
tags: [investigation, x11, input]
---
# Pointer queries must share admitted namespace state

## Question

Why does a newly connected client see the pointer at the screen corner while
another client is receiving pointer input?

## Evidence

The baseline is `2630d936`, on master. The read-only live probe used installed
`ef1ba0e7` in session
`00000001788748566248-d7be692d-f3cf-4f0d-9e70-9e6dfb3e7359`.
A fresh authenticated connection on display 77 returned root and window
positions `(0,0)`, no child, and mask zero. No input was injected into that
session, and its authentication cookie was not printed or retained.

The old private-state test passed because it seeded the exact connection-local
snapshot being queried. A new socket regression routed `(143,259)` to one
client, waited for delivery, and queried through a fresh connection. It failed
with `left: 0, right: 143` before the repair and passed afterward. Temporary
socket tests require host execution because the development sandbox refuses
Unix socket binding; that refusal was a fixture limitation, not the defect.

## Finding and resolution

The initial t060 note missed the socket reply override. Dispatch constructed
zero-valued replies, then the connection worker patched them from its input
writer's snapshot. Only a connection that had received pointer input had that
snapshot. Its cached mask also missed modifier changes without motion, and its
private window hierarchy could not resolve a peer's window.

The shared input authority now records the last logically processed observation
for each namespace. Broker publication follows epoch and freeze checks and
precedes subscription selection and socket delivery. Core QueryPointer,
XIQueryPointer, and XIQueryDevice position/scroll fields read this state.
Pointer mappers are namespace-and-seat scoped; connection cleanup retains
observations for surviving peers and drops them at the last client's exit.

The snapshot names Engine's original surface, separately from X grab delivery.
The runtime resolves its current X descendants, stacking, shapes, map state,
and window-relative coordinates. It preserves Engine's root/local transform
pair, adjusts the local anchor across configure/reparent changes without new
motion, and invalidates a destroyed anchor. Child lookup checks the surface's
generational identity. The Engine wire boundary and blind WM do not change.

Before an observation, and after revocation, the pointer starts at zero with
empty button state. Queries expose the last input admitted to the namespace;
they do not promise continuous physical sampling while input goes elsewhere.
Completing TranslateCoordinates' child field is outside this repair.

## Validation and remaining work

Four socket regressions cover both byte orders, existing and newly connected
peers, core/XI agreement, nested and overlapping children, unmap, reparent,
destruction, Engine-driven geometry changes, transformed input, modifier-only
changes, held buttons, scroll valuators, confined peers, redirected grabs,
frozen input, thaw, and epoch revocation. They use production routing and wire
interfaces, with no new tests in production source.

`cargo xtask check` passed, including workspace tests, Clippy, source-layout
checks, retained-evidence verifiers, and the host buffer-age proof. One Clippy
suggestion was then applied as an equivalent boolean simplification; strict
`cargo clippy --offline -q -p sophia-x-authority --all-targets -- -D warnings`
and the four focused socket tests passed afterward. Formatting and diff checks
also passed. The last helper edit only wraps long lines.

Evidence is retained privately at
`~/.local/state/sophia/development-evidence/t060-2630d936-7664a3905c3b`.
It contains the full and focused logs, strict-Clippy result, source patch,
compiled-test identity, and verified SHA-256 inventory. The candidate is
`2630d936eec4cbec7b1bdb6a893f458096318c67` plus source patch SHA-256
`7664a3905c3bb42191e46f80db1725e21db9127a748db450ce2c004bb7bec990`.
These records exclude session credentials and browser dumps.

These are deterministic checks. One installed menu-placement and drag check
remains the physical acceptance gate for t060.

The user's replacement live session is
`00000001788751946481-31db6852-d07f-4f08-8ed9-87f63a561f59`, running
`86ab21e4879cc5b3154ca1192775de73e6e6a030`. Its binary SHA-256 is
`0d972718c734751aba7fe58eb5075eb9f2f776bb4da7fdea3d69e80f98ac4d06`.
Preflight, input guard, and graphics takeover completed; the recorder was
running with no discarded records or storage errors, and the inspected startup
events contained no failure, rejection, protocol-error, or recovery record.
A fresh read-only QueryPointer probe still returned the empty initial state.
Pointer activity in an application had not yet been confirmed, so this is
neither physical acceptance nor evidence that the repair failed.

After pointer activity, another fresh connection returned root and window
coordinates `(604, 760)`, mask zero, and a nonzero child. The namespace-shared
observation is now demonstrated in the installed session. The user reports
[blank or black Thunar menu portions](ce2b55uy-blank-thunar-menus-and-frozen-brave-need-separate-pixel-and-delivery-evidence.md);
menu rendering and the unreported drag check still prevent physical acceptance.

## Connections

- [t060](../plans/queue-11-parallel-production-readiness.md#t060) owns the exit;
  active status remains in the task ledger.
- [X authority contract](../../sophia-x-authority.md) defines input ownership.
- [Brave watchdog](h0vxis10-brave-gpu-watchdog-repeats-during-live-use.md) was
  investigated alongside this repair; its GPU hang is not attributed to
  QueryPointer.
