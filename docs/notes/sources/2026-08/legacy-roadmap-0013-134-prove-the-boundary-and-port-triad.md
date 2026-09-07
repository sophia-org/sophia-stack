---
id: legacy-roadmap-0013
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 13.4 Prove The Boundary And Port Triad

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 700–1152.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0009-milestone-13-public-policy-protocol-and-hagia.md).

<!-- BEGIN IMPORTED BODY -->

Interface major 1, wire revision 3 is stable. The fixed nine-view scroller is
the frozen profile, not the feature ceiling. Hagia's retained-behavior port
ledger closed across spatial policy, Hagia Shell, Sophia session/dedicated
authorities, and the required brokers/portals. River/Wayland and Niri
compatibility machinery is excluded; retained product behavior is not.

- [ ] Implement the minimum experimental display-list, target-resolved input,
  redacted broker, and shell-role transport needed to port retained Triad shell
  workflows before the WM freeze. Keep this endpoint distinct from
  `sophia_wm_v1`; this item does not itself stabilize `sophia_shell_v1` or pull
  general rendering-efficiency work forward from Milestone 14.
  **Start with the broker, not the transport.**
  `docs/sophia-shell-v1-direction.md` is explicit that the metadata broker is the
  larger prerequisite: the redacted presentation feed has no implementation, and
  without it the shell interface would be specified against a data source that does
  not exist. Specifying a transport first would produce a wire for nothing to send.
  The feed has exactly two sources with different trust properties, and conflating
  them is the failure this row is most likely to have. Policy-authored structure —
  workspace list, occupancy, focus — is blind-safe by construction and already
  answered by `docs/sophia-indicator-descriptor.md`, because workspace state
  originates in the policy process where no broker can see it. Only toplevel
  identity for taskbars and docks needs real sanitization, and keeping the two apart
  is what lets a status bar never request identity at all.
  The first buildable piece is therefore the toplevel descriptor and its reduction:
  a record a shell can render, constructible only by reducing client metadata, so
  that leaking a title, app ID, PID, or path is a compile error rather than a review
  finding. Activation, close, and minimize ride the existing opaque action tokens,
  which already exist and already carry issuer scoping — the broker mints no new
  authority for them. Decision 2 settled the neighbouring question for the WM side:
  classifications reach policy through a capability-gated extension chunk, so the
  broker's design is no longer constrained by what fits in `SnapshotSurface`.
  The crate question is settled: `sophia-broker`, its own crate and its own
  `PolicyRole::Broker` socket. Not `sophia-portal`, whose broker is a single-use
  transfer grant lifecycle with nothing in common beyond the word, and whose own
  ownership row forbids the client-global visibility a metadata broker needs.
  **Built so far**, all with the authority reducing and the broker never holding raw
  identity: the disclosure vocabulary in `sophia-protocol`; authority-side reduction
  in `sophia-x-authority`; the `sophia-broker` crate owning trust, icon tokens,
  disclosure rules, and descriptor emission; `PolicyRole::Broker` with its own socket
  and env var; and the metadata broker health smoke reporting the real broker.
  The chain is proven to compose end to end in `crates/sophia-session/tests/metadata_chain.rs`
  — authority reduction, broker, `ChromeDescriptorTable` — including that a title
  never reaches Engine under a `ClassOnly` rule, and that the Engine ingress needed
  no widening to accept broker output.
  **The broker tranche is now hosted.** `sophia_broker_v1` has its own bounded
  schema, strict codec, revision negotiation, owner-only role socket, and exact
  protected-peer admission. A production Hagia session starts `MetadataBroker`
  in a distinct Bubblewrap domain. The broker publishes a per-surface rule to the
  X Authority; only the authority's reduced candidate returns; and the sanitized
  descriptor commits to the session owner's real Engine
  `ChromeDescriptorTable`. The default production rule is `ClassOnly`, and a
  protected executable smoke proves no title escalation, ambient display/session
  bus, inherited descriptor, host-home/temporary-file visibility, or outbound
  network. Signed archive `0004` supplies its ordered real-session proof.
  **The reference rendering tranche is now complete.** A pure Engine reducer
  turns at most sixteen exact-generation descriptors plus issuer- and
  recipient-epoch-scoped actions into a bounded title-only rectangle/text list
  and presented opaque targets. The same immutable list lowers independently
  for unequal heads; a 128-entry, 16-MiB renderer-private JetBrains Mono cache
  retains in-flight rasters safely. Exact press/release capture returns only
  the opaque capability reference and activation identity. Offline tests and a
  16-entry two-head timing probe cover this tranche. The umbrella remains open:
  the protected wire, external Hagia Shell, and shell-enabled live dispatch now
  exist, and explicit X-grab arbitration plus compiled-profile enablement are
  complete. Signed installed shell evidence remains open.

