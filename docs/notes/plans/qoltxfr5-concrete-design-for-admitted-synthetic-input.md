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
retirement after focus moved to B still clears A.

**A failed release becomes reconciliation debt, not a log line.** A recipient
whose lease is stale or whose epoch advanced may still be alive and still
believe the input is held; that is a different case from a recipient that is
gone. So:

- the source's authorization and its ledger contribution retire immediately --
  a revoked injector gains nothing by holding debt open;
- the **debt** persists, bounded, until either targeted clearing is acknowledged
  by the recipient, or recipient recovery or termination establishes there is no
  surviving state to clear;
- debt is settled by cleanup authority that outlives the grant and the lease,
  carrying the recorded target identity rather than resolving a current one;
- it is never replayed to current focus, and the recorded identity is not
  disclosed to anyone but the settlement path.

`client_keys.rs`'s flush path already retains failed releases rather than
discarding them; this follows it.

**`DeliveredTo` is invalidated when something else already cleared that
delivery.** A global focus transition that released A's input clears the record,
so a later retirement cannot deliver a second release for the same hold.

**Repeat ownership is explicit.** Repeat belongs to the source that started it.
If A started repeat and B remains a holder, retiring A **cancels the repeat**
and does not transfer it to B: B never asked to repeat, and inheriting it would
manufacture input B did not request. The aggregate hold survives; only the
repeat ends.

A passive grab activated in a target client by synthetic input is the target's.
Settlement delivers the release that would ordinarily end it and does not cancel
the grab directly.

## Delayed request lifecycle

A delay cannot be a sleep. `connection/dispatch.rs:575` reads synchronously and
then busy-waits on `server_owner` with a 1ms sleep; a sleeping connection cannot
observe departure, revocation, or a completion it must wait for.

**Readability is not EOF.** `POLLIN` is also set by ordinary unread pipelined
bytes, and with ingress paused those bytes stay unread, so polling `POLLIN`
alone spins forever. The wait is therefore on:

- `POLLRDHUP` and `POLLHUP` and `POLLERR` for peer departure, which is what
  actually distinguishes a gone peer from a talkative one;
- a **pollable** completion and revocation notifier -- an eventfd or pipe read
  end registered in the same `poll` set, not an unpollable channel;
- the deadline, as the `poll` timeout.

`POLLIN` on paused ingress is *not* a wake reason and is masked out. A stream
read returns EOF only after buffered bytes are consumed, so departure is
concluded from the hangup flags rather than from a read this design refuses to
perform. A write-half-close with `POLLRDHUP` and no `POLLHUP` means the peer
stopped sending but may still read: pending work continues to completion, and
the connection closes normally afterwards.

**States, owner, and what each waits on.**

| State | Timer | Waits on | Leaves on |
| --- | --- | --- | --- |
| `Ready` | none | socket `POLLIN` | request decoded |
| `Submitted` | none | notifier: accepted | Session accepts, arming the delay |
| `Delayed` | armed by **Session at acceptance**, observed by the connection as a `poll` timeout | hangup flags, notifier, deadline | deadline reached, or cancelled |
| `Executing` | none | notifier: processed | Session reports processed |
| `Retiring` | none | notifier: settled | contributions and debt settled |

The connection owns the `poll`; **Session owns the delay's start**, so the delay
is measured from acceptance rather than from whenever the connection got
scheduled. `Delayed` reaching its deadline transitions to `Executing`; the next
request is read only after `Executing` reports processed. Sequence numbers and
errors stay with the connection thread throughout.

**Server-grab wait services cancellation too.** The busy-wait at `:592` gets the
same treatment: it waits on the notifier and hangup flags alongside grab
ownership, so a client blocked on another client's server grab still observes
revocation and departure. An impervious client does not enter it. A delayed
`FakeInput` whose deadline expires while another client holds a server grab
waits there if ordinary, proceeds if impervious, and remains cancellable in
either case.

**Capacity refusal cancels the request, not the holds.** `BadAlloc` refuses the
one request that could not be admitted and leaves existing contributions intact;
it does not enter `Retiring` and does not revoke the grant. Only disconnect,
revocation and epoch invalidation retire holds. The previous revision routed all
capacity refusals into `Retiring` without saying so, which would have dropped a
caller's existing holds because a later request did not fit.

## The commit boundary, as an extension

Since no single atomic boundary exists, this names one, and the previous
revision's "same step" claim is withdrawn:
`advance_control_epoch` (`routing/broker.rs:94`) is a compare-and-swap on an
atomic counter, and `route_pending` (`:497`) notices and *applies* it later.
Advance and application are two moments, so nothing is atomic by virtue of
calling it.

**Vocabulary, fixed here because the review found it ambiguous.**

