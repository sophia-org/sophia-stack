---
id: qoltxfr5
date: 2026-09-12
kind: plan
tags: [plan, milestone]
---
# Concrete design for admitted synthetic input

Status: proposed, revised after review of `c2b6e262`. No implementation or
deployment authorized.

Authority, principles and the operator's chosen constraints live in
[htm85gg0](../decisions/htm85gg0-admission-ingress-and-provenance-for-synthetic-input.md).
This note says how to satisfy them and does not restate their normative text.

## Corrections to the previous revision

Three claims about this codebase were wrong and are withdrawn:

- **The owner loop does not iterate once per frame.** `primary_frame_pacer`
  gates repaint and caps waits; `owner_loop/authority.rs:18-90` shows the loop
  turning on work availability and preemption. "1ms is 6% of a frame" conflated
  a turn with a frame and is withdrawn along with the derived drain claim.
- **There is no existing atomic revocation boundary to join.**
  `physical_input_phase.rs:936` is a local boolean inside an output-topology
  macro. `input.rs:334` advances leases and the broker control epoch; its
  callers separately clear captures, repeats and handoffs, and
  `owner_loop/lifecycle.rs:44` is a second caller for seat release. This is
  infrastructure to extend, and the extension is specified below rather than
  assumed.
- **`client_keys.rs`'s focus check is a repeat guard.** It decides whether a
  key *repeat* is still deliverable. It is not evidence that dropping a release
  is safe, and the previous design leaned on it as if it were.

## Owning components and state

| Concern | Owner | State |
| --- | --- | --- |
| Grant issue and revoke | Session | grants keyed by `ClientAdmissionId` |
| Admission predicates | Session, reusing Control v1's | pinned peer, credentials, namespaces |
| Decode, refusal, imperviousness | `sophia-x-authority` connection | per-connection grant handle |
| Delayed request | `sophia-x-authority` connection | one `PendingSynthetic` |
| Contribution ledger | Engine, beside `KeyRepeat` | see below |
| Routing, focus, grabs | Engine | unchanged |

### Contribution ledger

Per `(seat, input)` where input is a keycode or a button:

```
sources:   Set<SourceId>              // physical device ids and grant ids
recipient: Option<DeliveredTo>        // surface, lease, epoch, device
```

`DeliveredTo` is recorded **at delivery of the first press**, not recomputed at
release. That is what makes a last-holder release targetable after focus has
moved, which the repeat guard cannot do.

## Contribution state machine

| From | Event | To | Delivery |
| --- | --- | --- | --- |
| empty | press(S) | {S} | **deliver press**, record `DeliveredTo` |
| {..S..} | press(S) | unchanged | none; duplicate is not a second hold |
| nonempty, S absent | press(S) | +S | none; already held |
| S absent | release(S) | unchanged | none; refusal recorded |
| size>1 | release(S) | -S | none; survivor keeps the hold |
| {S} | release(S) | empty | **deliver release to recorded `DeliveredTo`** |

Release delivery targets the recorded recipient, not current focus, so a
retirement after focus moved to B still clears A. If that delivery fails --
recipient gone, lease stale, epoch advanced -- the ledger entry is retired and
the failure recorded; it is never retargeted onto whoever now has focus.

Modifier and repeat ownership follow the aggregate: repeat is cancelled when a
synthetic source that owned it retires, and left alone when a survivor remains.
A passive grab activated in a target client by synthetic input is the target's;
retirement delivers the release that would ordinarily end it and does not cancel
the grab directly.

## Delayed request lifecycle

A delay cannot be a sleep. `connection/dispatch.rs:575` reads synchronously and
then busy-waits on `server_owner` with a 1ms sleep; a sleeping connection cannot
observe EOF, revocation, or a completion it must wait for.

States, owned by the connection thread:

| State | Waits on | Leaves on |
| --- | --- | --- |
| `Ready` | socket read | request decoded |
| `Submitted` | completion channel | Session acknowledges processing |
| `Delayed` | deadline, completion, revocation, **socket readability** | whichever fires |
| `Retiring` | cleanup acknowledgement | contributions retired |

`Delayed` and `Submitted` wait on a poll over the socket FD *and* the channel, so
EOF is observed even with unread pipelined bytes behind the delay, and
revocation does not wait for the deadline. Ingress is paused: no further request
is read, and the bounded receive buffer plus socket backpressure hold the peer.
Descriptors already received stay in `pending_request_fds` and are counted
against the existing arity cap.

The delay **starts when Session accepts the request** and ends when Session
acknowledges that the simulated input has been *processed*, not when the timer
expires and not at enqueue. Subsequent requests from that connection resume only
then, which is what the protocol requires. Sequence numbers and errors stay with
the connection thread; Session returns a completion, not a reply.

Cancellation paths: disconnect, revocation, epoch advance, and capacity refusal
all transition to `Retiring`, which is serviced from reserved capacity.

## The commit boundary, as an extension

Since no single atomic boundary exists, this names one. A grant carries
`(admission, session, seat, grant generation, control epoch)`. Session holds one
serialized operation -- the existing authority work take in
`owner_loop/authority.rs` is the point where batches are already taken in order
-- and within it:

