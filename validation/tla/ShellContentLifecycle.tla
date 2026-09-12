---------------------- MODULE ShellContentLifecycle ----------------------
EXTENDS Naturals, FiniteSets

(***************************************************************************
 * Residency and composition for the sophia_shell_v1 content capability.    *
 *                                                                          *
 * `ShellDescriptorLifecycle` already owns retention versus revocation, and *
 * `StableBackingLease` owns what a renderer copy holds while presentations *
 * still reference it. Neither bounds accepted resident bytes, because a    *
 * descriptor shell uploads no pixels. That is what this model adds, plus   *
 * the obligation a consumed pacing permit creates.                         *
 *                                                                          *
 * Bytes are abstracted to one unit per resource generation, so the caps    *
 * are cardinalities. The question is conservation across staging, resident *
 * and retiring, not arithmetic.                                            *
 *                                                                          *
 * Assumption, not a result: the renderer makes progress. A stalled GPU is  *
 * modelled by a reference that has not drained, never by storage freed to  *
 * satisfy a bound.                                                         *
 *************************************************************************)

CONSTANTS Resources, MaxGeneration, StagingCap, ResidentCap, RetiringCap,
          RetireRejectsAssembling, TimeoutEmitsOutcome

ASSUME /\ Resources \subseteq (Nat \ {0})
       /\ Resources # {}
       /\ MaxGeneration \in (Nat \ {0})
       /\ StagingCap \in (Nat \ {0})
       /\ ResidentCap \in (Nat \ {0})
       /\ RetiringCap \in (Nat \ {0})
       /\ RetireRejectsAssembling \in BOOLEAN
       /\ TimeoutEmitsOutcome \in BOOLEAN

Gens == 1..MaxGeneration
Keys == [res: Resources, gen: Gens]

VARIABLES
    granted, revoked, transfer, highWater,
    accepted, retiring, released,
    slot, candRefs, owed, presentedRefs, presentedLive

vars == <<granted, revoked, transfer, highWater,
          accepted, retiring, released,
          slot, candRefs, owed, presentedRefs, presentedLive>>

