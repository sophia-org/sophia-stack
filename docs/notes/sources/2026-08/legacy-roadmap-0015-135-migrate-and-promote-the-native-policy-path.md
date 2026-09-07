---
id: legacy-roadmap-0015
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 13.5 Migrate And Promote The Native Policy Path

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 1321–2074.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0009-milestone-13-public-policy-protocol-and-hagia.md).

<!-- BEGIN IMPORTED BODY -->

- [ ] Install a bounded Hagia profile through the session-hosted public
  transport and canonical reducer, using only the retained column and
  scrolling layouts. Prove Kitty, Firefox, floating dialogs, work areas,
  ordered repeated actions, pointer move/resize, multi-output views,
  `glxgears`, `vkcube`, policy restart, and clean logout.
  Packaging and installation accept an explicitly supplied Hagia binary and
  publish `Sophia Hagia (Native Policy)` beside Kitty and xmonad recovery
  entries. Promotion means selecting that entry as the remembered ordinary
  session after deterministic preflight; it has no workday-duration
  prerequisite. Every exit must enter the checksummed Hagia ledger as passed,
  recovered, failed, or pending, with cumulative scenario coverage.
  Installed attempt `0001` reached both physical outputs and admitted Kitty,
  then failed closed at `layout_pending`: Hagia proposed `2560x1440` while the
  Engine retained a coherent `1323x1424` recovery extent. The immutable failed
  attempt and exact TTY recovery are preserved. Public proposals now undergo
  Engine constraint reconciliation before reducer staging, the bounded live
  restart regression passes, and a replacement physical attempt remains the
  acceptance proof.
  Installed attempt `0002` then exposed the public admission-ownership half of
  the same boundary: the corrected Manage committed, but remained eligible for
  replanning until visual retirement, producing 3,121 reconciliations before
  startup failed closed. A committed public Manage now consumes only its exact
  planning ownership while retaining the independent visual-retirement fence;
  deterministic tests cap the resulting policy and checkpoint traffic.
  Installed attempt `0003` then completed the physical session but exposed an
  evidence-only manifest collision: the release and attempt recorder emitted
  the same Hagia digest field. The immutable attempt remains failed closed.
  Signed successor `66a279286bddd0354b6022102c4dac5254e34481`
  canonicalizes that field without weakening duplicate rejection. Installed
  attempt `0004` passed exact Sophia/Hagia identity, two-output startup,
  physical actions, presentation, clean normal logout, lifecycle, coverage,
  and archive verification. Hagia is now the ordinary remembered session; the
  item remains open for the named restart, state-transition, application, and
  output-topology scenarios above.
- [x] Remove API v7 and Engine-owned workspace policy only after both freeze
  conditions below hold. The freeze conditions closed first. Configuration now
  accepts only `sophia_wm_v1`; the v7 frame codec, client-hosted socket,
  transport worker, policy reload exchange, Engine IPC error/restart branch,
  demo server/process modes, and obsolete tests are deleted. Shortcut matching
  remains protocol-neutral in Engine. The workspace reducer moved to
  `sophia-x11-wm-bridge`, where it is private compatibility state translated
  into complete public projections. The deterministic xmonad and public-policy
  regressions remain.
- [x] Freeze `sophia_wm_v1` and retain an archived revision-3 client only after
  the retained Triad behavior port is complete and the Rust reference, Hagia,
  X11 bridge, and C client pass the complete black-box reconnect/restart
  corpus. Do not remove API v7, declare stability, or create the permanent
  archived compatibility client before both conditions hold. A digest-pinned
  candidate snapshot may run beforehand, but it is not designated stable until
  this item closes.
  The first condition is defined by Hagia's `docs/triad-port-ledger.md` at Triad
  baseline `fb8fb27e`; `docs/triad-port-ledger-pointer.md` locates it and
  summarizes its 28 classified rows. Twenty-one are complete, none are partial
  or open, and seven are excluded with written product rationales. Signed
  frame-fed archive `0001` closes the physical output gate. Shared
  reconnect/restart, public xmonad migration, and the immutable archived client
  also pass.
  Before it lands, settle the wire decisions enumerated in
  `docs/wm-v1-freeze-surface.md`. Twenty-four of the 28 rows need no wire change;
  the residue was workspace-name projection, broker classification shape, the
  continuous-pointer payload, and the output logical-space contract. All four
  are now settled and normative in `docs/sophia-policy-ipc.md`. The output
  logical-space contract landed ahead of the output-authority tranche that would
  otherwise be the first thing tempted to widen `SnapshotOutput`.
  Broker classification shape is also settled, and it removed a pre-freeze
  obligation rather than satisfying one. Reading Triad's `WindowRule` at baseline
  `fb8fb27e` showed the rules are mostly parametric — a default workspace, a column
  proportion, a named scratchpad, a floating position — which no bitfield or enum
  carries, so the "small closed set in spare `capability_bits`" shape was never
  going to fit. Classifications instead ride a capability-gated extension chunk in
  the reserved `0xFF00`–`0xFFFF` range, which is uncounted and therefore costs no
  `*Begin` layout change. Nothing needs reserving in `SnapshotSurface`, and the
  classification vocabulary is no longer frozen with the revision. This option only
  became available when outbound gating landed and made clause 2 sound; the original
  analysis predates it.
  The generator now rejects any ordinary record declaring a kind in the reserved
  range, so what was a review-time rule about a number is checked.
  The last two are now settled too, both as recommended, so all four wire decisions
  are closed and none of them cost a layout change.
  Workspace names project as `ProjectionIndicator` labels: no field, no record, a
  hard 32-byte UTF-8 ceiling truncated on a character boundary, and a name is never
  an identity — activation stays on the action token so a label cannot become a
  namespace. The retained command surface closes the rest of that row.
  The continuous-pointer payload fixes its vocabulary now and its behavior later.
  `PolicyInteractionKind` gains `Drag` and `Scroll`, and four values is the whole
  vocabulary; the payload rides the existing `interaction_*` fields with the axis
  discriminant in the former `reserved_cause` slot, now named `interaction_axis`,
  scroll using the coordinate pair as its delta and leaving the size fields zero.
  `PolicyInteractionPhase` needs nothing — all
  four phases are already wire-reachable and only `End` is ever constructed. The
  coalescing rule and `Cancel`'s revocation semantics are behavior rather than
  layout, and both stay gated on the lock and security-authority epoch barrier: a
  guessed revocation contract would be worse than a late one.
  The pre-freeze implementation from this analysis is complete. The binding
  constraint remains server-to-client enum vocabularies, not record kinds: an uncounted
  extension chunk in reserved kinds `0xFF00`–`0xFFFF` stays available after the
  freeze, but enum values sit at fixed offsets inside fixed-width records where no
  side channel reaches them.