- [x] Create Hagia as a standalone Nim repository with no Triad history,
  River/Wayland dependency, inherited binary, or shared build scaffolding. Its
  independent envelope and record decoder pass Sophia's retained corpus.
- [x] Complete Hagia's first socket proof: strict snapshot assembly, exact
  affected-output request, projection encoding, committed outcome, and
  canonical Engine reduction without generated Sophia or Triad protocol types.
- [x] Keep Hagia tags, stable `ViewId` values, ordered per-output views, and
  reconnect affinity private. Project the fixed nine-view, fixed-point
  scrolling layout into one-output-per-surface Sophia geometry
  with no Hagia back door.
- [ ] Add deterministic Hagia reducer messages for view/tag changes, focus,
  movement, grouping, layout adjustment, output focus/moves, presentation
  state, floating, scratchpads, and opaque session operations.
  The fixed nine-view scroller profile now covers ordered view moves, output
  focus and movement, column consume/expel, width/height adjustment, floating,
  fullscreen, maximize, minimize/restore, and operation-slot-bound opaque
  session requests. Hagia now retains nonempty multi-tag view and
  focused-window membership through eighteen additional opaque actions;
  dynamic workspace lifecycle, occupancy navigation, and scratchpads are now
  implemented in Hagia's action catalog; configured workspace naming remains
  partial because `setWorkspaceName` has no bound action. The transitions
  remain unbound until configuration migration preserves Triad's existing chord
  meanings. Interface revision 3 admits 256 binding registrations, so capacity is not the
  constraint: the bootstrap emits 39 key plus 2 pointer bindings from Triad's
  baseline 132 key plus 5 pointer bindings, and the other 96 classify into
  shell, broker, portal, and session authorities that do not exist yet. That
  authority split, not a slot count, is what keeps binding classification on
  the pre-freeze port path.
