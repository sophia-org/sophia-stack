---
id: htm85gg0
date: 2026-09-12
kind: adr
status: proposed
tags: [adr]
---
# Admission, ingress and provenance for synthetic input

## Context

[t030](../plans/queue-11-parallel-production-readiness.md) excluded `XTEST` and
said why: it is four requests and cheap to write, but the reference server's is
entirely ungated, and their own notes record that as a known gap -- any client
can drive the pointer and keyboard. Sophia's input is session-owned authority,
so t030 held that synthetic input needs an explicit admission story *before it
exists at all*, and invited a revisit when conformance tooling made that story
worth designing. `xts5` drives the mouse through XTEST, and a physical-acceptance
harness wants it for setup. This is that revisit.

This note settles admission, ingress and provenance. It does not authorise
implementation; it is the contract an implementation would have to satisfy.

## What the shared seat cannot promise

The earlier framing -- that XTEST cannot be namespace-scoped -- was too strong,
and is withdrawn. A virtual seat, or an injector that verifies and bounds every
effect to an admitted namespace, is a coherent design. What is true is narrower:

> The **shared-seat** path cannot promise namespace confinement. Injected input
> follows global focus and grabs, so its effects land wherever those point,
> which may be a window in another namespace.

That is why the shared-seat path is desktop administration in the sense
`docs/scripting.md` defines, and why it is refused by default. It is not a claim
that no scoped design exists. `ClassicShared` membership is not a synthetic-input
grant either: scripting.md separates resource admission from control
authorization, and shared resource access does not imply the right to drive the
seat.

## Admission

**Authorization is per admitted caller, not per server.** A global enable makes
the capability *available*; it must never be read as authorizing every connected
client. Each admitted client carries its own synthetic-input authorization,
derived from verified admission, never from a caller-supplied field, a matching
UID, or membership of a shared namespace.

**Discovery follows authorization.** `ListExtensions` and `QueryExtension` report
XTEST only to a caller authorized for it, and they must agree -- the same
property t086 repaired for the rest of the extension set. Absence is the default
for an unavailable capability. Hiding the extension is not a security boundary:
a client that guesses the opcode still meets the same denial, and that denial is
an explicit error, never silence or a dropped request.

**Authorization is rechecked at execution, not only at dispatch.** Delayed input
and revocation both mean the decision can change between a request arriving and
its effect landing. A revoked caller's pending synthetic input does not execute.

`BadAccess` is the denial, and it is intentional policy rather than a missing
feature. That distinction belongs in the refusal log, because the two look
identical from the client side and only one is a bug.

## Ingress

**Synthetic input enters where real input enters.** Session authorizes; Engine
remains the input authority. `FakeInput` must not write client events directly,
because doing so would step around committed focus, active grabs, the security
epoch, input recovery, and lock and VT policy -- every protection the real path
enforces.

There is already a precedent in tree:
`crates/sophia-session/src/live_session/owner_loop/input_proof.rs` injects
through the committed Engine path and *asserts* it traversed committed focus
rather than assuming it. That is the shape, and it is evidence the ingress
constraint is satisfiable rather than aspirational. It is not itself a general
XTEST grant.

**Bounded queueing, and cleanup that survives failure.** `FakeInput`'s time field
is a **delay, not a timestamp**, and it constrains the processing order of that
client's subsequent requests. A contract for it has to state the queue bound,
what happens when the bound is reached, and how a caller that disconnects or is
revoked mid-sequence has its held keys, buttons and grabs released. A synthetic
press with no release is a stuck modifier for the whole seat.

## Provenance

**Synthetic input keeps a synthetic identity, end to end.** It must never be
promoted into something indistinguishable from a physical gesture. The existing
`source=synthetic` labelling is the precedent to extend, and the label has to
survive into whatever evidence a run produces.

**Acceptance consequence.** Synthetic input may provide setup, and may provide
explicitly labelled regression evidence. It **cannot substitute for the physical
evidence** t077, t060 and t062 require: it establishes nothing about hardware,
libinput, VT delivery, physical timing, or recovery through the original device
path. The earlier phrasing "never any evidence" was too broad and is withdrawn;
the correct constraint is that it never substitutes for their required physical
evidence.

Where a run uses synthetic setup before a physical phase, the synthetic phase
releases its keys, buttons and grabs and settles pending work first, and the two
phases stay separately identifiable in the record.

## The other three requests

`GetVersion` is a version handshake and can be answered wherever the extension
is advertised at all.

`CompareCursor` splits. With an explicit cursor and window it is a
resource-scoped question and can be answered within the caller's namespace.
Asking about the **current** cursor is a disclosure question about what the seat
is displaying, and needs its own check or an admitted virtual-seat reading.

`GrabControl` makes the calling client **impervious to server grabs**. It does
not release pointer or keyboard grabs and does not grant injection; the earlier
reading of it as device-grab control was wrong. Gating it with the
administrative surface is defensible, but its actual scheduling semantics have
to be implemented and tested rather than stubbed. Neither of these may report
behaviour the server does not perform.

## Consequences

Default remains absence, as t030 decided. What changes is that there is now a
contract an admitted implementation must satisfy, rather than an open gap.

An implementation admitted under this note owes the independent manifest cases
for enabled, disabled, unauthorized and revoked callers, in both byte orders.
XTS results stay separate from physical acceptance records.

Nothing here authorises writing the extension. It states what writing it would
have to honour.