- [x] Decide the `sophia_wm_v1` forward-compatibility rule. The three-clause form
  is recorded normatively in `docs/sophia-policy-ipc.md` under Versioning: the
  frozen revision is final for record layouts and enum vocabularies; new WM-side
  facts arrive as capability-gated extension chunks in the reserved kind range;
  new authorities take new interface families. Receivers keep rejecting unknown
  kinds because gating guarantees they are never sent one they did not negotiate.
- [x] Build outbound capability gating before the freeze. It is a prerequisite of
  the rule above, not an optimization: without it a frozen client can be sent
  content it must reject. `encode_wm_v1_policy_snapshot` now takes the selected
  capability set and omits governed record kinds along with their declared counts,
  so the transfer stays self-consistent; scene outputs and surfaces stay ungated as
  core semantics. The production caller passes what negotiation actually selected.
  Two tests hold the line, both in `crates/sophia-protocol/tests/policy_semantics.rs`:
  a producer pin asserting that enabling a capability leaves ungated chunk ordinals,
  kinds, item counts, and bytes byte-identical, and one binding the corpora to the
  default set plus each capability in isolation. Clause 2 of the forward-compatibility
  rule is sound as a result: an extension chunk in the reserved kind range reaches
  only a client that negotiated it. Enum widening remains now-or-never, because no
  gate reaches a value at a fixed offset inside a record already being sent.
- [x] Give bounded resources one saturation vocabulary instead of thirty ad-hoc
  choices. `crates/sophia-protocol/src/capacity/` holds the passive types, the
  pure admission arithmetic, the report coalescer, and a driver that owns no
  clock and no thread, so a resource's behaviour at its bound is data beside its
  capacity rather than control flow at each call site. `dod.md` now scopes its
  terminal rule to the pre-admission group FIFO it was always about, and
  enumerates the five dispositions under Saturation Dispositions; the FIFO stays
  terminal, which is what keeps this a classification rather than a general
  softening. `style-guide.md` records the telemetry-schema rule for the first
  time. The X authority backpressure path reuses the shared stall ledger with its
  four existing tests unchanged, which is what proves the step is behaviour-
  neutral.
- [x] Extend `validation/tla/TargetInputPacing.tla` with the producer it never
  had. Device acquisition, bounded deferral, scoped endpoint closure that flushes
  its terminating boundary, and terminal failure are all modelled; 1,969,760
  generated and 567,966 distinct states to depth 21. Seven negative controls each
  violate their own invariant, including the one on `Drain` fairness, so the four
  new actions sitting outside fairness did not turn progress into a tautology.
  Two invariants were removed for failing their own controls rather than kept as
  decoration. Four counterexamples are pinned as deterministic Rust regressions
  in `crates/sophia-protocol/tests/capacity.rs`.
- [x] Apply the dispositions to the session-killing input and presentation
  sites, one revertible commit each. Libinput acquisition defers 50ms and then
  abandons a batch it counts, rather than ending the worker. The seven
  routed-input ingress sites close the endpoint epoch, which `architecture.md`
  already required and which needs no queue reserve because the epoch advance is
  an atomic rather than a queued record; releases still get a bounded wait that
  ordinary input does not. The XKB worker returns `XkbWorkerSaturated` and
  `XkbWorkerUnavailable` instead of panicking in its thread and blocking without
  a deadline. The present-cadence sampler slides a 1024-interval window instead
  of latching an overflow flag that killed the measurement for the rest of the
  session. A lost key-timing sidecar is consumed; a *mismatched* one stays fatal,
  because that means the serial-to-timing association is wrong. A full
  pressed-key ledger closes the epoch and flushes what it holds, which is what
  lets it drain. `input_capacity.rs` and `client_keys.rs` hold the constants and
  the flush group, and `input.rs` came down from 1444 to 1304 lines.
- [x] Drop the session observation batch from that list. It proposed
  reject-and-consume, but `session/reducer.rs` shows the batch is not telemetry:
  every observation drives a phase transition and emits a command, so dropping
  one would skip a frame render or a scanout submission while reporting success.
  The terminal error is correct and the producer-side bound already prevents the
  overflow. See `docs/research-log.md`.
- [x] Fix the frame-service arbitration deadlock that stalled every topology
  candidate. The reducer admitted a present the handler then silently refused,
  while withholding the only effect that drains what the handler refused over;
  neither could advance and no submission reached the kernel. The reducer now
  owns the ordering and each handler refusal is unreachable by construction,
  which also closes a latent session fatal in software staging.
  `validation/tla/FrameServiceArbitration.tla` models the reducer/handler split
  and violates `PresentSettles` when the old reservation is restored. An
  existing test had pinned the defect as correct; it is rewritten as the
  regression. Bound software presents orphaned by topology installation now
  settle, and the quiescence wait escalates once -- skipping runnable and
  waiting presents with `Skipped` feedback, keeping the displayed topology --
  before rejecting a candidate. See `docs/research-log.md`.
