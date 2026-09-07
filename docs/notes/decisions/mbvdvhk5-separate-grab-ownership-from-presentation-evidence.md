---
id: mbvdvhk5
date: 2026-09-07
kind: adr
status: accepted
tags: [adr, input, security]
---
# Separate grab ownership from presentation evidence

## Context

The [Qt menu investigation](../investigations/37xvg0y7-qt-menus-fail-to-open-while-pointer-leases-are-rejected.md)
found two ordering failures in explicit pointer preparation: the session had
not yet observed the mapped target, or the target had not yet appeared in a
retired frame. A client may wait for grab success before drawing, so delaying
success until presentation can create a circular wait.

The application lease contract also required an exact output presentation
epoch. That epoch changes when unrelated layers enter or leave the output.
It represents a scene revision, not the identity or authority of the held
grant. Relaxing equality without replacing its evidence would nevertheless be
unsafe: the existing application-only hit can miss shell or chrome occlusion.

## Decision

Separate authoritative mapping, grab ownership, and physical-input readiness.
X11 owns its grab rules. Engine grants the profile-scoped lease and validates
physical input against current presentation evidence. The WM receives no new
input payloads or protocol identities.

A Prepare names its connection's last observation actually enqueued. Engine
must apply and account for that observation prefix before validating mapping,
admission, and owner eligibility. Transaction gaps are allowed. The frontend
releases shared authority-state locks while awaiting its synchronous reply.
Neither preparation nor activation waits for scanout.

Lease phases remain provisional, active, and releasing. Presentation binding
is independent. Engine owns one exhaustive readiness decision; authorization
resolves the current lease and checks readiness again. Readiness observed
earlier is never a transferable permission token.

An application scene revision requires renewed evidence of the original
target's eligible presentation and the pointer's permitted scope, including
shell, descriptor, chrome, tab, and secure occlusion. Same-scope application
overlap remains permitted. Exact surface generation, admission, authority,
control epoch, device restriction, and pinned output remain mandatory. Shell
capture and disclosure rules do not change.

Pending preparation, unpresented ownership, and held input are bounded.
Timeout or cancellation removes the exact request, reservation, and held
sequence together; missing prerequisites never permit a fallback. Promotion
inherits the pinned output, and late responses cannot restore retired owners.
The [normative contract](../../target-resolved-input.md#bounds-and-cancellation)
owns the concrete bounds and their timer anchors.

## Alternatives

- Keep exact scene equality. This preserves conservative refusal but makes
  unrelated desktop changes terminate otherwise valid interactions.
- Remove equality while trusting the application hit alone. This can route
  through shell or secure occlusion and is rejected.
- Wait for presentation before replying with grab success. A client that draws
  after success may never produce that presentation.
- Authorize on timeout using current partial state. This bypasses the very
  prerequisite the request was waiting for and is rejected.
- Add a public deferred X result or a new popup wire protocol. Neither is
  needed to fix internal ordering and evidence ownership.

## Consequences

The Engine state machine gains explicit presentation binding and bounded
unpresented ownership. The session must resolve scope against every presented
occlusion source and handle cancellation across pending requests and input.
The frontend must preserve exact request ordering and rollback identity while
releasing shared locks during bridge waits.

The design allows independent scene updates without silently extending a
grab's authority. Its cost is more explicit lifecycle state and a broader
presented-scope resolver. The epoch amendment is safe only when that resolver
and its negative tests are integrated. A green Qt probe does not establish
that every reported Brave input failure is repaired.

## Acceptance and connections

Created as proposed on 2026-09-07. Accepted the same day when the user approved
the final t066 implementation plan and requested implementation. This records
architectural acceptance, not completed implementation or physical acceptance.

[Target-Resolved Input](../../target-resolved-input.md) carries the current
contract. [Sophia X authority](../../sophia-x-authority.md) describes frontend
ownership. The bounded arbitration and admission models are documented in the
[TLA+ validation guide](../../../validation/tla/README.md); they check the
contract, not a refinement proof of the Rust implementation.
