----------------------- MODULE PointerGrabAdmission -----------------------
EXTENDS Naturals, Sequences, FiniteSets

(***************************************************************************
 * t066 design model: bridge publication, owner-loop accounting, and lease   *
 * readiness are separate actions. This is a bounded contract model, not a  *
 * claim that the current Rust implementation has already been verified.     *
 * Source correspondence and scenario coverage: PointerGrabAdmission.md.    *
 *************************************************************************)
CONSTANTS PrepareTicks, BindingTicks, MaxTime, MaxHeld,
          IgnorePrerequisite, ReviveCancelled

ASSUME /\ PrepareTicks > 0 /\ BindingTicks > PrepareTicks
       /\ MaxTime >= BindingTicks + PrepareTicks /\ MaxHeld > 0
       /\ IgnorePrerequisite \in BOOLEAN /\ ReviveCancelled \in BOOLEAN

(* Transaction ids have gaps; only these actually publish observations. *)
Observations == <<2, 5>>
Outputs == {"left", "right"}
NoLease == [phase |-> "none", binding |-> "waiting", output |-> "none",
            created |-> 0, deadline |-> 0, required |-> 0]

VARIABLES published, dequeued, applied, mapApplied, request, lease,
          held, firstHeld, ackPending, cancelled, now, targetOutput,
          contextLive, scopeInside, grants, deliveries

vars == <<published, dequeued, applied, mapApplied, request, lease,
          held, firstHeld, ackPending, cancelled, now, targetOutput,
          contextLive, scopeInside, grants, deliveries>>

Init ==
    /\ published = 0 /\ dequeued = 0 /\ applied = 0 /\ mapApplied = FALSE
    /\ request = [state |-> "none", required |-> 0, deadline |-> 0]
    /\ lease = NoLease /\ held = <<>> /\ firstHeld = 0
    /\ ackPending = FALSE /\ cancelled = FALSE /\ now = 0
    /\ targetOutput = "none" /\ contextLive = TRUE /\ scopeInside = TRUE
    /\ grants = <<>> /\ deliveries = <<>>

Receipt(index) == IF index = 0 THEN 0 ELSE Observations[index]
PrerequisiteApplied == Receipt(applied) >= request.required

(* transport publication: receipt changes only after successful enqueue. *)
PublishObservation ==
    /\ published < Len(Observations)
    /\ published' = published + 1
    /\ UNCHANGED <<dequeued, applied, mapApplied, request, lease, held,
         firstHeld, ackPending, cancelled, now, targetOutput, contextLive,
         scopeInside, grants, deliveries>>

(* owner_loop/authority.rs intake is not the application linearization. *)
DequeueObservation ==
    /\ dequeued < published /\ dequeued' = dequeued + 1
    /\ UNCHANGED <<published, applied, mapApplied, request, lease, held,
         firstHeld, ackPending, cancelled, now, targetOutput, contextLive,
         scopeInside, grants, deliveries>>

(* Map effects apply before accounting. Observation 5 has no Engine work. *)
AccountObservation ==
    /\ applied < dequeued /\ applied' = applied + 1
    /\ mapApplied' = TRUE
    /\ UNCHANGED <<published, dequeued, request, lease, held, firstHeld,
         ackPending, cancelled, now, targetOutput, contextLive, scopeInside,
         grants, deliveries>>

(* frontend/explicit_pointer_grab.rs: deadline and prerequisite fixed once. *)
RequestPrepare ==
    /\ request.state = "none" /\ published > 0
    /\ request' = [state |-> "pending", required |-> Receipt(published),
                   deadline |-> now + PrepareTicks]
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, lease, held,
         firstHeld, ackPending, cancelled, now, targetOutput, contextLive,
         scopeInside, grants, deliveries>>