| Term | Meaning |
| --- | --- |
| queued | accepted at the connection, stamped with its bound generation and epoch, not yet validated |
| committed | validated under the serializing operation and admitted to routing |
| processed | routing has applied it and Session has produced completion |

A connection's next request waits for **processed**, not queued or committed.

**The serializing operation.** A single Session-held lock covers, as one
indivisible step: execution-time revalidation, ledger mutation, focus and grab
resolution, and revocation application. Anything that changes grant validity
takes the same lock. This is the mechanism the previous revision gestured at and
did not name.

**Epoch is carried, never re-stamped.** A queued request keeps the generation
and epoch it was bound to at acceptance. Validation compares those against
current state under the lock. A later enqueue must not promote it: without this,
a request queued before an epoch advance would be validated as though it arrived
after, which is precisely the race the sender's then-current stamping invites.

**Two race orders, both defined.**

- *Revocation wins:* revocation takes the lock first, the grant's generation is
  invalidated, the queued request fails revalidation and is refused without
  side effect, and its contributions -- if any earlier press committed -- enter
  reconciliation debt.
- *Request wins:* the request takes the lock first, commits, and is processed;
  revocation then applies and retires the now-committed contributions through
  the ordinary settlement path. A committed effect is not undone.

**Per-client cancellation does not advance the seat epoch.** Disconnect or
single-grant revocation is its own ingress into the serializing operation,
retiring only that client's sources and debt. It must not clear another source's
hold and must not cancel a target's grab. The seat-wide epoch advance remains
for seat-wide events.

**Protected actions, and why the obvious rule was dangerous.** The previous
revision said any synthetic contributor disqualifies a protected action. That
hands an injector a **veto over the operator's recovery path**: an injector
holding Ctrl would disqualify a physically complete Ctrl+Alt+Backspace even
while the operator physically holds every key of it. That is exactly backwards
-- the emergency chord exists for when something has gone wrong, including an
injector misbehaving.

The rule is therefore about sufficiency, not contamination:

- a protected action requires the necessary state to be held **by physical
  sources**, and to be triggered by a **physical** transition;
- synthetic contributions **cannot supply a missing component** of such a chord;
- synthetic contributions **cannot taint** an independently sufficient physical
  chord.

This needs the physical recognizer fed independently of the aggregate. If
synthetic already holds Ctrl and the operator then physically presses Ctrl, the
*aggregate* does not change and the ledger delivers nothing -- but the physical
source transition must still reach the emergency and VT recognizers, or the
overlap silently disables recovery. Physical transitions are reported to those
recognizers whether or not they change the aggregate.

This grants synthetic input nothing new; it prevents it removing something.

**Lock and seat state.** No existing lock path is asserted. What exists is seat
release in `owner_loop/lifecycle.rs` and the session control surface. The
proposed integration is an explicit predicate consulted before commit, reading
seat-active and session-lock state from those, **failing closed** when either is
unavailable or indeterminate rather than permitting.

## Scheduling: provisional values, to be measured

Numbers are starting points to instrument, not capacity claims. They are stated
because the previous revision removed them and left nothing to verify.

- **Insertion point.** Synthetic work is taken where authority batches are
  taken in `owner_loop/authority.rs`, after physical input and after revocation
  and cleanup, which are serviced unconditionally first.
- **Replenishment: 2ms of synthetic service per 16ms interval.** An interval,
  not a turn, because turns are work-driven and not uniform. Unused budget does
  **not** carry over; carry-over would let an idle period fund a burst that
  displaces physical input.
- **Cap: 32 synthetic events per interval**, whichever of cap or budget is
  reached first.
- **Round-robin cursor persists across stops**, so a source that missed an
  interval is served first in the next.
- **Progress expectation: N = 16 intervals.** Every ready source is serviced
  within 16 intervals under the 16-injector bound, which is the property to
  test. It is not a latency promise: a single non-preemptible operation may
  overrun its budget, and no seat is guaranteed to drain in any interval.

## Capacity accounting

| Population | Bound |
| --- | --- |
| Active grants | 16 per seat |
| **Retiring grants with unsettled debt** | 16 per seat, independently bounded |
| Pending requests | 1 per active grant; seat total 16 is *derived* |
| Ledger entries | ≤ keycodes + buttons per seat, fixed by the input domain |
| Sources per entry | ≤ physical devices + 16 |
| Debt entries | ≤ ledger entries × (devices + 16), storage reserved before a press is accepted |
| Connection buffer, FDs | existing bounded receive and arity caps, unchanged |
| Seats | per-seat bounds × seats, with a Session-wide ceiling when >1 seat exists |