- [x] Stop a queued policy cause from being rejected for naming outputs a
  topology change replaced. Holding ordinary cycles for the whole of a candidate
  made queuing across a transition normal rather than rare, so a cause raised
  before the commit reached the projection reducer after it and failed the
  session with `UnknownAffectedOutput`. A cause's outputs are a hint about where
  work is owed, not an identity, so they resolve against the scene the request
  will actually carry; a cause that outlived every output it named still needs
  servicing, because the topology moving is itself a reason to lay out again.
- [x] Keep a policy socket timeout apart from a dead transport. The socket
  carries `SO_RCVTIMEO`, Linux reports expiry as `WouldBlock`, and folding that
  into an I/O error restarted a window manager that was merely slow -- which is
  what a client is after a topology change hands it a whole new layout. Also
  fixed alongside: the non-blocking read path restored blocking mode on only two
  of its four exits, so any other error left the socket non-blocking and made
  the next blocking read fail instantly for the same reason.

Eight consecutive gate failures on this path shared one shape, and it is worth
naming as architecture rather than as eight bugs: **the same fact is derived in
two places from different data, and nothing forces the two to agree.** The
hotplug and policy writers shared one `Quarantined` field; the wait's
precondition was stricter than what installation required; scanout quiescence
and runtime quiescence were separate definitions of the same question; the frame
reducer emitted on `native_phase` while its handler refused on `pending_frame`;
a queued cause's captured outputs outlived the scene they named. The durable
fixes were the ones that made the disagreement unrepresentable -- naming the
quarantine's holder in its type, and having both callers read one predicate --
rather than the ones that merely detected it. These duplications are semantic
rather than lexical, so they cannot be found by searching for a name; that is
why they survived review and only surfaced under execution, and it is what makes
a bounded model asserting `Gate => HandlerPrecondition` the right instrument
where the types cannot be made to carry the agreement.

- [x] Audit the post-commit topology path statically before running it again.
  `mark_policy_committed` -> `observe_presentation` -> `Stable` has never
  executed, so every defect on it is latent, and the physical gate surfaces
  exactly one per manual thirty-second run. Enumerate each guard and every
  fallible step between the commit log and `Stable`, and judge which are
  satisfiable in a normal commit, rather than discovering them one run at a
  time. The audit closed four latent failures before another physical run:
  policy commitment now forces a full repaint after taking its retirement
  baseline; parked hardware snapshots from an equal or older authority epoch
  are dropped rather than ending the session; a second policy topology effect
  cannot begin until the owner is `Stable`; and startup submission requirements
  are keyed and re-derived by opaque head identity when scanout is replaced.
  Deterministic regressions cover the stale-publication decision, back-to-back
  owner transition, and same-count head reordering. The signed head-loss/return
  run remains the promotion proof, not this audit.
- [x] Do *not* collapse `output_topology_preparation_quiescent` (scanout) and
  `topology_rebind_quiescent` (runtime), which an audit showed would break the
  working path. They read as two definitions of one question but answer
  different ones: the scanout asks whether a preparation may *begin*, and it is
  false for the whole of an installed candidate, because
  `install_applied_output_topology` restores the preparation with phase
  `CandidateInstalled` -- which is exactly when the runtime's rebind runs. An
  AND enforced at the rebind would reject every candidate installation. The
  owner's pre-apply wait ANDs them correctly, because it precedes both. Both
  now carry doc comments saying so, since the similar names are what invited
  the mistake. The audit also found the escalation log mixing a blocker
  observed before the skip with a runtime report taken after it, describing a
  state that never existed; both are now read after.
- [ ] Cover `output_topology_preparation_quiescent`, its blocker, and the head
  report with tests. They have none anywhere, and no test calls
  `begin_output_topology_preparation` either, so the scanout half of the
  pre-apply wait is unpinned. `native.rs` has roughly eighty lines of headroom
  before it becomes a twenty-seventh audit error, so new code belongs beside the
  tests rather than in it.
- [x] Drop a queued policy cause whose surface was withdrawn under it. The
  outputs fix was half of the instance: `Focus` and `Interaction` causes also
  name a surface, the projection reducer refuses one that is gone, and that
  refusal ends the session. A cause is queued long enough for this to matter
  only because ordinary cycles are held for the whole of a candidate, which is
  exactly the window in which a surface can vanish.
- [x] Bound the post-commit presentation wait. Confirmed statically that a
  topology commit does *not* force a flip: `reconfigure_output_size` returns
  false when the primary's own size is unchanged, which a three-to-two output
  change need not alter, and the commit recomposes only when it returns true.
  What the wait actually protects is narrower than it first appears -- first
  presentation already put the new *topology* on screen, so the wait is for the
  new *layout*, which is what keeps input from resuming onto stale window
  positions. That makes the stall case the safe case: a relayout that moves
  nothing produces no damage and so no flip, and in exactly that case the
  displayed layout is already the committed one. Since that is
  indistinguishable from a slow client, the wait now expires after two seconds,
  says what was missing, and restores input rather than holding a desktop at
  shortcuts-only forever.
- [x] Wait at the runtime deadline for policy requests already *issued*, and
  drop causes still queued locally. The first attempt folded the whole pending
  count into the key drain, which was wrong twice over: the queue never
  converges, because a moving pointer keeps raising fresh causes for as long as
  the drain waits, and the timeout message printed only the three key counters,
  so a policy-driven expiry reported `pressed=0 pending_deliveries=0
  release_barrier_pending=0` and named nothing. That is the same
  two-facts-one-field mistake this roadmap already describes, committed while
  documenting it. A queued cause was promised to nobody and is dropped when the
  session stops; an issued request is owed an answer. The completion check now
  reads the same count, so a cause raised during the drain cannot fail a session
  the drain just declared clean.
