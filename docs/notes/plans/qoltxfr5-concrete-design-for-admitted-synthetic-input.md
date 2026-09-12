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
gone.

**Settlement is a server-side fact, never an application acknowledgement.** Core
key and button events carry no application acknowledgement, and a socket write
proves only that bytes left, not that a client processed them. So settlement is
one of these existing receipts, each settling a different layer:

| Receipt | Settles |
| --- | --- |
| `RoutedInputOutcome::Accepted` for the release | the release reached the authoritative recipient's route |
| `RoutedInputOutcome::RejectedStaleTarget` / `RejectedDeniedNamespace` | the recorded target no longer exists as such -- nothing survives to clear |
| `XAuthorityRouteLeaseRelease` for the recorded lease | that lease's held state is gone with it |
| client termination or recovery through `input_recovery` | the recipient itself is gone |

Debt therefore persists after a **failed enqueue** (never routed), a **stale
identity** that did not resolve to one of the rejection outcomes above, and a
**blocked delivery** that has not yet produced an outcome. It does not persist
once any row above applies.

Debt carries the recorded target *and* a hold identity, so a late completion for
an old debt cannot erase a newer hold on the same input: the identities differ
and the stale completion is discarded.

**Unresponsive recipients end, they are not waited on forever.** A debt whose
recipient produces no outcome within a bounded number of service intervals
escalates to recipient termination through the existing disconnect path, which
then settles it by the last row. Logging alone is not settlement; neither is
waiting for an acknowledgement the protocol does not define.

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
perform.

**Half-close is latched, because `POLLRDHUP` stays set.** Once observed it would
wake the poll forever, so the write-half-close is recorded once and its wake
interest removed while ingress is paused. `POLLHUP`, `POLLERR` and the revocation
notifier remain armed. An independent check confirms the hazard: two consecutive
zero-timeout polls both returned `POLLRDHUP` while `MSG_PEEK` still showed a
buffered request.

**Half-close does not discard what the peer already sent.** A peer may send a
delayed `FakeInput`, then `GetInputFocus`, then `shutdown(SHUT_WR)`. Finishing
only the `FakeInput` and closing would drop a valid request the peer is entitled
to have answered. So after the delayed request is processed, ingress resumes and
parses the buffered requests in order, bounded as ordinarily, until a read
actually returns EOF. Delays encountered among them are honoured, and errors are
reported normally. Full departure -- `POLLHUP` or `POLLERR` -- still cancels
immediately.

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

**The serializing operation is the runtime mutex that already exists.** A new
Session lock would invert lock order against `state.runtime` and deadlock the
first time a connection held one and wanted the other. So there is no new lock:

| Participant | Entry point | Role |
| --- | --- | --- |
| Connection dispatch | `lock_x11_request_runtime` (`writers.rs:762`) | grabs, input authority, and now grant validity and the ledger |
| Grab writers | `dispatch/core/grabs.rs` | already take that lock |
| Session control | `control_runtime_pending` priority path | already preempts request work; revocation uses it |
| Broker routing | `route_pending(&mut self)` (`broker.rs:496`) | **not** a lock participant -- single-threaded consumer |

Grant validity, the ledger and revocation become state guarded by that same
mutex. Lock order is unchanged because there is one lock, and revocation reaches
it through the existing control-priority path rather than racing request work
for it.

**The enqueue gap is closed by carrying the epoch, not by holding the lock.**
The review is right that releasing the mutex after enqueue leaves a gap, and
that holding it while waiting for the broker -- which needs its own `&mut self`
turn -- deadlocks. Neither is chosen. Instead:

- under the lock: revalidate, mutate the ledger, resolve focus and grabs, and
  enqueue the routed input **stamped with the grant's bound epoch**;
- release the lock;
- `route_pending` already compares `input_control_epoch` against
  `applied_input_control_epoch` and applies an advance before routing
  (`broker.rs:497-501`). A request enqueued before a revocation is therefore
  validated against the epoch it carries, at the consumer, and is dropped there
  if the epoch moved.

So the gap is safe because the consumer re-checks, which is the mechanism the
broker already implements for leases. Nothing waits on a worker while holding a
lock that worker needs.

**Revised race traces.**