1. recheck the grant against current generation and epoch;
2. resolve focus and grabs;
3. commit or refuse;
4. acknowledge to the connection.

A request that fails the recheck at step 1 does not execute. `input.rs:334`'s
`advance_control_epoch` already refuses if the frontend rejects the advance,
which is the ordering hook: grants are invalidated in the same step, before any
subsequent synthetic work can commit. Per-client disconnect retires only that
client's sources; it never touches another source's contribution or cancels a
target's grab.

**Protected actions.** This design does not assert an existing lock path. What
exists is seat release in `owner_loop/lifecycle.rs` and the session control
surface; the proposed integration is an explicit predicate consulted before
commit, reading seat active state and session lock state, refusing when either
denies. Synthetic provenance is checked there too, including a chord mixing
physical and synthetic sources: if any contributing source is synthetic, the
chord cannot reach a protected action.

## Scheduling: provisional, and measured before fixed

All numbers below are starting points to instrument, not capacity claims.

- **Insertion point.** Synthetic work is taken in the same place authority
  batches are taken, after physical input and after revocation and cleanup.
- **Budget.** A time budget, replenished per service interval rather than per
  turn, since turns are work-driven and not uniform. It stops *starting* new
  synthetic work; a running operation completes and may overrun.
- **Round robin.** The cursor persists across budget stops, so a source that
  missed one interval is first in the next. Progress expectation: every ready
  source is serviced within N intervals, measurable and to be verified.
- **No drain guarantee.** A per-interval event cap and a time budget can each
  stop first; neither promises a seat drains within any frame.

## Capacity accounting

| Population | Bound | Formula |
| --- | --- | --- |
| Authorized injectors | 16 per seat | reserved at grant, released at revoke or disconnect |
| Pending requests | 1 per injector | seat total is *derived*, 16, not a second guard |
| Ledger entries | keys+buttons per seat | bounded by input space, not by injector count |
| Sources per entry | physical devices + grants | ≤ devices + 16 |
| Connection buffer | existing bounded receive | unchanged by this design |
| Seats | per-seat bounds × seats | Session-wide ceiling when >1 seat is exposed |
| Cleanup | all held inputs, not 16 | bounded resumable sweep, reserved capacity |

Cleanup is sized against *accumulated held state*, not pending requests: a
retiring injector may hold many keys and buttons while holding no pending
request. The sweep is resumable so it completes across intervals without needing
the ordinary path.

The seventeenth connection receives `BadAlloc` on its first grant-requiring
request, before any side effect, and its slot is never speculatively held.

## The advertised bundle

**GetVersion** returns the supported version wherever XTEST is advertised.

**CompareCursor** with an explicit cursor obeys the caller's namespace for the
window and cursor lookup. `None` compares against no cursor. `CurrentCursor`
reads what the seat is displaying and is the bundle's one pure-read disclosure;
it is permitted for the administrator bundle and refused otherwise.

**GrabControl** is real scheduling behaviour, not a permission bit. The busy-wait
at `connection/dispatch.rs:592` breaks when the server grab is unowned or owned
by this client; an impervious client also breaks. Cases: an impervious client
progresses under another client's server grab, an ordinary client waits, and
imperviousness is retired on revocation and disconnect so ordinary waiting
returns.

**FakeInput** validates event type, keycode range, button mapping, absolute
versus relative motion, and root or screen target. Invalid fields produce the
protocol's error rather than a clamp. A guessed opcode from an unauthorized
caller is refused exactly as an authorized-but-denied one is, since hiding is
not a boundary.

## Acceptance matrix

Socket, both byte orders: authorized; unauthorized; disabled; revoked during a
delay; reconnect not inheriting pending work; `BadAlloc` at capacity with no
side effect; each protected action denied; discovery agreeing with authorization
*for a given authorization state*; GrabControl progress and wait; FakeInput
field validation and sequence completion; CompareCursor explicit, `None` and
current.

Rust, which the ClassicShared socket host cannot establish:

- **cross-namespace administrative effect is permitted and correct** -- an
  authorized administrator drives the committed focus target in another
  namespace, delivered only to the authoritative recipient, with no broadcast
  and no disclosure beyond it. This replaces the previous case, which promised
  namespace confinement the approved design explicitly rejects;
- unauthorized injector denied regardless of namespace;
- resource and cursor lookups still namespace-restricted;
- ordinary first press and release, before any overlap;
- focus change between press and retirement, release reaching the recorded
  recipient;
- grab change between press and retirement;
- physical + synthetic, synthetic retired, physical hold survives;
- A + B, one retired, the other survives;
- sole holder retired, aggregate and recipient clear;
- failed reconciling delivery retires the entry without retargeting;
- revocation landing on a delayed request at the boundary;
- cleanup of many held inputs while the ordinary path is saturated;
- duplicate press and stale release;
- EOF observed during a delay behind unread pipelined bytes.

## Out of scope

Virtual seat, selective delegation, configurable permission subsets, and any
route by which synthetic input could close a physical acceptance obligation.