- [ ] Add Engine-owned pointer interactions, bounded focus history, private
  checkpoint/reconciliation, and crash/restart proof while applications and
  the last committed scene remain alive.
  One completed Engine-captured move/resize now crosses as a reduced final
  interaction, and Hagia validates its target, capability, output bounds, and
  exact floating geometry. Focus and minimized histories are bounded, and an
  owner-only, size-bounded checkpoint uses a same-directory fsynced atomic
  replacement and is revalidated before complete-snapshot reconciliation. The
  owner-side recovery matrix now kills the public
  path at proposal-staged, frontend-pending, prepared, and terminal-outcome
  boundaries through the normal supervisor; all four preserve layout, restart
  at a fresh epoch, and drain cleanly. Continuous updates, a physical
  checkpoint restore and operation-phase faults remain open. The separate
  client lifecycle gate now defines post-negotiation and complete-snapshot
  crash/replacement checks; it remains unclaimed until an authorized live run.
  Hagia now emits exactly one bounded `PolicyDirty` after a restored checkpoint
  first reconciles and commits; the independent socket test proves generation
  advance and a fresh complete cycle. Both installed Hagia gates require its
  diagnostic after restart, but no physical evidence has been claimed.
  `tools/hagia_policy_physical_gate.sh` now encodes the opt-in two-output
  restore/presentation/active-output procedure. Its first authorized run
  proved checkpoint save, supervised epoch-2 restart, checkpoint load and
  reconciliation, retained fullscreen geometry, and post-restart page flips,
  then failed the exact text boundary because seven keys arrived while Engine
  and frontend focus differed. Engine now retains only those client-bound keys
  in a bounded exact-target handoff while continuing to resolve Hagia's
  reserved chords. A replacement run then proved the exact text and clean
  bounded shutdown, but the operator entered the final phrase before the
  post-restart actions. That run also showed that checkpoint occurrence 4 was
  reached during startup, before the intended physical trigger. The gate now
  arms occurrence 6, after the two pre-restart actions, labels every missing
  phase explicitly, and uses the phrase only as the final exit signal. A third
  run delivered exactly the 34 press/release events for the 16-character final
  phrase plus Enter, with no preceding physical chord events, no ordered
  actions, and no restart. It was a clean negative attempt rather than routing
  evidence. The gate now keeps an event-driven procedure visible inside Kitty,
  advances only when each committed action appears in the evidence stream, and
  withholds the final phrase until the restart and complete action sequence are
  proven. Its first guided run exposed a harness-ordering bug: the leading
  Super transition reached the application path before the bound action key
  was consumed and incorrectly entered the exact text matcher. The text proof
  now excludes non-text modifier transitions while leaving their ordinary
  client delivery intact; an unmatched non-modifier key still fails exactly.
  The next guided run proved the complete pre-restart sequence, epoch-2
  checkpoint load/reconciliation and refresh, post-restart fullscreen, and two
  maximize transitions. It then minimized its own guide before displaying the
  restore instruction and timed out with no text events. Minimize and restore
  are now one visible paired instruction. A follow-up still stopped after
  minimize with exactly the twelve routed modifier transitions belonging to
  the six committed chords. The guide now leads with an explicit three-line
  warning to press and release `Super+R` while the screen is blank, and evidence
  distinguishes immediate physical action admission from later policy commit.
  A later attempt reached the second maximize prompt but exhausted the old
  deadline while the operator was reading. Timestamp review corrected that
  diagnosis: a separate 15-second physical-sequence timeout fired, while the
  global deadline had not. Physical proofs now retain that fail-fast default
  but may request an explicit bounded override; this human-guided gate uses ten
  minutes inside an eleven-minute global ceiling and still exits immediately
  on success. The following run proved that `Super+R` was admitted and
  committed but still left a blank scene: minimize had removed the surface
  from Sophia's supposedly complete policy snapshot, so Hagia correctly
  reconciled it as destroyed before applying restore. Sophia now retains
  authority-observed facts separately from visible layers. A follow-up run
  proved that the X `mapped` observation also cannot carry policy lifetime:
  Engine admission is not a second client `MapWindow`, so an admitted surface
  may retain its pre-admission `mapped=false` observation. The snapshot now
  follows explicit request/withdraw ownership until withdrawal or removal. The
  gate additionally requires a nonempty checkpoint after restore, rejecting
  the observed committed no-op. The first request/withdraw-lifetime run then
  failed closed at startup because authority presentation observations admitted
  three transient surfaces while one lacked an X11 client route. Snapshot
  admission now intersects retained policy ownership with a live authority
  route, which survives minimize but excludes unrouted hierarchy observations;
  the regression covers that race. The nonexclusive real-Hagia restart smoke
  passes this boundary with one routed surface, nonempty epoch-2 checkpoint
  reconciliation, and clean shutdown. The following physical run proved the
  complete action sequence and, critically, retained one nonempty surface
  through minimize and restore. It then failed after accepting the final text
  because Kitty closed while Hagia's last one-surface projection was in flight.
  Public response handling now advances current Engine scene facts before
  materializing placements and retires such a response as `RejectedStale`.
  The guide now remains alive after accepting the phrase until Sophia completes
  the proof, rather than voluntarily closing during evidence settlement.
  The next attempt admitted and committed both `Super+Right` chords but never
  restarted: legitimate extra settlement checkpoints had shifted the fixed
  occurrence used for fault injection. The physical gate now correlates its
  one restart to the committed fullscreen action, the first committed
  active-output action, and the next nonempty checkpoint. Its watcher runs
  beside an `exec`-replaced Hagia, preserving the supervisor-authorized PID,
  and the marker prevents a second epoch from being killed.
  The resulting run proved restart timing, checkpoint recovery, and the entire
  physical action sequence, then flushed all 52 action-plus-text key
  transitions. It failed only because the stock semantic-result writer was
  appended after Kitty's custom guide command and therefore became unused
  guide arguments. Sophia now passes the private result path explicitly and
  the guide records the exact line it reads; an isolated completed-log replay
  proves the witness.
  The final physical run passed: one causal restart preserved the nonempty
  checkpoint and fullscreen state; every required post-restart action
  committed; all 52 routed action-plus-text transitions flushed; exact text
  changed pixels and reached a kernel page flip in 24 ms; and health, topology,
  native ownership, namespace, Xauthority, and process cleanup were clean.
  Continuous pointer updates and operation-phase fault coverage remain open
  within this broader item.