- *Revocation first:* revocation takes the mutex via the control path,
  invalidates the generation and advances the control epoch. A request still
  queued at the connection fails revalidation and is refused with no side
  effect. A request already enqueued to the broker carries the old epoch and is
  dropped at `route_pending`. Contributions committed before revocation enter
  reconciliation debt.
- *Request first:* the request takes the mutex, commits, and is enqueued with
  the current epoch. Revocation then applies; the enqueued item still routes,
  because it was committed under the epoch in force, and its contributions are
  retired afterwards through settlement. **"Processed" is the broker reporting
  `RoutedInputOutcome` for that item**, not the enqueue.

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
- **Progress expectation: N = 16 intervals, conditional.** Given service
  opportunities actually occurring and runnable work, every ready source is
  serviced within 16 intervals under the 16-slot bound. It is not a bound under
  arbitrary owner-loop stalls, a held server grab, or a non-preemptible
  operation overrunning its budget. That conditional form is what the tests
  assert.

## Capacity accounting

**One slot policy, chosen.** Sixteen grant slots per seat, **total**, counting
grants still carrying unsettled debt. A slot is reusable only once its debt is
settled. The previous revision offered two alternatives; the alternative is
removed, and the formulas below follow from this one.

| Population | Bound |
| --- | --- |
| Grant slots per seat | 16 total, active plus retiring-with-debt |
| Pending requests | 1 per active grant; seat total is *derived*, never more than 16 |
| Ledger entries | ≤ keycodes + buttons per seat, fixed by the input domain |
| Sources per entry | ≤ physical devices + 16, matching the slot total |
| Debt entries | ≤ ledger entries × (devices + 16); storage reserved before a press is accepted |
| Connection buffer, FDs | existing bounded receive and arity caps, unchanged |
| Seats | **this design covers the single exposed seat only** |

Multi-seat is out of scope rather than hand-waved: a Session-wide ceiling would
need a seat inventory this design does not specify, so a second exposed seat
requires a revision.

**Cleanup has a reserved service allowance**, not merely reserved storage: a
fixed share of each service interval is spent on settlement before ordinary
synthetic work, so debt drains even while injectors are saturating the path.
That is what makes the resumable sweep terminate.

### Decision order for a grant-requiring request

Evaluated at the **first grant-requiring request** on a connection, in this
order, first match deciding:

| Condition | Answer |
| --- | --- |
| Option disabled | XTEST absent; guessed opcode gets `BadRequest` |
| Caller ineligible | XTEST absent; guessed opcode gets `BadAccess` |
| Grant revoked | `BadAccess` |
| Eligible, no free slot | `BadAlloc` |
| Eligible, slot available | grant issued, request proceeds |

Discovery reports XTEST only in the last row. An eligible seventeenth client is
therefore an ordinary, healthy X client that sees XTEST absent and receives
`BadAlloc` only if it guesses the opcode. It may retry: a later request
re-evaluates, and a slot freed by settlement is available to it.

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
in every case**, including `None` and `CurrentCursor`, and an **explicit cursor
argument is itself resolved through the caller's namespace** like any other
resource lookup; the previous revision
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
- Motion is absolute against the named root by default, relative when requested.
  **Root `None` is valid** and selects the pointer's current screen -- the
  previous revision called it an error. A root that is neither `None` nor an
  existing root is an error.
- **FakeInput produces no wire reply on success.** Its completion is internal:
  it releases the connection to read again, and later requests keep ordinary
  sequencing. The previous revision implied a reply.
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
- EOF observed during a delay behind unread pipelined bytes;
- half-close with a pipelined request behind the delayed one: the buffered
  request is parsed and answered after processing, not discarded;
- half-close does not spin: repeated polls do not rewake on a latched
  `POLLRDHUP`;
- revocation, and a peer grab or focus change, landing exactly in the gap
  between enqueue and `route_pending`;
- a late debt completion carrying an old hold identity does not erase a newer
  hold on the same input;
- an unresponsive recipient escalates to termination within its bound rather
  than holding debt open;
- a slot is not reusable until its debt settles, and becomes reusable after;
- the ordered decision table: disabled, ineligible, revoked, capacity and
  granted each produce their own answer.

## Out of scope

Virtual seat, selective delegation, configurable permission subsets, and any
route by which synthetic input could close a physical acceptance obligation.