(* Owner accepts authority facts, never waits for targetOutput/presentation. *)
Reserve ==
    /\ request.state = "pending" /\ now < request.deadline
    /\ ~cancelled /\ contextLive /\ mapApplied
    /\ (IgnorePrerequisite \/ PrerequisiteApplied)
    /\ lease' = [phase |-> "provisional", binding |-> "waiting",
                  output |-> "none", created |-> now,
                  deadline |-> now + BindingTicks, required |-> request.required]
    /\ request' = [request EXCEPT !.state = "reserved"]
    /\ ackPending' = TRUE
    /\ grants' = Append(grants,
          [required |-> request.required, applied |-> Receipt(applied)])
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, held, firstHeld,
         cancelled, now, targetOutput, contextLive, scopeInside, deliveries>>

(* A response can race cancellation after reservation but before receipt. *)
ReceivePrepared ==
    /\ ackPending
    /\ ackPending' = FALSE
    /\ IF cancelled
          THEN /\ request' = request
               /\ lease' = IF ReviveCancelled
                    THEN [NoLease EXCEPT !.phase = "active"] ELSE lease
          ELSE /\ request' = [request EXCEPT !.state = "accepted"]
               /\ lease' = lease
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, held, firstHeld,
         cancelled, now, targetOutput, contextLive, scopeInside, grants, deliveries>>

(* Activation and presentation are independent; success permits drawing. *)
Activate ==
    /\ request.state = "accepted" /\ lease.phase = "provisional"
    /\ ~cancelled /\ contextLive /\ now < lease.deadline
    /\ lease' = [lease EXCEPT !.phase = "active"]
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, request, held,
         firstHeld, ackPending, cancelled, now, targetOutput, contextLive,
         scopeInside, grants, deliveries>>

Present(output) ==
    /\ lease.phase = "active" /\ targetOutput = "none"
    /\ contextLive /\ targetOutput' = output
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, request, lease,
         held, firstHeld, ackPending, cancelled, now, contextLive, scopeInside,
         grants, deliveries>>

(* shared input wiring: phase may still be provisional while events arrive. *)
Hold(output) ==
    /\ lease.phase \in {"provisional", "active"}
    /\ lease.binding = "waiting" /\ ~cancelled /\ contextLive /\ scopeInside
    /\ now < lease.deadline /\ Len(held) < MaxHeld
    /\ lease.output \in {"none", output}
    /\ lease' = [lease EXCEPT !.output = output]
    /\ held' = Append(held, [output |-> output, at |-> now])
    /\ firstHeld' = IF Len(held) = 0 THEN now ELSE firstHeld
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, request,
         ackPending, cancelled, now, targetOutput, contextLive, scopeInside,
         grants, deliveries>>

Bind ==
    /\ lease.phase \in {"provisional", "active"}
    /\ lease.binding = "waiting" /\ lease.output \in Outputs
    /\ lease.output = targetOutput /\ now < lease.deadline
    /\ ~cancelled /\ contextLive /\ scopeInside
    /\ lease' = [lease EXCEPT !.binding = "bound"]
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, request, held,
         firstHeld, ackPending, cancelled, now, targetOutput, contextLive,
         scopeInside, grants, deliveries>>

Deliver ==
    /\ lease.phase = "active" /\ lease.binding = "bound"
    /\ Len(held) > 0 /\ now < lease.deadline
    /\ contextLive /\ scopeInside /\ ~cancelled
    /\ lease.output = targetOutput /\ Head(held).output = targetOutput
    /\ deliveries' = Append(deliveries,
          [phase |-> lease.phase, binding |-> lease.binding,
           required |-> lease.required, applied |-> Receipt(applied),
           live |-> contextLive, inside |-> scopeInside,
           cancelled |-> cancelled, output |-> lease.output,
           presented |-> targetOutput])
    /\ held' = Tail(held)
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, request, lease,
         firstHeld, ackPending, cancelled, now, targetOutput, contextLive,
         scopeInside, grants>>

