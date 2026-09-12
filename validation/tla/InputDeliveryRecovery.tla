-------------------- MODULE InputDeliveryRecovery --------------------
EXTENDS Naturals, FiniteSets

CONSTANTS Watchdog, EarlyBarrier, Budget, LastTick
Clients == {1, 2}
Ids == {1, 2}
None == 0
States == {"unused", "ingress", "bound", "writing", "returned"}
Outcomes == {"none", "flushed", "failed", "revoked"}

VARIABLES phase, client, admittedAt, outcome, written, pending, barrier,
          revoked, settled, now, seat, vtAt, control, unsafeDispatch
vars == <<phase, client, admittedAt, outcome, written, pending, barrier,
          revoked, settled, now, seat, vtAt, control, unsafeDispatch>>

Init ==
    /\ phase = [id \in Ids |-> "unused"]
    /\ client = [id \in Ids |-> None]
    /\ admittedAt = [id \in Ids |-> 0]
    /\ outcome = [id \in Ids |-> "none"]
    /\ written = {} /\ pending = {} /\ barrier = {}
    /\ revoked = {} /\ settled = [id \in Ids |-> 0]
    /\ now = 0 /\ seat = "active" /\ vtAt = 0
    /\ control = "held" /\ unsafeDispatch = FALSE

\* crates/sophia-x-authority/src/x11_socket/routing/recovery.rs:64 InputRecovery::admit (register-before-enqueue);
\* input_delivery.rs InputDeliveryState::track. Owner cannot run its watchdog
\* between send and track; this action ends at that owner service boundary.
Admit(id) ==
    /\ phase[id] = "unused" /\ seat = "active" /\ now <= 1
    /\ phase' = [phase EXCEPT ![id] = "ingress"]
    /\ admittedAt' = [admittedAt EXCEPT ![id] = now]
    /\ pending' = pending \cup {id}
    /\ barrier' = IF id = 1 THEN barrier \cup {id} ELSE barrier
    /\ UNCHANGED <<client, outcome, written, revoked, settled, now, seat, vtAt,
                    control, unsafeDispatch>>

\* crates/sophia-x-authority/src/x11_socket/routing/recovery.rs:130 bind, after registry/delivery.rs resolves an X grab. A client
\* is chosen here, independently of the original target owner.
Bind(id, receiver) ==
    /\ phase[id] = "ingress" /\ outcome[id] = "none"
    /\ receiver \notin revoked
    /\ phase' = [phase EXCEPT ![id] = "bound"]
    /\ client' = [client EXCEPT ![id] = receiver]
    /\ UNCHANGED <<admittedAt, outcome, written, pending, barrier, revoked,
                    settled, now, seat, vtAt, control, unsafeDispatch>>

\* crates/sophia-x-authority/src/x11_socket/connection/writers/input.rs:67 checks delivery_active before writing, without retaining
\* the recovery mutex across I/O. Writing may never complete.
StartWrite(id) ==
    /\ phase[id] = "bound" /\ outcome[id] = "none"
    /\ client[id] \notin revoked
    /\ phase' = [phase EXCEPT ![id] = "writing"]
    /\ UNCHANGED <<client, admittedAt, outcome, written, pending, barrier,
                    revoked, settled, now, seat, vtAt, control, unsafeDispatch>>

\* crates/sophia-x-authority/src/x11_socket/connection/writers/input.rs:738 write_all + flush returns. A separate finish action contends with recovery
\* for the ledger lock; successful writing does not force a Flushed receipt.
WriteReturns(id) ==
    /\ phase[id] = "writing" /\ client[id] \notin revoked
    /\ phase' = [phase EXCEPT ![id] = "returned"]
    /\ written' = written \cup {id}
    /\ UNCHANGED <<client, admittedAt, outcome, pending, barrier, revoked,
                    settled, now, seat, vtAt, control, unsafeDispatch>>

\* crates/sophia-x-authority/src/x11_socket/routing/recovery.rs:208 finish/terminal_locked: first terminal result wins.
Finish(id) ==
    /\ phase[id] = "returned" /\ outcome[id] = "none"
    /\ client[id] \notin revoked
    /\ outcome' = [outcome EXCEPT ![id] = "flushed"]
    /\ settled' = [settled EXCEPT ![id] = @ + 1]
    /\ UNCHANGED <<phase, client, admittedAt, written, pending, barrier,
                    revoked, now, seat, vtAt, control, unsafeDispatch>>

Due(id) == Watchdog /\ outcome[id] = "none" /\ phase[id] # "unused"
    /\ (now >= admittedAt[id] + Budget \/ (seat = "waiting" /\ now > vtAt))

\* crates/sophia-x-authority/src/x11_socket/routing/recovery.rs:366 recover: an unresolved route is cancelled, with a tombstone
\* until its queued entry is consumed. No fictitious client is disconnected.
CancelUnresolved(id) ==
    /\ Due(id) /\ client[id] = None
    /\ outcome' = [outcome EXCEPT ![id] = "revoked"]
    /\ settled' = [settled EXCEPT ![id] = @ + 1]
    /\ UNCHANGED <<phase, client, admittedAt, written, pending, barrier,
                    revoked, now, seat, vtAt, control, unsafeDispatch>>