- [x] Drain an outstanding policy request at the runtime deadline instead of
  reporting it as pending work. A deadline lands at an arbitrary instant, so the
  last pointer motion before it can raise a focus request that cannot settle in
  the same tick; ending there discarded the user's final intent and failed a
  session on work that was never stuck. The existing bounded key drain already
  had the right shape, so policy requests are counted alongside held keys rather
  than given a second mechanism, and one that genuinely cannot settle still
  times out and is still reported.
- [x] Let an oversized recovery extent yield to the client's declared
  constraints instead of failing the session, and drop the topology-commit
  release entirely. The extent becomes both minimum *and* maximum, pinning the
  surface to exactly the pixels it had already produced, so on an output that
  cannot hold it no proposal is satisfiable and reconciliation errors out. Two
  attempts to fix that from the commit site were both wrong, for opposite
  reasons: releasing every extent stranded a surface mid-admission, and
  releasing only those no output could hold kept one that fitted the mirrored
  output and was then placed on the extended one. The commit site cannot know
  where the relayout will put a surface; `constrained_size` does, because it
  receives the target bounds. The extent is a courtesy rather than a client
  requirement, so it gives way there and the surface keeps its declared limits.
- [x] Release only the recovery extents no output in the new topology can hold.
  The first attempt released every extent, which stranded a surface still
  mid-admission: its extent was the only size evidence it had, re-priming reads
  `safe_size`, and that comes from a committed size such a surface has not got
  yet, so it never committed and never regained one. The gate showed it
  directly -- `recovery_extent_cleared reason=output_topology_changed` followed
  by a surface that configured eight times and visually committed never. Both
  keeping and clearing are wrong in different cases, so the rule is now the
  narrow one: drop an extent when no remaining output could satisfy it, keep it
  otherwise.
- [x] Release recovery extents at a topology commit. An extent records the
  pixels a client had already produced on the output it was then on, held so
  admission can show real content before the blind WM drives final geometry,
  and it lives until that admission commits. A surface still mid-admission when
  the topology changed kept a 1280x1440 extent -- half of the mirror group --
  and the relayout placed it on the 1920x1080 extended output, where constraint
  reconciliation fails the session outright rather than shrinking the surface.
  The clamp never consults output bounds, so no proposal could have satisfied
  it. Re-priming reads the client's current size and costs one pass, so
  dropping a still valid extent is nearly free; keeping a stale one is not.
- [x] Let the reference policy client tolerate a quiet owner at any time. The
  owner issues a cycle only when policy work exists, so silence is the ordinary
  state, but the client forgave a read timeout only while a topology
  transaction was preparing. Once that settled, the next silent window killed
  the client, and the request the owner sent afterwards had nobody left to
  answer it -- which the session then reported as an unsettled in-flight
  request at its deadline. A closed peer reports itself differently and still
  ends the loop.
- [x] Give clients a realistic window to acknowledge a layout. The reference
  policy asked the owner to hold one open for 300ms, which is a property of the
  clients being tiled rather than of the compositor: a topology change can
  double a surface's width -- 1280x1440 to 2560x1440 when two heads merge into
  one mirrored output -- and a terminal answering that reallocates its buffers
  and re-renders its whole grid while the compositor drives three heads. At
  300ms the same code converged with no timeouts on one run and timed out seven
  times on the next, rolling the layout back each round and never settling.
  This is tuning, not a defect fix: the compositor delivered every configure
  correctly on both runs.
- [x] Find why no present reaches "stable", which is what startup readiness
  waits on. Neither of the two candidate causes was it: starvation was ruled
  out by the eleven flips being spread across the run, and no present was
  overtaken before being displayed. The predicate asked for the wrong thing.
  Besides requiring that this transaction be displayed with real pixels -- the
  only part that is about the flip -- it required that nothing newer be
  submitted and that no head be busy, which is a claim about the instant of
  judgement rather than about the frame. Judgement happens after a whole
  `service_native` pass, and that pass submits the successor before it returns;
  a mirror group's retirement promotes the coalesced successor into every
  head's exporter slot while the retirement is still being reported. Those
  conjuncts were written when the frame reducer reserved the primary, and that
  reservation was the arbitration deadlock, not a source of quiet.
  `PresentFrameOwnership.tla` already permits a successor submitted after a
  retirement is observed, so the code had been stricter than its own model. A
  present is now stable when its page flip retired with it displayed and
  carrying nonzero pixels. Telemetry moved to `schema=2`, dropping a
  `pending_primary` field that was `!stable` under a name suggesting an
  independent fact, in favour of the pixel count.
- [x] Spend the composition pixel proof where light could actually appear. With
  the predicate narrowed, the next run reported `nonzero_rgb_pixels=0` on every
  retirement while naming the right transaction, which located the last term.
  That count is not this frame's: a context reads its composition back at most
  three times and keeps the last result, because a full-framebuffer readback
  cannot run per frame. All three attempts were spent in the first hundred
  milliseconds on compositions with zero layers -- clears to black, which prove
  nothing -- so the zero they latched rode every later present and readiness was
  unreachable by construction. An attempt now requires a composition with at
  least one layer, the budget is a named constant instead of a literal in three
  places, and the stamping site says what the value is: the head's proof that it
  has shown light, not a measurement of the frame carrying it.