**Retiring grants are bounded separately, and this is the gap the review
found.** Releasing a slot at revocation while settlement continues lets
repeated grant/revoke cycles accumulate old grant identities and debt behind the
16 live slots. So a slot is **occupied until settlement completes**, or -- if
that proves too coarse -- a separately bounded retiring population applies
admission backpressure. Either way the total is bounded; the previous revision's
accounting was not.

**Cleanup is sized against accumulated held state**, not pending requests: a
retiring injector may hold many inputs and no pending request. The sweep is
resumable across intervals and draws on reserved capacity.

**The seventeenth connection stays healthy.** It is an ordinary X client: it
connects, is admitted, and works. Discovery reports XTEST **absent** to it,
because it holds no grant, and absence is consistent with every other
unauthorized caller. `BadAccess` is the answer to a guessed opcode from a caller
with no grant; `BadAlloc` is the answer to a caller that *would* qualify but for
which no slot is free. The two are distinguishable and mean different things.

## Discovery and refusal, restored

These were in the first revision and dropped from the second; they belong here.

**One discovery function.** `advertised_extension_names()` and
`extension_query_result` both read the connection's grant through a single
helper, so the two enumeration paths cannot disagree for a given authorization
state -- the property t086 repaired by hand. They agree *for a state*, not
across an intervening revocation: a client that enumerates, is revoked, then
queries correctly sees different answers, and that is not a disagreement.

**Refusal record.** `kind` (absent | unauthorized | revoked | capacity |
protected-action | malformed | unsettled-debt), `client`, `grant generation`,
`security epoch`, `seat`, `request`, `decision`. Intentional denial stays
distinguishable from missing implementation, which the wire alone does not
settle.

## The advertised bundle

**GetVersion** returns major 2, minor 2 -- the version whose semantics this
design implements -- wherever XTEST is advertised. A client asking for more is
answered with what is supported, not refused.

**CompareCursor** takes a window and a cursor. The **window is namespace-checked
in every case**, including `None` and `CurrentCursor`; the previous revision
checked it only for an explicit cursor, which would have let an unauthorized
window be named so long as the cursor argument was special. `None` compares
against no cursor. `CurrentCursor` reads what the seat displays and is the
bundle's one pure-read disclosure: permitted for the administrator bundle,
refused otherwise.

**GrabControl** is scheduling behaviour. An impervious client progresses while
another client holds a server grab; an ordinary client waits at `:592`; both
remain cancellable there. Imperviousness retires on revocation and disconnect,
returning the client to ordinary waiting.

**FakeInput** distinguishes malformed from unusual:

- **Off-screen motion is valid** and is clipped to the nearest on-screen
  coordinate, per the protocol. The previous revision's blanket "no clamp" was
  wrong: clamping is the specified behaviour here, and only *fields* are
  rejected rather than refused-and-clamped.
- Malformed is an out-of-range event type, a keycode outside the server's
  min/max range, or a button outside the pointer mapping -- these are errors.
- Keycodes convert through the server's keycode range; buttons through the
  current pointer mapping, so a remapped pointer injects what the user's mapping
  means.
- Motion is absolute against the named root by default, relative when requested;
  a root that does not exist is an error, not a silent substitution.
- Sequence completion follows the delayed lifecycle: the reply or error belongs
  to the connection thread and is emitted when the request is processed.
- A guessed opcode from an unauthorized caller is refused exactly as an
  authorized-but-denied one, since hiding is not a boundary.

## Admission rechecks

The grant is not only issued and trusted. At execution, under the serializing
lock: the peer is rechecked as current, protected-role exclusion is re-applied
so a protected role cannot acquire a grant through namespace or resource
admission, and lock and seat state are read from their owning components. Any
of those being unavailable or indeterminate **fails closed**.

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
- failed reconciling delivery holds bounded debt until settlement is
  acknowledged, and never retargets -- replacing the earlier case, which
  accepted a forgotten failure;
- a fully physical emergency chord still triggers while an injector overlaps one
  of its components;
- a chord needing a synthetic component does not trigger;
- a physical press that does not change the aggregate still reaches the physical
  recognizers;
- repeat started by A, with B still holding, cancels on A's retirement rather
  than transferring to B;
- `BadAlloc` at capacity leaves the caller's existing holds intact;
- write-half-close with pending work completes rather than aborting;
- paused ingress with unread pipelined bytes does not spin;
- grant/revoke cycled repeatedly does not accumulate grants or debt past the
  bound;
- revocation landing on a delayed request at the boundary;
- cleanup of many held inputs while the ordinary path is saturated;
- duplicate press and stale release;
- EOF observed during a delay behind unread pipelined bytes.

## Out of scope

Virtual seat, selective delegation, configurable permission subsets, and any
route by which synthetic input could close a physical acceptance obligation.