\* crates/sophia-x-authority/src/x11_socket/routing/recovery.rs:310 disconnect_locked: revoke routing, shutdown a separate socket
\* clone, THEN publish failure receipts under the same ledger lock. The
\* syscall is assumed to finish; no fairness of the blocked writer is needed.
Disconnect(receiver) ==
    /\ receiver \notin revoked
    /\ \E id \in Ids : Due(id) /\ client[id] = receiver
    /\ revoked' = revoked \cup {receiver}
    /\ outcome' = [id \in Ids |-> IF client[id] = receiver /\ outcome[id] = "none"
                        THEN "failed" ELSE outcome[id]]
    /\ settled' = [id \in Ids |-> IF client[id] = receiver /\ outcome[id] = "none"
                        THEN settled[id] + 1 ELSE settled[id]]
    /\ UNCHANGED <<phase, client, admittedAt, written, pending, barrier,
                    now, seat, vtAt, control, unsafeDispatch>>

Recover == (\E id \in Ids : CancelUnresolved(id)) \/ (\E c \in Clients : Disconnect(c))

\* crates/sophia-session/src/live_session/owner_loop_state.rs:97 drain: authenticated receipt observation precedes
\* barrier removal. Duplicate, wrong-client and late receipts are stuttering.
Observe(id) ==
    /\ id \in pending /\ outcome[id] # "none"
    /\ pending' = pending \ {id} /\ barrier' = barrier \ {id}
    /\ UNCHANGED <<phase, client, admittedAt, outcome, written, revoked,
                    settled, now, seat, vtAt, control, unsafeDispatch>>

\* crates/sophia-session/src/session_control.rs:211 service_when: ordinary queue/ACK timers only start
\* after the release barrier. In this model client 2 owns the waiting control.
DispatchControl ==
    /\ control = "held" /\ phase[1] # "unused" /\ barrier = {}
    /\ 2 \notin revoked
    /\ control' = "dispatched"
    /\ unsafeDispatch' = (outcome[1] = "none")
    /\ UNCHANGED <<phase, client, admittedAt, outcome, written, pending,
                    barrier, revoked, settled, now, seat, vtAt>>

\* crates/sophia-session/src/live_session/owner_loop/lifecycle.rs:8 pending VT: stop new admissions, allow the bounded grace,
\* then use the same cancellation/disconnection path for ALL pending input.
RequestVt ==
    /\ seat = "active" /\ phase[1] # "unused" /\ now <= LastTick - 1
    /\ seat' = "waiting" /\ vtAt' = now
    /\ UNCHANGED <<phase, client, admittedAt, outcome, written, pending,
                    barrier, revoked, settled, now, control, unsafeDispatch>>
Handoff ==
    /\ seat = "waiting" /\ pending = {}
    /\ seat' = "suspended"
    /\ UNCHANGED <<phase, client, admittedAt, outcome, written, pending,
                    barrier, revoked, settled, now, vtAt, control, unsafeDispatch>>

\* A finite clock abstraction in half-second ticks: Budget=12 is six seconds;
\* VT grace is one tick. All admissions occur by tick 1; LastTick=14 outlives
\* their deadlines. Saturation stutters rather than wrapping the clock.
Tick ==
    /\ now < LastTick /\ now' = now + 1
    /\ UNCHANGED <<phase, client, admittedAt, outcome, written, pending,
                    barrier, revoked, settled, seat, vtAt, control, unsafeDispatch>>

\* Negative control for settling the barrier before revoking deliverability.
DropBarrierEarly ==
    /\ EarlyBarrier /\ barrier # {} /\ barrier' = {}
    /\ UNCHANGED <<phase, client, admittedAt, outcome, written, pending,
                    revoked, settled, now, seat, vtAt, control, unsafeDispatch>>

Next == (\E id \in Ids : Admit(id) \/ StartWrite(id) \/ WriteReturns(id) \/ Finish(id)
                        \/ Observe(id) \/ (\E c \in Clients : Bind(id, c)))
        \/ Recover \/ DispatchControl \/ RequestVt \/ Handoff \/ Tick \/ DropBarrierEarly

Spec == Init /\ [][Next]_vars /\ WF_vars(Tick) /\ WF_vars(Recover)
        /\ (\A id \in Ids : WF_vars(Observe(id)))
        /\ WF_vars(DispatchControl) /\ WF_vars(Handoff)

TypeOK == /\ phase \in [Ids -> States] /\ client \in [Ids -> (Clients \cup {None})]
          /\ outcome \in [Ids -> Outcomes] /\ pending \subseteq Ids /\ barrier \subseteq Ids
          /\ revoked \subseteq Clients /\ written \subseteq Ids
          /\ settled \in [Ids -> 0..1] /\ now \in 0..LastTick
          /\ seat \in {"active", "waiting", "suspended"}
NoFalseFlush == \A id \in Ids : outcome[id] = "flushed" => id \in written
ExactlyOneTerminal == \A id \in Ids : settled[id] = IF outcome[id] = "none" THEN 0 ELSE 1
BarrierSound == phase[1] # "unused" /\ outcome[1] = "none" => 1 \in barrier
NoPrematureControl == ~unsafeDispatch
NoAcceptedObligationLost == \A id \in Ids : phase[id] # "unused" /\ outcome[id] = "none" => id \in pending
NoOldSeatDebt == seat = "suspended" => pending = {}
DeliveryProgress == \A id \in Ids : (id \in pending) ~> (id \notin pending)
HealthyControlProgress == (control = "held" /\ phase[1] # "unused") ~> (control = "dispatched" \/ 2 \in revoked)
SeatProgress == (seat = "waiting") ~> (seat = "suspended")
=======================================================================