Staging == {r \in Resources : transfer[r] # 0}

\* Storage a candidate or the presented bundle can still reach.
Pinned ==
    presentedRefs \cup (IF slot \in {"pending", "submitted"} THEN candRefs ELSE {})

Init ==
    /\ granted = FALSE
    /\ revoked = FALSE
    /\ transfer = [r \in Resources |-> 0]
    /\ highWater = [r \in Resources |-> 0]
    /\ accepted = {}
    /\ retiring = {}
    /\ released = {}
    /\ slot = "free"
    /\ candRefs = {}
    /\ owed = FALSE
    /\ presentedRefs = {}
    /\ presentedLive = FALSE

Grant ==
    /\ ~granted
    /\ granted' = TRUE
    /\ UNCHANGED <<revoked, transfer, highWater, accepted, retiring, released,
                   slot, candRefs, owed, presentedRefs, presentedLive>>

(***************************************************************************
 * Transfers. Begin reserves staging AND resident credit together, so a     *
 * completing transfer can never fail at End for budget. Freshness is       *
 * checked on the new admission only; an older accepted generation stays    *
 * referenceable until it is retired.                                       *
 *************************************************************************)
AdmitTransfer(r, g) ==
    /\ granted
    /\ transfer[r] = 0
    /\ g > highWater[r]
    /\ Cardinality(Staging) < StagingCap
    /\ Cardinality(accepted) + Cardinality(Staging) < ResidentCap
    /\ transfer' = [transfer EXCEPT ![r] = g]
    /\ highWater' = [highWater EXCEPT ![r] = g]
    /\ UNCHANGED <<granted, revoked, accepted, retiring, released,
                   slot, candRefs, owed, presentedRefs, presentedLive>>

AcceptTransfer(r) ==
    /\ transfer[r] # 0
    /\ accepted' = accepted \cup {[res |-> r, gen |-> transfer[r]]}
    /\ transfer' = [transfer EXCEPT ![r] = 0]
    /\ UNCHANGED <<granted, revoked, highWater, retiring, released,
                   slot, candRefs, owed, presentedRefs, presentedLive>>

\* Cancel or deadline. Reclaims staging only; it never touches accepted bytes.
FailTransfer(r) ==
    /\ transfer[r] # 0
    /\ transfer' = [transfer EXCEPT ![r] = 0]
    /\ UNCHANGED <<granted, revoked, highWater, accepted, retiring, released,
                   slot, candRefs, owed, presentedRefs, presentedLive>>

(***************************************************************************
 * Pacing and candidates. The slot credit is reserved at grant and carried  *
 * through assembly into pending work; it is not released at Begin, which   *
 * would hand the same slot to a second promise.                            *
 *************************************************************************)
GrantPermit ==
    /\ granted
    /\ slot = "free"
    /\ slot' = "permit"
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released, candRefs, owed, presentedRefs, presentedLive>>

ExpirePermit ==
    /\ slot = "permit"
    /\ slot' = "free"
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released, candRefs, owed, presentedRefs, presentedLive>>

\* A well-formed Begin consumes the permit and thereby owes a terminal result.
BeginCandidate ==
    /\ slot = "permit"
    /\ slot' = "assembling"
    /\ owed' = TRUE
    /\ candRefs' = {}
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released, presentedRefs, presentedLive>>

NameResource(k) ==
    /\ slot = "assembling"
    /\ k \in accepted
    /\ candRefs' = candRefs \cup {k}
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released, slot, owed, presentedRefs, presentedLive>>

EndCandidateValid ==
    /\ slot = "assembling"
    /\ candRefs \subseteq accepted
    /\ slot' = "pending"
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released, candRefs, owed, presentedRefs, presentedLive>>

\* Incomplete, invalid, or timed out. The obligation settles with an outcome
\* rather than internally, or the peer waits forever for a frame it may demand
\* only after a terminal result.
EndCandidateRejected ==
    /\ slot \in {"assembling", "pending"}
    /\ slot' = "free"
    /\ owed' = FALSE
    /\ candRefs' = {}
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released, presentedRefs, presentedLive>>

\* Weakened control only: an assembly deadline that adjusts server bookkeeping
\* without emitting a terminal result. The peer demands its next frame only
\* after an outcome, so this strands it. `NoAcceptedObligationLost` must catch it.
SilentAssemblyTimeout ==
    /\ ~TimeoutEmitsOutcome
    /\ slot = "assembling"
    /\ slot' = "free"
    /\ candRefs' = {}
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released, owed, presentedRefs, presentedLive>>

SubmitCandidate ==
    /\ slot = "pending"
    /\ slot' = "submitted"
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released, candRefs, owed, presentedRefs, presentedLive>>

PresentCandidate ==
    /\ slot = "submitted"
    /\ presentedRefs' = candRefs
    /\ presentedLive' = ~revoked
    /\ slot' = "free"
    /\ owed' = FALSE
    /\ candRefs' = {}
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released>>

(***************************************************************************
 * Retirement. Retire moves a generation out of resident, but storage still *
 * reachable by a presentation is not releasable.                           *
 *************************************************************************)
RequestRetire(k) ==
    /\ k \in accepted
    /\ Cardinality(retiring) < RetiringCap
    /\ accepted' = accepted \ {k}
    /\ retiring' = retiring \cup {k}
    \* Retire forbids new references, and a candidate still assembling has not
    \* earned its pins, so one naming this key is rejected rather than left to
    \* reach End against storage that is already leaving the resident class.
    /\ IF RetireRejectsAssembling /\ slot = "assembling" /\ k \in candRefs
       THEN /\ slot' = "free"
            /\ owed' = FALSE
            /\ candRefs' = {}
       ELSE UNCHANGED <<slot, owed, candRefs>>
    /\ UNCHANGED <<granted, revoked, transfer, highWater, released,
                   presentedRefs, presentedLive>>

ReleaseResource(k) ==
    /\ k \in retiring
    /\ k \notin Pinned
    /\ retiring' = retiring \ {k}
    /\ released' = released \cup {k}
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted,
                   slot, candRefs, owed, presentedRefs, presentedLive>>

(***************************************************************************
 * Revocation retains pixels and kills input. It does not free storage.     *
 *************************************************************************)
Revoke ==
    /\ granted
    /\ ~revoked
    /\ revoked' = TRUE
    /\ presentedLive' = FALSE
    /\ UNCHANGED <<granted, transfer, highWater, accepted, retiring, released,
                   slot, candRefs, owed, presentedRefs>>

\* A late retirement never restores input authority.
LateRetirement ==
    /\ revoked
    /\ presentedLive' = FALSE
    /\ UNCHANGED <<granted, revoked, transfer, highWater, accepted, retiring,
                   released, slot, candRefs, owed, presentedRefs>>

Next ==
    \/ Grant
    \/ \E r \in Resources, g \in Gens : AdmitTransfer(r, g)
    \/ \E r \in Resources : AcceptTransfer(r)
    \/ \E r \in Resources : FailTransfer(r)
    \/ GrantPermit
    \/ ExpirePermit
    \/ BeginCandidate
    \/ \E k \in Keys : NameResource(k)
    \/ EndCandidateValid
    \/ EndCandidateRejected
    \/ SilentAssemblyTimeout
    \/ SubmitCandidate
    \/ PresentCandidate
    \/ \E k \in Keys : RequestRetire(k)
    \/ \E k \in Keys : ReleaseResource(k)
    \/ Revoke
    \/ LateRetirement

Spec == Init /\ [][Next]_vars

TypeOK ==
    /\ granted \in BOOLEAN
    /\ revoked \in BOOLEAN
    /\ transfer \in [Resources -> 0..MaxGeneration]
    /\ highWater \in [Resources -> 0..MaxGeneration]
    /\ accepted \subseteq Keys
    /\ retiring \subseteq Keys
    /\ released \subseteq Keys
    /\ slot \in {"free", "permit", "assembling", "pending", "submitted"}
    /\ candRefs \subseteq Keys
    /\ owed \in BOOLEAN
    /\ presentedRefs \subseteq Keys
    /\ presentedLive \in BOOLEAN

\* Nothing reaches presentation without having been fully accepted.
NoPartialPresentation ==
    \A k \in presentedRefs : k \in accepted \cup retiring

\* Each class stays inside its own cap and the classes stay disjoint.
BoundedResidency ==
    /\ Cardinality(Staging) =< StagingCap
    /\ Cardinality(accepted) =< ResidentCap
    /\ Cardinality(retiring) =< RetiringCap
    /\ accepted \cap retiring = {}

\* Released storage has no remaining owner, and released is terminal.
NoOrphanedStorage ==
    /\ released \cap accepted = {}
    /\ released \cap retiring = {}
    /\ released \cap Pinned = {}

\* A new admission advances the high-water generation; an older accepted
\* generation stays referenceable. Those are different checks.
ResourceAdmissionFreshness ==
    /\ \A r \in Resources : transfer[r] # 0 => transfer[r] =< highWater[r]
    \* No storage exists for a generation that was never admitted. The first
    \* conjunct alone is trivially true, since a Begin sets both together.
    /\ \A k \in (accepted \cup retiring \cup released) :
         k.gen =< highWater[k.res]

\* An accepted candidate keeps its pins across retirement, so live storage is
\* accepted or retiring -- never released.
CandidateReferenceValidity ==
    /\ slot \in {"assembling", "pending", "submitted"} =>
         candRefs \subseteq (accepted \cup retiring)
    /\ candRefs \cap released = {}
    /\ presentedRefs \cap released = {}

\* Retained pixels never carry input authority after revocation.
InputRevocationDominates ==
    revoked => ~presentedLive

\* A consumed permit owes a terminal result; the slot is never free while one
\* is outstanding.
NoAcceptedObligationLost ==
    owed => slot # "free"

================================================================================