- [x] Stop ending the session when it revokes its own input. Closing the input
  security epoch for an output policy change strands whatever was in flight, and
  both paths that do so -- draining frozen input, refusing an event stamped with
  the closed epoch -- reported `RouteRejected`, which the owner loop treats as
  fatal. A live run died four seconds in because the pointer moved during the
  topology change. This is the timeout-is-not-a-fault distinction again:
  `EpochRevoked` now names the session's own boundary and retires like a
  departed target, while `RouteRejected` keeps meaning a fault and keeps ending
  the session.
- [x] Compose the scene for every output, not for the primary one. The gate
  reached its telemetry stage -- readiness at 256ms, topology committed and
  settled, health clean, presents stable with real pixel counts -- and failed
  because the extended head never showed a client pixel. The presentation
  layout filtered every layer through `surface_visible_on_output(layer,
  output.id)` against the primary output alone, which is harmless with one
  logical output and wrong with two: the surface the policy placed on the
  extended output left the scene. A staged Present is released only when its
  surface is in the projection, the surface enters the projection only when it
  is composed, and the resize that places it commits only when that Present
  retires, so nothing moved again. The head stayed blank and its client, still
  waiting on that Present, stopped drawing -- which is why the terminal looked
  dead to the keyboard even though the keys were routed and flushed.
Mirror work is sequenced deliberately, because every item below changes either
the logical size of an output or the pacing of its heads -- the inputs whose
churn produced the last several gate failures. Take the physical gate to
`status=passed` first, then the optimized-head choice (contained, no new model
surface), then per-head pacing (model first), then mode matching (its own gate
run). Correct pacing outranks sharper mirroring: the first is wrong on ordinary
hardware, the second only decides which screen looks best.

- [x] Let a mirror group's heads flip at their own refresh rates. One generation
  is held until every required head has flipped
  (`LiveProductionMirrorGroupLifecycle` completes on the `flipped` set covering
  `required`), so mirroring 144Hz beside 60Hz would run both at 60. Mixed
  refresh across separate logical outputs is already independent -- each head
  owns its CRTC and its own vblank -- so this is a mirror-group question only.
  Each head should take the newest ready generation on its own vblank and
  coalesce what it missed, with the group's primary head owning present
  completion; throttling a client's frame callbacks to the slowest panel on the
  desk is worse than a mirror that briefly lags. Three things must move
  together: buffer lifetime becomes per-head (a slow head may still be scanning
  a generation the fast head has left, so release waits for every head that
  scanned it), which is frame-slot semantics and so is modelled before any code
  -- `validation/tla/MirrorHeadPacing.tla` now states the successor rule, and
  restoring today's joint advance in it violates `PrimarySubmitNeverBlocked`,
  which is what the fast head being throttled looks like in the model;
  `stable_present` stops
  quantifying over every head of the output and names the primary; and the
  mirror gate's matching-content criterion must permit the bounded lag or it
  will fail correct runs. The Rust slice is now implemented: topology carries
  an explicit primary per logical output, the prepare-all cohort completes
  logical presentation on that primary, each head independently submits the
  newest complete generation and coalesces stale unsubmitted work, and native
  owners release only after the last scanning head moves on. The mirror and
  mixed verifiers require ordered `primary_presented -> released` evidence, and
  their visual prompts judge convergence only after motion settles. Engine,
  backend feature, verifier-fixture, and TLA checks are local prerequisites;
  signed source `e946cc725bf731515a477c86e9a575554965418c` passed and
  independently re-verified mirror archive `0007` and mixed archive `0001`,
  including ordered primary-presentation/last-head-release evidence on the real
  mismatched topology.
- [x] Make the optimized head a property of a mirror group, as macOS does. The
  choice turned out to be expressible already: a group proposal carries one
  logical rect and a mapping per member, so optimizing for a head is sizing the
  group to that head's mode and marking it `Exact` while the others `Fit`. The
  compositor needed nothing -- it already plans each head against the group's
  logical size. The reference policy now takes the choice as a parameter and
  the gate exposes it as `SOPHIA_MIXED_OPTIMIZE_FOR_LABEL`, defaulting to the
  primary, which is what every run so far did implicitly. Note this also makes
  the group's size follow the optimized head's mode rather than whatever rect
  the group already had; those coincide on this rig and the former is the
  meaningful one.
- [x] Give the pixel evidence a population that reads intensity. The plan to
  filter in linear light rested on a claim that turned out to be false: that the
  composition-region pixel populations already reported would show the change.
  They could not. Those buckets key on which channels are lit and never on how
  brightly, deliberately, so that a palette check survives an intensity
  conversion and still exposes a channel swap -- the property that makes them
  good at their job makes them blind to this one. Gamma moves intensity and
  nothing else, so every one of them would have held still while the pixels
  underneath changed, leaving only `checksum`, which proves a difference without
  saying which way it went. `luminance_sum` and `luminance_buckets` were added
  for it, with integer weights summing to 256 so the shift never rounds and the
  numbers stay as reproducible as the checksum beside them. Judge a filtering
  change on the histogram, not the sum: a mean holds still while the population
  behind it splits, which is exactly the shape gamma-space filtering makes.
- [x] Filter in linear light. Every filter weight was applied to gamma-encoded
  bytes as though they were light, which is the ordinary cause of muddy edges on
  resampled text: averaging the encoded bytes 0 and 255 gives 127, about a fifth
  of the light of white rather than half of it. The reconstruction shader now
  decodes each tap before weighting it and re-encodes the sum once at the end,
  under gamma 2.0, matching the transfer function `software/raster_replay.rs`
  already chose for the CPU path and for its stated reason.

  Both directions, from one program. Catmull-Rom is an interpolating kernel, so
  it is the textbook bicubic upsample as well as a reduction filter; the upscale
  path stopped falling through to a hardware bilinear and no new kernel had to be
  chosen. The sampler is `NEAREST` for every reconstructed draw, which is the
  part that is easy to get wrong invisibly: a hardware `LINEAR` filter would
  blend the texels in gamma-encoded space before the shader ran, and the evidence
  would still read `sharp_downscale status=active`. The filter and the program
  are now one decision taken in one function rather than two that could disagree.

  Premultiplied sources are unpremultiplied across the decode -- `v*v/a` in and
  `sqrt(L*a)` out -- and alpha is never transformed, being coverage rather than
  light. The clamp precedes the encode because Catmull-Rom rings negative and
  `sqrt` of a negative reaches the screen as a hole. Three negative controls
  were run against the shader and the filter policy; each fails the intended
  test when reverted.