(* One cleanup body, shared by explicit cancellation, expiry and invalidation. *)
ClearOwnership ==
    /\ request' = [request EXCEPT !.state = "cancelled"]
    /\ lease' = NoLease /\ held' = <<>> /\ cancelled' = TRUE

Cancel ==
    /\ request.state \in {"pending", "reserved", "accepted"}
    /\ ClearOwnership
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, firstHeld,
         ackPending, now, targetOutput, contextLive, scopeInside, grants, deliveries>>

Invalidate ==
    /\ contextLive /\ contextLive' = FALSE
    /\ ClearOwnership
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, firstHeld,
         ackPending, now, targetOutput, scopeInside, grants, deliveries>>

LeaveScope ==
    /\ scopeInside /\ scopeInside' = FALSE
    /\ ClearOwnership
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, firstHeld,
         ackPending, now, targetOutput, contextLive, grants, deliveries>>

Overflow ==
    /\ lease.phase # "none" /\ Len(held) = MaxHeld
    /\ ClearOwnership
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, firstHeld,
         ackPending, now, targetOutput, contextLive, scopeInside, grants, deliveries>>

Tick ==
    /\ now < MaxTime /\ now' = now + 1
    /\ IF (request.state \in {"pending", "reserved"}
               /\ now + 1 >= request.deadline)
           \/ (lease.phase # "none" /\ lease.binding = "waiting"
               /\ now + 1 >= lease.deadline)
           \/ (Len(held) > 0 /\ (now + 1 >= lease.deadline
                                  \/ now + 1 >= firstHeld + BindingTicks))
          THEN ClearOwnership
          ELSE UNCHANGED <<request, lease, held, cancelled>>
    /\ UNCHANGED <<published, dequeued, applied, mapApplied, firstHeld,
         ackPending, targetOutput, contextLive, scopeInside, grants, deliveries>>

Next == PublishObservation \/ DequeueObservation \/ AccountObservation
        \/ RequestPrepare \/ Reserve \/ ReceivePrepared \/ Activate
        \/ (\E output \in Outputs : Present(output) \/ Hold(output))
        \/ Bind \/ Deliver \/ Cancel \/ Invalidate \/ LeaveScope
        \/ Overflow \/ Tick
Spec == Init /\ [][Next]_vars

TypeOK ==
    /\ published \in 0..2 /\ dequeued \in 0..2 /\ applied \in 0..2
    /\ mapApplied \in BOOLEAN /\ now \in 0..MaxTime
    /\ request.state \in {"none", "pending", "reserved", "accepted", "cancelled"}
    /\ request.required \in {0, 2, 5}
    /\ lease.phase \in {"none", "provisional", "active"}
    /\ lease.binding \in {"waiting", "bound"}
    /\ lease.output \in Outputs \cup {"none"}
    /\ targetOutput \in Outputs \cup {"none"}
    /\ ackPending \in BOOLEAN /\ cancelled \in BOOLEAN
    /\ contextLive \in BOOLEAN /\ scopeInside \in BOOLEAN
    /\ Len(held) <= MaxHeld /\ Len(grants) <= 1

AccountedPrefix == applied <= dequeued /\ dequeued <= published
AppliedBeforeGrant ==
    \A index \in 1..Len(grants) : grants[index].required <= grants[index].applied
CancelledHasNoOwnership == cancelled => lease.phase = "none" /\ Len(held) = 0
UnboundLifetimeIsAbsolute ==
    lease.phase # "none" /\ lease.binding = "waiting" =>
        lease.deadline = lease.created + BindingTicks /\ now < lease.deadline
DeliveryRequiresCurrentEvidence ==
    \A index \in 1..Len(deliveries) :
        LET event == deliveries[index] IN
        /\ event.phase = "active" /\ event.binding = "bound"
        /\ event.live /\ event.inside /\ ~event.cancelled
        /\ event.output = event.presented /\ event.output \in Outputs
        /\ event.required <= event.applied
=============================================================================