- [x] Carry committed public-policy fullscreen, maximize, minimize, and restore
  state through the X frontend's protocol-visible state transition and verify
  exact configure/state feedback. Engine geometry, focus exclusion, semantic
  minimized placement, and render-layer omission are implemented. The offline
  path now installs protected `_NET_WM_STATE` and ICCCM `WM_STATE`, waits for a
  flushed frontend acknowledgement before policy promotion, and restores the
  previous state on rejection. Exact socket tests cover property values,
  notifications, and denied client overwrite/deletion. The installed Hagia
  physical gate now proves all four transitions with real Kitty across one
  supervised checkpoint restart and clean shutdown.
- [ ] Prove the retained scrolling layout plus actions, constraints,
  focus, hidden surfaces, multi-output moves, output loss/return, crash,
  restart, and hot-swap. Port Janet after candidate validation and fallback
  behavior have their own model and deterministic tests; both remain on the
  revision-1 freeze path.
  `PolicyOutputSettlement.tla` now proves the topology core for an atomic
  two-output candidate, output loss, and generation-advancing return. Dynamic
  output ingress now uses a capacity-one udev rescan hint, an owner-wide
  quiescence/rebuild barrier, one routed-input epoch advance, complete
  scanout/pointer/RandR/policy publication, and policy-plus-presentation fence
  before input resumes. `OutputTopologyLifecycle.tla`, Alloy, Z3, and focused
  Rust tests cover the offline boundary. Its guarded physical multi-output
  disconnect/reconnect harness remains unrun, so installed evidence is open. The public
  owner now admits only complete, valid output snapshots atomically, advances
  generations after disappearance, selects a surviving active output, and
  recognizes same-ID descriptor changes without partially mutating state.
  Unified desktop output admission now also constructs a pure stable-ID plan
  with exact candidate and rollback states after revalidating the complete
  reconciliation against the owned capability snapshot. Startup still issues
  no configuration KMS mutation; atomic test/apply execution and backend
  settlement remain the next dedicated output-authority tranche. A pure
  coordinator now models those typed test/apply/rollback effects and exact
  generation/digest completions, rejects stale or phase-invalid results,
  discards test-rejected candidates, and requires terminal rollback settlement
  after apply failure. That coordinator is no longer dead code: a typed effect
  executor trait and a bounded driver now carry one prepared candidate from test
  through apply or rollback to a terminal settlement, and startup drives the real
  phase machine. Deterministic
  tests cover activation, declined test, rollback after apply failure, and failed
  recovery, and prove the declined path never reaches apply.
  **Startup's test phase now reaches the kernel.** The candidate is resolved into
  topology heads and submitted as one `TEST_ONLY` request, so the kernel judges the
  complete desktop rather than nothing at all. Startup still issues no configuration
  KMS mutation, and not because a flag says so:
  `NativeOutputTopologyValidationExecutor` has no apply to gate. Its heads carry no
  plane state, so there is nothing to scan out and applying them would activate
  CRTCs showing nothing; mutation needs `NativeOutputCommitExecutor` and real
  framebuffers.
  A validated topology still settles as rejected, because apply then refuses, and it
  refuses with the same `WouldBlock` a busy device reports. The settlement alone
  therefore cannot distinguish an accepted desktop from a busy card, so the executor
  retains what the kernel said and startup logs it separately as `validation=`.
  A topology spanning two DRM devices is declined rather than validated: one atomic
  request reaches one device, and validating a fragment would answer a question
  nobody asked.
  `tools/run_native_output_gate_tty4.sh` passed on tty4 at commit `eab7922e`,
  which is the first identity-pinned evidence for this row:
  `result=passed`, probe accepted with and without plane state, and
  `validation=accepted outputs=2 heads=2`. Its transcript carries the commit, the
  binary's checksum, the sysfs connector facts, and a digest over the body, so the
  claim can be rechecked rather than believed — the checksum matches what that
  commit builds and the digest matches the body.
  `tools/native_topology_validate.sh` runs that chain read-only against real
  hardware — capabilities, projection, reconciliation, plan, heads, phase machine —
  differing from startup only in opening the cards directly rather than through a
  seat controller. **Its first run passed on the AMD two-output reference:**
  `validation=accepted settlement=not_applied outputs=2 heads=2`. The kernel accepts
  the configured two-output desktop as one atomic request.
  What remains unproven offline is the executor adapter itself. Covering its three
  arms needs a fake atomic-commit device, which means the `drm` crate in
  `sophia-cli`, and `docs/live-backend-dependency-policy.md` keeps device-facing
  types in `sophia-backend-live`. The submission beneath it and the resolution above
  it are both covered; the seam is covered by that hardware run.
  Apply now exists behind two gates and remains **unrun**. `native-topology-apply`
  refuses without `SOPHIA_NATIVE_OUTPUT_APPLY=1`, and
  `tools/native_topology_apply.sh` refuses again, validates first, and refuses to
  apply a topology `TEST_ONLY` would not accept. What apply can reach is bounded by
  construction rather than by care: apply heads reuse the framebuffer each CRTC
  already scans out, so the only topology expressible is one whose scanout size
  matches what is displayed, and anything else declines as `NeedsFramebuffer` before
  a commit is submitted. A mode change needs a buffer allocated at the new size,
  which is renderer work and is not in this tranche.
  **That bound turns out to exclude the reference host, which was not the
  expectation.** The plan was that re-applying the topology already on screen would
  be the smallest real mutation available. It is not available: the first authorized
  run refused, and the refusal names why. `DP-2` is a 1920x1080 panel scanning out a
  2560x1440 framebuffer, because the console gives both CRTCs one buffer sized for
  the larger monitor, while the candidate asks each output for its own preferred
  mode. There is no correctly sized buffer to reuse, so no apply is expressible here
  without allocating one. Notably the shared buffer is the mirror-group shape, and
  the two outputs disagree on size, so reusing it for both heads would fail
  `MismatchedMirrorSize` as well — the same constraint reached from the other side.
  Apply therefore stays unrun, and its hardware evidence waits on the renderer
  rather than on anything in the apply path. Nothing was submitted and no output
  state changed; the resolver failed closed before the first commit, which is the
  behavior the gate exists for.
  **Apply is frame-fed, not scratch-fed.** The obvious unblock — allocate a buffer
  at the new mode's size so apply can run whenever it likes — is foreclosed by
  `docs/renderer-import-boundary.md`: native KMS initialization waits for the first
  committed-state frame rather than requiring a speculative or blank visual
  bootstrap. A scratch buffer is that bootstrap, and it would put a frame on screen
  that no committed state produced. The sequence is therefore resize the frame
  target to the new mode, compose one frame at that size from committed state, then
  apply the topology naming that frame. `LiveGbmEglFrameTargetRecord` and its
  created/retained/resized/invalidated/retired lifecycle already model the resize
  step; what was missing was the precondition tying it to activation.
  `native_output_apply_admission` is that precondition. It answers whether every
  enabled output has a valid frame target at its requested mode, and names the
  output and both sizes when one does not, so a mode change reports where the
  session is in the transition rather than reporting a hardware defect. Disabled
  targets owe no frame, for the same reason they contribute no head. Apply consults
  it before composing anything, which turns a missing framebuffer from a discovery
  mid-resolution into a refusal before submission: the reference host now reports
  `native output 2 has a 2560x1440 frame but the requested mode is 1920x1080`.
  Both the precondition and head composition read the currently scanned-out
  framebuffer through one reader, `read_native_current_framebuffer`, so they cannot
  disagree about what "currently displayed" means.
  The renderer half is now wired into normal session startup. An accepted desktop
  output plan is projected into the resource-free authority candidate, retained as
  a private startup transaction, and dispatched only after the visual runtime can
  resize its frame targets and compose the candidate frame from committed scene
  state. It then follows the ordinary atomic apply/rollback, first-presentation,
  and publication sequence; no output-policy peer owns or settles this private
  candidate, and peer reconnect cannot cancel it. The projection and complete
  `sophia-cli` all-features suite pass. The bounded
  `--output-proof-rollback-after-apply` control now forces only that private
  startup transaction into the existing reverse-card rollback path after final
  KMS acceptance and before candidate installation. It requires normal public
  Hagia, native scanout, a bounded runtime, a prepared startup candidate, and
  explicit hardware arming. `tools/run_frame_fed_output_gate_tty4.sh` binds one
  signed Sophia/Hagia build and exact DP-1/DP-2 profile to separate successful
  apply and forced-rollback logs; its verifier rejects publication in the
  rollback phase and its archive binds source, binaries, configurations, and
  connector facts. Synthetic mutation coverage passes. What remains is running
  this gate with explicit authorization on the reference DRM hardware. The
  three-slot recycling pool in Milestone 14 stays gated on its own measurement and
  is not a prerequisite here.
  Rollback heads are resolved beside apply heads, before anything is submitted,
  from the topology still on screen. Sourcing them afterwards would source them from
  a desktop that is already wrong. An output that cannot be restored fails the whole
  plan closed, so an apply never begins without a way back.
  Applying blocks and carries no page-flip event: a modeset must complete before a
  caller may believe it did, and there is no flip to wait on.
  The DRM primitives that executor needs already exist:
  `LibdrmNativeAtomicCommitRequest` exposes `modeset`, `allow_modeset`, and
  `test_only`, and property discovery already finds connector `CRTC_ID` and CRTC
  `MODE_ID`/`ACTIVE`. Composing one request across N heads now exists too:
  `build_native_multi_head_atomic_request` folds every head into one
  `AtomicModeReq` so the kernel accepts or rejects the complete topology and a
  partially applied desktop is never observable. It validates before adding any
  property, so a rejected build never yields a half-populated request, and it
  rejects an empty head set, a shared connector, CRTC, or plane, an invalid size,
  and a missing mode blob on a modeset. Heads sharing one framebuffer are a mirror
  group and must agree on scanout size, which is where the same-mode rule is
  enforced. The returned request carries the previously dead `test_only` path, so
  a caller can validate a topology without touching hardware.
  A planned timing now resolves to a mode too: `resolve_native_output_mode_index`
  matches a requested timing against a connector's reported modes and returns the
  first match, which is the same choice the capability reader makes when it dedupes
  reduced timings, so advertisement and commit cannot disagree about which mode a
  timing names. `create_mode_blob` takes that resolved mode, and
  `create_mode_blob_for_selection` delegates to it.
  `submit_native_multi_head_topology` submits one topology as one request, setting
  `TEST_ONLY` for a validation intent and reporting an unbuildable head set apart
  from a kernel refusal, since one is a mistake in what was asked for and the other
  is hardware declining something well-formed. `NativeOutputCommitExecutor` adapts
  that to the activation reducer and gates apply, so a caller can validate against
  real hardware and then decline.
  What remains is the piece that cannot be settled offline: resolving a candidate
  into heads. That means naming connectors, CRTCs, and planes for outputs that may
  not be active yet, sourcing correctly sized framebuffers for a mode that is not
  running, and sourcing the previous topology's heads for rollback.
  `native-topology-probe` exists to answer the one question that decides how much
  of that is needed: whether a `TEST_ONLY` modeset requires plane state and a valid
  `FB_ID`, which is driver-dependent. It is read-only — every commit carries
  `TEST_ONLY` and the only framebuffer it names is one the CRTC already scans out,
  so nothing is allocated and no output state changes. It submits the same modeset
  twice, once with connector and CRTC state only and once with plane state added,
  and reports the two outcomes separately. `tools/native_topology_probe.sh` runs it
  as one step: it refuses to start while a display server holds the card, because a
  `MasterUnavailable` run proves nothing, then builds, captures the report, and
  states the conclusion.
  Atomic commits need DRM master even to validate, so the probe reports
  `MasterUnavailable` rather than a rejection when a compositor holds the card, and
  refuses to draw a conclusion.
  **The framebuffer question is answered.** On the AMD two-output reference, from a
  bare TTY holding DRM master, both probes are accepted: 2 connected connectors, 36
  modes, `without_plane_state=accepted`, `with_current_framebuffer=accepted`. A
  `TEST_ONLY` modeset validates with connector `CRTC_ID` and CRTC `MODE_ID`/`ACTIVE`
  alone, so resolving a candidate into heads does **not** need a framebuffer
  allocated at the new mode's size before anything can be checked. Validation can
  precede allocation.
  Getting there required fixing a defect the probe existed to expose. Its first run
  reported both probes rejected with `EINVAL` at matching mode and framebuffer sizes,
  which was not the hardware refusing anything: `LibdrmNativeAtomicCommitRequest`
  defaulted `page_flip_event` true and `test_only()` did not clear it, and the kernel
  rejects `TEST_ONLY` together with `PAGE_FLIP_EVENT` with `EINVAL` before inspecting
  a single property. Every validation-only commit in the tree returned `EINVAL`
  unconditionally, including `submit_native_multi_head_topology`'s `Validate` intent,
  so `NativeOutputCommitExecutor::validating` would have declined every topology and
  looked like hardware saying no. The flag is now derived rather than stored, so the
  combination is unrepresentable, and a deterministic test holds it.
  That is also the reason the report now carries errno and both sizes: the first run
  recorded `Rejected` with nothing to diagnose it by, and a rejection that cannot say
  why is indistinguishable from a bug in the asker.
  A separate power authority now exists: `crates/sophia-engine/src/output_power.rs`
  holds per-output levels for the outputs the desktop currently has. Power is kept
  apart from enablement because blanking a screen and removing a monitor are
  different facts — a dark output keeps its bounds, work area, and surfaces, and
  policy must not see the transition, while a disabled output leaves the complete
  snapshot and forces a relayout. The distinction is easy to lose at the KMS layer,
  where atomic modesetting powers a head down by clearing the same CRTC `ACTIVE`
  that disables one; that is a property of the commit, not a licence to merge the
  two above it. A topology change preserves the level of every surviving output and
  keeps none for a departed one, so a mode change cannot relight what was powered
  down and a reconnected monitor cannot inherit a stale level. Power transitions do
  not travel through topology activation: they alter no geometry, invalidate no
  candidate, and need no rollback beyond the previous level. The KMS write waits on
  the same framebuffer allocation apply does.
  Reservations turned out to be largely present rather than missing. Work areas are
  already re-projected from the new output rects as part of topology publication
  (`owner_loop/topology_phase.rs`), inside the same publication that swaps the
  outputs, so geometry and work area do commit together on the hotplug path. What
  was missing was any test pinning it; a mode change that shrinks an output under a
  live reservation is now covered.
  One fail-open edge is now documented and pinned rather than fixed. Reservations
  are root-relative, so a shrink can leave one outside the new root, and such a
  reservation is filtered *before* reduction — reduction then succeeds and reports
  the full output as available, silently releasing the reservation. An out-of-root
  reservation that arrives malformed should indeed be ignored, and the pure reducer
  cannot tell that case from one that was valid until the mode changed, because it
  holds no previous state. The fail-closed path exists next door: a reduction that
  returns `None` makes callers preserve the previous work area. Closing the gap
  means deciding at the layer that does hold previous state
  (`SurfaceOutputReservationState`), and it changes behavior for every bar, so it is
  called out here rather than settled quietly.
  That decision is now taken, and it rules out both shortcuts. Failing closed by
  preserving the previous work area is wrong after a shrink: the preserved rectangle
  belongs to the larger output and would put policy beyond the screen. Releasing the
  reservation is suboptimal; preserving a stale one is incoherent. Clamping the span
  inside the pure reducer is also wrong, because the reducer cannot tell a span
  clamped by a shrink from one that arrived malformed, and admitting the latter to
  fix the former trades a fail-open edge for weaker rejection. The fix belongs in
  `SurfaceOutputReservationState`: a reservation already admitted against a larger
  root is re-projected onto the smaller one, while a reservation arriving for the
  first time is validated against the current root and rejected if it does not fit.
  Same geometry, different provenance, different answer. Implementation is open.
  Evidence follows that.
  `PolicyRefreshLifecycle.tla` additionally proves that newer dirty
  generations survive an older in-flight refresh and that active output
  settles atomically with the frontend layout. Alloy and Z3 retain operation
  binding and presentation-geometry attacks alongside their protected checks.

<!-- END IMPORTED BODY -->

## Archived subsections

- [Multi-Monitor Per-Head Composition Critical Path](legacy-roadmap-0014-multi-monitor-per-head-composition-critical-path.md)