- [ ] Give the upscale direction a real kernel. Superseded in part: Catmull-Rom
  is itself the textbook bicubic upsample, so the linear-light change serves
  both directions from the one program that already exists and the upscale path
  stops being a hardware bilinear of encoded bytes. What remains is the question
  of whether bicubic upscale is sharp enough -- Lanczos-2 first, small and
  predictable on glyphs with no vendored source, with FSR 1 EASU or NIS as the
  alternative if photographic content proves to matter more than text. Both are
  MIT and self-contained, but either would be the first third-party shader in
  this tree, which is a provenance decision rather than a detail. A sharpen-only
  post-pass is separable and optional; contrast-adaptive sharpening rings on
  glyphs that were already crisp. None of this may change the reported sampling
  class.
- [x] Keep the GLSL in its own files and compile it before a GPU sees it. The
  shaders were string literals in Rust, which is workable until you notice that
  a shader failing to compile is not fatal by design: the pipeline logs
  `status=unavailable`, falls back to the direct program, and the session runs on
  with its filtering silently uncorrected. Right at runtime, wrong as the first
  place a typo is discovered. `tools/check_shaders.sh` compiles every
  `.vert`/`.frag` under `crates/*/src` with a real GLSL front end; it refuses to
  run without a validator and refuses a run that matched no sources, since a
  search over nothing would otherwise report success. Front-end only -- it does
  not know a driver's limits or whether a uniform was bound.

- [x] Bound head-native content by the scene, not by the framebuffer. Every
  mirror policy until centre-unscaled projected the scene across the whole head,
  so "the framebuffer" and "the region the scene occupies on it" were one rect
  and clipping to either gave the same answer. A scene placed inside a border
  separates them, and content bounded by the framebuffer paints into the margin
  that is meant to hold background alone. Borders showed it first because they
  are bright lines and because they were not clipped at all -- surfaces and the
  cursor were clipped, just to the wrong rect.

  Border bands are clipped one at a time rather than by clipping the `outer` and
  `inner` rects they are derived from. Those are not the same operation: where
  the clip leaves the two degenerate their difference is still positive, so a
  window lying entirely outside the scene keeps a band at its original
  off-screen coordinates. The first version of this note claimed the wrong
  failure mode -- an invented band at the clip edge -- and enumerating the cases
  showed it does not happen; the surviving off-screen band does.

- [x] Relay out for a public policy when the work area moves. The check sat
  below the early return into the public path, so it ran only for a private
  policy: three places set `work_area_relayout_required` and one read it, and
  that reader was unreachable in every session running a public policy client --
  which is every session running the reference WM. `enqueue_relayout` opens by
  handling the public case, so the capability was always present and only the
  ordering kept it from being reached.

  What it cost was window chrome. Chrome clearance going from zero to two raised
  the flag when a focus ring first appeared; nothing consumed it, so windows
  stayed placed against the old clearance while the ring was drawn against the
  new one. A ring is drawn outside the window geometry, and the window filled its
  output exactly, so the ring landed wholly outside that output. The only part
  visible anywhere was the sliver crossing into a neighbouring output in root
  space -- one monitor's window border appearing on another monitor, and none on
  its own.

- [x] Convert a public policy's placement into content geometry without losing
  the outer allocation that blind policy owns.

  The defect is real and located. A public policy's placement is an outer
  allocation, chrome included -- the code says so in a comment directly above the
  line that assigns it into the layer as content geometry. The private path
  converts through `apply_surface_chrome_clearance`; the public path assigns it
  raw, so every surface it places fills its whole allocation and the chrome drawn
  around it has nowhere to go. A focused window allocated an entire output puts
  its focus ring wholly outside that output, visible only where it crosses into a
  neighbouring one, which is exactly what the rig shows.

  The conversion reached the geometry -- a surface was observed at 1916x1076 --
  and the session then failed. Layout timeouts went from one per run to seven,
  and the session ended with `wm_layout=1` pending. Adding a content-size request
  alongside the converted extent did not help, which is the fact that matters:
  the second attempt assumed the client was never told to resize, and it made no
  difference, so that is not the whole reason either.

  The missing boundary was transaction ownership. Reconciliation now carries
  both projections: the outer geometry remains the policy/reducer value, while
  chrome clearance is applied before layout-epoch reconciliation and produces
  the content geometry plus any client configure request. The public proposal
  materializes that content projection, but the reducer stages and commits only
  the outer projection after layout settlement. An omitted policy configure may
  therefore still generate the content resize that chrome requires without
  making an unacknowledged geometry current. Schema-2 chrome records expose the
  reconciled, acknowledged, and settled stages, and regressions cover the
  outer/content split, generated configure, and post-settlement materialization.

  Signed mixed attempt from `7ff94e20` proved the projection arithmetic and
  exposed one remaining ownership race. A second public relayout arrived after
  the first resize epoch had installed its content geometry and armed exact
  pixels, but before one surface's native retirement committed its size. The
  staging filter compared only with the last retired size, treated the identical
  target as new, and sent a second configure that displaced the armed candidate.
  An installed target already owned by `ResizeVisualCommitTracker` now removes
  only that duplicate resize request. A standing recovery target whose geometry
  is not installed still configures normally. Both sides have deterministic
  regressions. The signed `3d19e2e6` rerun passed runtime, telemetry, and visual
  confirmation; this resize race is physically closed.

