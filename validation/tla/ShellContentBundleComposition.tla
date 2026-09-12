------------------- MODULE ShellContentBundleComposition -------------------
EXTENDS Naturals

(***************************************************************************
 * The seam between `ShellContentLifecycle` and `ShellWorkAreaCoordination`.*
 *                                                                          *
 * Neither model alone establishes `CoherentBundle`. The work-area model     *
 * carries `candidateReady` as an opaque boolean and proves a presented      *
 * bundle was ready, coherent, and generation-exact. The content model       *
 * proves what makes a candidate's pixels legitimate. Between them sits an   *
 * assumption neither states: that readiness stays true.                     *
 *                                                                          *
 * It does not. Readiness is latched when content, reservation, and the WM   *
 * answer agree, and content can be retired or revoked afterwards. This      *
 * model exists to check that presentation revalidates content at commit     *
 * rather than trusting a latch, which is the only way a presented bundle    *
 * can be spliced from live geometry and dead pixels.                        *
 *                                                                          *
 * Safety only. A stalled policy or renderer may prevent presentation        *
 * forever, exactly as the work-area model already allows.                   *
 *************************************************************************)

CONSTANTS MaxGeneration, RevalidateContentAtPresent

ASSUME /\ MaxGeneration \in (Nat \ {0})
       /\ RevalidateContentAtPresent \in BOOLEAN

Gens == 1..MaxGeneration
NoGen == 0

VARIABLES
    contentGen, contentLive,
    reservationGen, wmAnswerGen, ready,
    presentedGen, presentedLive, presentedHistory, lastOutcome

vars == <<contentGen, contentLive, reservationGen, wmAnswerGen, ready,
          presentedGen, presentedLive, presentedHistory, lastOutcome>>

Records == [generation: Gens, contentLive: BOOLEAN]

Init ==
    /\ contentGen = NoGen
    /\ contentLive = FALSE
    /\ reservationGen = NoGen
    /\ wmAnswerGen = NoGen
    /\ ready = FALSE
    /\ presentedGen = NoGen
    /\ presentedLive = FALSE
    /\ presentedHistory = {}
    /\ lastOutcome = "none"

\* A complete candidate whose resources are all accepted, in content terms.
PrepareContent(g) ==
    /\ contentGen' = g
    /\ contentLive' = TRUE
    /\ ready' = FALSE
    /\ reservationGen' = NoGen
    /\ wmAnswerGen' = NoGen
    /\ UNCHANGED <<presentedGen, presentedLive, presentedHistory, lastOutcome>>

\* Engine derives the work-area snapshot from that exact candidate.
DeriveReservation ==
    /\ contentGen # NoGen
    /\ reservationGen' = contentGen
    /\ UNCHANGED <<contentGen, contentLive, wmAnswerGen, ready,
                   presentedGen, presentedLive, presentedHistory, lastOutcome>>

\* The WM answers that exact snapshot.
WmAnswer ==
    /\ reservationGen # NoGen
    /\ wmAnswerGen' = reservationGen
    /\ UNCHANGED <<contentGen, contentLive, reservationGen, ready,
                   presentedGen, presentedLive, presentedHistory, lastOutcome>>

\* Readiness latches when all three agree. This is the work-area model's
\* opaque `candidateReady`.
LatchReady ==
    /\ contentGen # NoGen
    /\ reservationGen = contentGen
    /\ wmAnswerGen = contentGen
    /\ ~ready
    /\ ready' = TRUE
    /\ UNCHANGED <<contentGen, contentLive, reservationGen, wmAnswerGen,
                   presentedGen, presentedLive, presentedHistory, lastOutcome>>

\* Retirement or revocation after readiness was latched. The content model
\* permits this; the work-area model cannot see it.
InvalidateContent ==
    /\ contentLive
    /\ contentLive' = FALSE
    /\ UNCHANGED <<contentGen, reservationGen, wmAnswerGen, ready,
                   presentedGen, presentedLive, presentedHistory, lastOutcome>>

ExactBundle ==
    /\ ready
    /\ contentGen # NoGen
    /\ reservationGen = contentGen
    /\ wmAnswerGen = contentGen
    /\ (RevalidateContentAtPresent => contentLive)

AttemptPresent ==
    /\ contentGen # NoGen
    /\ IF ExactBundle
          THEN /\ presentedGen' = contentGen
               /\ presentedLive' = contentLive
               /\ presentedHistory' = presentedHistory \cup
                    {[generation |-> contentGen, contentLive |-> contentLive]}
               /\ ready' = FALSE
               /\ lastOutcome' = "presented"
          ELSE /\ UNCHANGED <<presentedGen, presentedLive, presentedHistory,
                              ready>>
               /\ lastOutcome' = "rejected"
    /\ UNCHANGED <<contentGen, contentLive, reservationGen, wmAnswerGen>>

Next ==
    \/ \E g \in Gens : PrepareContent(g)
    \/ DeriveReservation
    \/ WmAnswer
    \/ LatchReady
    \/ InvalidateContent
    \/ AttemptPresent

Spec == Init /\ [][Next]_vars

TypeOK ==
    /\ contentGen \in {NoGen} \cup Gens
    /\ contentLive \in BOOLEAN
    /\ reservationGen \in {NoGen} \cup Gens
    /\ wmAnswerGen \in {NoGen} \cup Gens
    /\ ready \in BOOLEAN
    /\ presentedGen \in {NoGen} \cup Gens
    /\ presentedLive \in BOOLEAN
    /\ presentedHistory \subseteq Records
    /\ lastOutcome \in {"none", "presented", "rejected"}

\* The composition property. A presented bundle never pairs live geometry
\* with content that stopped being live before commit.
PresentedBundleHasLiveContent ==
    \A record \in presentedHistory : record.contentLive

\* Geometry and pixels in one presented bundle come from one generation.
BundleGenerationsAgree ==
    presentedGen # NoGen =>
        \E record \in presentedHistory : record.generation = presentedGen

\* The same property as state rather than history. Dropped the earlier
\* rejection-preservation clause: it compared a value to itself, and
\* `ShellWorkAreaCoordination` already proves that one properly.
PresentedStateIsLive ==
    presentedGen # NoGen => presentedLive

\* Readiness can only be latched while the three agree, so nothing else may
\* set it.
ReadyImpliesAgreement ==
    ready => /\ contentGen # NoGen
             /\ reservationGen = contentGen
             /\ wmAnswerGen = contentGen

================================================================================
