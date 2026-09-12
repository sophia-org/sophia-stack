---
id: qoltxfr5
date: 2026-09-12
kind: plan
tags: [plan, milestone]
---
# Concrete design for admitted synthetic input

Status: proposed. No implementation or deployment authorized.

Authority, principles and the operator's chosen constraints live in
[htm85gg0](../decisions/htm85gg0-admission-ingress-and-provenance-for-synthetic-input.md).
This note says how to satisfy them and is reviewable against code. It does not
restate their normative text.

## Owning components and state

| Concern | Owner | State |
| --- | --- | --- |
| Grant issuance, revocation | Session | grant table keyed by `ClientAdmissionId` |
| Admission predicates | Session, reusing Control v1's | pinned peer, credentials, namespaces |
| Request decode, refusal | `sophia-x-authority` dispatch | none |
| Pending delayed request | `sophia-x-authority`, per connection | at most one, see below |
| Contribution ledger | Engine, beside `KeyRepeat` | per `(seat, keycode/button)` source set |
| Routing, focus, grabs | Engine | unchanged |

The contribution ledger is the one genuinely new structure. Everything else
extends something that exists.

## Connection admission and discovery

A grant is issued per X connection, after the startup-only option is enabled
*and* the connection independently satisfies the host-user predicates. It binds
`(ClientAdmissionId, session identity, seat, grant generation, security epoch)`.

Discovery reads the same grant. `advertised_extension_names()` and
`extension_query_result` both consult it, from one function, so the two cannot
disagree -- the property t086 repaired by hand and which a second enumeration
path would reintroduce. Without a grant, XTEST is absent from both.

## Contribution ledger and duplicate transitions

Each held key or button carries a set of contributing sources, physical and
synthetic alike. Transitions:

- **Press from a source already holding** -- no aggregate change, ledger
  unchanged. The duplicate is not an error and not a second hold.
- **Press from a new source** -- source added. Aggregate already held, so no
  delivery.
- **Release from a source not holding** -- ignored, refusal recorded.
- **Release from a holder that is not the last** -- source removed, no delivery.
  This is the physical+A and A+B case: the survivor keeps the hold.
- **Release from the last holder** -- source removed, aggregate now clear, and
  authority-owned reconciliation delivers the release to the existing recipient
  under the current target, lease and epoch rules.

`client_keys.rs` already models a pressed key as
`SessionClientPressedKey { surface, seat, device, keycode }` and validates the
target is still current before delivering a repeat. The ledger reuses that
validation: a reconciling release delivers only where the existing target is
still current, and otherwise cancels rather than retargeting.

## Delay and scheduling

The owner loop is paced by `primary_frame_interval`, derived from
`head.refresh_millihz` -- 16.67ms at 60Hz. Every number below is a fraction of
that, and none is a preemption promise.

- **Service budget: 1ms per turn**, about 6% of a 60Hz frame. Chosen so that
  synthetic work cannot displace composition or physical input in the same turn.
  The budget stops *starting* new synthetic work; an operation already running
  completes. A non-preemptible operation can overrun it and the design does not
  claim otherwise.
- **16 events per owner turn**, so a saturated seat drains in about one frame
  rather than starving over many.
- **Round robin across ready sources**, so one injector cannot monopolise.
- **Physical input, revocation and cleanup are serviced before normal synthetic
  work**, unconditionally.

A delay is a `Deadline` per pending source, checked against the loop's existing
`Instant` reads. The full CARD32 millisecond range is accepted -- about 49.7
days -- because clamping would change conformant XTEST behaviour. A long delay
occupies one deadline and one buffered request, nothing that scales with its
length, and does not block the server, disconnect, or revocation. Only that
client's own later protocol work waits.

## The atomic boundary

Revocation already has a boundary in this loop.
`physical_input_phase.rs:936` advances the security epoch and then
`advance_application_input_security_epoch` revokes input leases, floating
pointer interaction and chrome captures, resets the focus handoffs, and calls
`key_repeat.cancel_seat(seat)`.

Synthetic grants join that boundary rather than inventing one. At an epoch
advance: grants bound to the old epoch stop being valid, pending delayed
requests bound to it do not execute, and their contributions retire under the
ledger rules above. Work already committed past the boundary is not undone.
Focus, grab and security resolution are already coherent there, which is the
reason to use it.

Lock, VT loss and seat departure reach the same path. A reconnecting client
gets a new grant; nothing pending survives, so there is no replay.

## Cleanup and overload

Cleanup capacity is reserved outside the request budget, so retiring
contributions works when the ordinary path is saturated -- cleanup that needs a
free queue fails exactly when it is needed.

Capacity refusal is `BadAlloc`, returned before any input side effect. Never a
silent drop, never a clamp.

**On the seat cap.** With one pending request per source and 16 sources, a
16-per-seat request cap is *derived*, not additional protection, and the design
says so rather than implying a second guard. What does need separate accounting,
and gets it: ledger memory held per seat, the bounded ingress buffer per
connection, the per-seat totals when more than one seat is exposed, and the
reserved cleanup capacity, which is sized independently of injector count.

## Refusal record fields

`kind` (absent | unauthorized | revoked | capacity | protected-action |
malformed), `client`, `grant generation`, `security epoch`, `seat`, `request`,
`decision`. The point is that intentional denial stays distinguishable from
missing implementation, which the wire alone does not settle.

## Acceptance matrix

Socket cases, both byte orders: enabled and authorized; enabled and
unauthorized; disabled; revoked mid-delay; reconnect not inheriting pending
input; capacity refusal returning `BadAlloc` with no side effect; each protected
action denied; discovery agreeing with authorization in `ListExtensions` and
`QueryExtension`.

Rust cases, which the ClassicShared socket host cannot establish:

- two namespaces, injection in one not observable in the other;
- physical + synthetic on one key, synthetic retired, physical hold survives;
- injector A + injector B, one retired, the other survives;
- sole synthetic holder revoked, aggregate and recipient clear, no stuck key;
- revocation landing exactly on a delayed request at the boundary;
- cleanup succeeding while the ordinary queue is exhausted;
- healthy peers and physical input serviced throughout a delayed caller's block;
- duplicate press and stale release, neither producing a second hold nor a
  spurious delivery.

## What this design does not do

It does not implement a virtual seat, selective delegation to a confined
harness, or configurable permission subsets. It does not make synthetic input
eligible to close a physical acceptance obligation.