- [x] Give the composition-region trace a head. The line carried a rect and no
  output identity, and on the three-head rig two heads compose a `1920x1080_0_0`
  rect -- DP-2 extended at `mapping=exact` and the DP-3 mirror member at
  `mapping=fit`. Their records are indistinguishable, so a before-and-after keyed
  on the rect compares whichever the collector happened to keep, and the first
  read of the linear-light verification was byte-identical for exactly that
  reason: it was comparing the unfiltered head with itself. The correction was
  confirmed on the 2560x1440 regions instead, whose geometry only one head has,
  which is luck rather than method. `trace_final_composition_region` takes the
  pipeline, stage, layer index and rect; the output identity has to reach it.
  Same class as the raster/presentation split -- a record that does not carry the
  fact needed to read it. `LiveCompositionTrace` now carries output, opaque head,
  and scene generation through lowering into the native renderer. Sampling
  evidence is schema 3 and keyed by all three, so the verifier correlates the
  exact draw between that head's queue and submit records instead of selecting
  an arbitrary equal-sized region.

- [ ] Filter the blend in linear light too, and the opacity multiply with it.
  The fixed-function ROP mixes `dst*(1-src.a)` on gamma-encoded destination
  bytes, and no shader can reach it: it needs `GL_FRAMEBUFFER_SRGB` and an
  sRGB-capable EGL surface, neither of which has been probed on this hardware,
  and imported client textures take their format from EGLImage rather than from
  us. The shader's `rgb * opacity` and its `min(rgb, a)` ringing clamp stay in
  encoded space alongside it deliberately -- moving one without the other would
  leave them inconsistent, so they travel together as one later change.
- [x] Offer centre-unscaled as a third mirror sizing policy. Both optimized
  policies make one panel pixel-exact by making the other stretch; this one gives
  that up so neither does, at the cost of an unused border on any head with room
  left over. It is the only policy under which both panels are exact at once.

  The compositor needed nothing. `OutputHeadMapping::Exact` already takes the
  logical size verbatim and the projection already centres it, so "this head owns
  the size" and "this head shows the image unscaled in a border" are the same
  placement; what differs is only whether there is a remainder. The policy
  chooses the size and the mapping follows, which is why this landed as a value
  of the reference client's sizing choice rather than as a compositor change.

  Two things it forced into the open. The size is the per-axis minimum across the
  group, not whichever member is smaller: two heads need not be ordered, and
  taking either mode whole would run the other's image past its edge where
  clipping crops rather than borders it -- a policy promising nothing resamples
  would have silently lost pixels instead. And the applied-topology predicate now
  reads the logical size, not the member mappings alone, because two exact
  members sized to the larger head are exactly that cropping configuration and
  wear the same pair of mappings. The type was renamed with it: a value that
  optimizes for neither head cannot live in a type called "which head is
  optimized" without one of its values meaning something else.

- [ ] Decide whether mirror members should be re-moded rather than resampled.
  Windows Duplicate and X both refuse to scale: they restrict the desktop to a
  mode every member supports, so each panel scans out natively and the larger
  one runs below its own resolution. That is a different knob from the
  optimized head -- one chooses which size the desktop is rasterized at, the
  other chooses whether members change mode to match it -- and it touches the
  topology candidate and rollback machinery, so it wants its own gate run.
- [ ] Watch whether the public policy transport restarts on quiet rather than
  on failure. A mixed run restarted the reference WM with
  `reason=public_transport_failed error=TimedOut` after twelve seconds of
  silence from a policy client that was alive. Restarting a wedged policy is the
  designed recovery and it worked -- layout preserved, session continued -- so
  this is not being loosened without evidence that the peer was healthy. It is
  worth distinguishing if restarts recur: what the owner was waiting for is not
  in the line, only that it waited.
- [ ] Demand rasters at the extent that was presented into, not at the raster's
  own extent. With a raster now stating its own size, `raster_requirements`
  briefly asks the authority to re-raster at a stale client's extent instead of
  the extent it was asked to fill. It self-corrects on the client's next frame,
  and fixing it properly means carrying the presentation extent on
  `CommittedSurfaceState` as well.
- [x] Classify sampling by extent, not by density alone. `sampling_class`
  compares `density_millis` against the projected density and never looks at
  whether the raster spans the geometry it is placed into, so a stale frame
  scaled up still reports `Exact`. The mixed-output gate's sharpness criterion
  reads that field, which means it is currently provable by a frame that was
  visibly resampled.

  The full shape of it, found while planning the linear-light work. The engine
  and the renderer each decide independently what a draw is: the plan compares
  two `density_millis` scalars, while `native_composition_sampling` compares the
  real source and target rectangles. Nothing connects them and nothing forces
  them to agree -- the parameter the renderer calls `requested_sampling` is its
  own geometry-derived value, a homonym of the engine's field rather than a copy
  of it. They diverge in four separate ways: content whose `logical_extent`
  differs from its placed geometry, which is the ordinary condition of any
  surface mid-resize; plain rounding, because the projected density truncates
  while the expected pixel size ceilings and each projected edge truncates
  independently, so any scale that is not dyadic-clean lands a pixel off; mixed-
  axis projections, where one scalar cannot describe an upscale in x and a
  downscale in y; and retained renderer images, which are exempt from the
  source-size check by design and so are classified from a number that is not
  the one drawn. A plan line reading `mapping=exact exact=1 downsampled=0
  upsampled=0` -- the exact string the gate greps for -- is therefore satisfied
  by a frame the GPU resampled, and the one renderer-side check in that script
  rejects `status=fallback|unavailable` while passing
  `requested=sharp_downscale status=active`. Secondary consequence: the plan's
  one-pixel repaint dilation for filter footprint is gated on the plan's class,
  so it is skipped exactly when the renderer is in fact filtering. It is the
  same defect class as the raster/presentation extent split -- one fact derived
  in two places with nothing forcing agreement -- and the fix is to make one
  derivation the source and the other read it.

  Engine now classifies each axis from realized source pixels and native target
  pixels, including an explicit mixed-axis class. Lowering recomputes that class
  after the actual retained image extent is known, passes it unchanged to the
  native renderer, and records the realized source extent in damage evidence.
  The mixed-output verifier requires a schema-2 plan with zero mixed draws and a
  keyed schema-3 renderer `exact` draw between queue and submit; legacy or
  fallback-only sampling lines fail closed. Exact, upscale, downscale, mixed,
  stale-retained-image, and verifier-mutation regressions pass. A signed physical
  candidate is still required before the mixed gate is promotion evidence.
