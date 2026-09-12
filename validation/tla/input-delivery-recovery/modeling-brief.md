# t077 input delivery recovery

Category B: concurrent owner, frontend router and per-client socket writers.
The issue is an accepted release remaining pending forever and holding focus,
close and seat handoff; it does not establish why pinentry's GUI stalls.

Scenarios:
1. A bound input writer blocks or exits without a receipt. Its original
   six-second deadline must revoke that exact receiving connection, then settle
   failures and release the owner barrier. An X grab can change the receiver.
2. Expiry races connection resolution or successful write completion. An
   unresolved cancelled ingress/frozen entry cannot reappear; receipts match
   exact connection plus monotonic delivery ID, and first terminal outcome wins.
3. A VT handoff with key-release and unrelated pointer obligations stops new
   old-seat admission and after 500 ms cancels unresolved routes/disconnects bound
   recipients. It cannot hand off while those deliveries remain live.
4. Releasing a barrier on time alone permits controls ahead of key release.
   Disabling the watchdog reproduces the original indefinite hold.

Safety: NoFalseFlush, ExactlyOneTerminal, BarrierSound, NoPrematureControl,
NoAcceptedObligationLost and NoOldSeatDebt. Liveness: DeliveryProgress,
HealthyControlProgress and SeatProgress under weak fairness of time, owner
recovery/receipt service, controls and seat transition. No responsive-client or
writer-completion assumption. Shared-lock corruption and a failing shutdown
syscall are explicit fatal errors in code, outside this successful-revocation
model. The model does not infer client receipt from kernel flush, model secret
input contents, or claim the pinentry GUI defect is repaired.

Finite configuration: two immutable delivery identities and two stable frontend
connection IDs. One delivery belongs to the control release barrier; the other
covers ordinary input. Admission occurs by half-second tick 1 and time saturates
at tick 14, beyond both six-second deadlines. The watchdog captures the original
admission time. Unresolved queue-credit tombstones and queue capacity have direct
Rust coverage; model `pending` describes the owner's outstanding obligations,
not the frontend's separate retained cancellation tombstones.