- [ ] Apply the timeout-is-not-a-fault distinction to the two remaining socket
  transports. Four places set `SO_RCVTIMEO`/`SO_SNDTIMEO`; the session's policy
  transport and the reference policy client are now fixed, while
  `sophia-runtime/src/output_transport.rs` and
  `sophia-wm-demo/src/output_v1.rs` still treat an expired window as an I/O
  failure. Neither has bitten yet because the output role completes early, but
  the class has now cost two separate sessions.
- [x] Force a topology commit to repaint rather than
  relying on the relayout to generate one. The first full run settled naturally
  -- presentation baseline 26, retirements 27 -- so the relayout did produce the
  flip, and the bounded wait added alongside never fired. Nothing guarantees
  that, though. With `policy_required` set, `mark_policy_committed`
  replaces the presentation baseline with the live retirement counter, so
  `observe_presentation` then needs a strictly *new* flip -- and neither of the
  two things the commit touches produces one. `scene.reconfigure_output_size`
  is conditional on the primary's own size changing, which a three-to-two output
  change need not do, and `cursor_updates.dirty` drives the cursor plane. If the
  relayout's flip lands before the pass that snapshots the baseline, the owner
  parks in `AwaitingPresentation` with no watchdog, deadline, or retry, and
  `input_quarantined()` holds routing at `ShortcutsOnly` for the duration:
  pointer motion dropped, cursor frozen, every non-shortcut key discarded. The
  baseline overwrite is deliberate and the invariant behind it is right -- a
  presentation that retired before the projection committed cannot release the
  quarantine -- so the fix must make the flip happen rather than weaken the
  requirement. `mark_policy_committed` now clears the CPU damage baseline and
  queues one full repaint after taking that baseline, making the required newer
  presentation causal instead of timing-dependent.
- [x] Give `pending_hardware_output_publication` the same treatment as the
  policy cause. It is parked across a candidate by design and its topology-epoch
  check is a hard error (`StalePublishedSnapshot`), so a snapshot parked before
  a commit that advances the epoch ends the session rather than being resolved
  or dropped. Publication now compares against the current authority topology
  epoch and drops an equal or older parked snapshot with a named record.
- [x] Guard `begin_policy_change` at its call site on the owner being `Stable`.
  There is no phase check there, so a second `sophia_output_v1` proposal
  promoted while the owner is `Published` or `AwaitingPresentation` fails with
  "live policy topology change overlaps another transition". Promotion now
  waits for `Stable`; a regression proves a second candidate cannot begin until
  post-commit presentation settles.
- [x] Re-derive `startup_required_submissions` when the scanout is replaced. It
  was positional over head indices with absolute submission counters, and only a
  length check guarded it, so a same-count reordering compared against the wrong
  head and a replacement scanout made the requirement permanently
  unsatisfiable. Requirements are now a map keyed by `RenderHeadId`, validated
  against exact head coverage, and rebuilt from the replacement scanout's head
  counters and focused geometry.
- [ ] Stamp work queued across a topology transition with the epoch it was built
  in, and revalidate or rebuild it on mismatch, rather than fixing instances.
  An inventory found the class runs wider than the two already fixed: absolute
  geometry in `session_controls` configure packets and in `Interaction` causes,
  pre-transition pointer coordinates released from `pointer_focus_handoff`, and
  the legacy WM's `queued_requests` carrying enqueue-time output and bounds --
  all of which produce a wrong result silently rather than an error. Follow the
  precedent already in the tree: `ApplicationRouteLease` stamps output,
  presentation epoch, authority session epoch, and control epoch, and validates
  all four at use with typed refusals instead of a session error.
- [ ] Bound the page-flip callback *read* rather than its write, the last
  session-killing site. The event is already consumed from the DRM fd when the
  queue is examined, so a write-side degrade would lose a retirement outright.
  Land it alone and last: it restructures the most physically coupled code here,
  and `fake_source.rs` should give deterministic coverage first.
- [ ] Close the remaining silent drops once the fatal sites are converted: the
  three `broker.rs` swallow points, the unbounded input-delivery channel, the
  deadline-free route-lease send, and config-time bound publication. Add
  `discarded = 0` assertions to the gate verifiers so a degraded run still fails
  rather than passing quietly, which is the one risk converting fatals to
  degradations actually creates.

Milestone 13 exits only when the public wire is independently implementable,
the retained Triad behavior port is complete across the correct authorities,
the formal and deterministic gates pass, installed Hagia uses the Engine
projection path, and a policy crash or replacement preserves the last coherent
desktop.

---

<!-- END IMPORTED BODY -->
