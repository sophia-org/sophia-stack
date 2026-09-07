---
id: legacy-roadmap-0016
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Milestone 14: Native Graphics Efficiency

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 2075–2321.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

<!-- BEGIN IMPORTED BODY -->

This milestone starts after installed Hagia is the usable ordinary session and
the revision-1 compatibility gate passes. It does not wait on elapsed dogfood
time. It optimizes the same native-X product; XLibre, Xorg, niri, river, and
other mature compositors are references rather than Sophia runtime components.

- [x] Extend the bounded visual-retirement model before changing frame-slot,
  coalescing, multi-output, shared-worker, direct-scanout, or buffer-lifetime
  semantics. Check out-of-order output retirement, supersession, fallback, and
  release safety. Retain a deterministic Rust regression for every
  implementation-relevant counterexample.
- [x] Recycle three generational frame-surface slots per output through
  explicit page-flip retirement, with bounded deferral when all slots are
  leased. Promoted on signed native archive `0001`: requests settled as
  completions plus bounded deferrals with no stale release and no slot leased
  at completion, and both presented heads reached full three-slot occupancy.
- [x] Carry bounded buffer-age damage history per slot and repaint only
  accumulated damage. Fall back to a full repaint whenever history is
  incomplete. Promoted on signed native archive `0002`: 129 partial repaints
  beside 627 full fallbacks, 201 history records with zero invalidations, and
  the slot ledger still exact at 201 requests settling as 201 completions with
  no stale release and no slot leased at completion. The model boundary came first
  (`VisualDamageHistory.tla`, negative controls for under-computed damage and
  a rebuilt bundle keeping its recorded generation). The content age comes
  from `EGL_BUFFER_AGE_EXT` per acquired back buffer, answering the brief's
  open question; a monotonic bundle generation catches every rebuild path; and
  every path that cannot prove an age falls back to a full repaint with a
  named reason. `tools/check_buffer_age_equivalence.sh` proves on this host's
  GPU, through a render node only, that a damage-limited render is
  byte-identical to a full one across a twelve-frame sequence with real
  partial repaints, and that a lying damage table is caught by the same
  comparison. The feature is opt-in via `SOPHIA_ENABLE_BUFFER_AGE_DAMAGE=1`;
  the native gate exports it as the promotion step, and its verifier requires
  schema-8 evidence in which at least one frame rendered partially. Tick this
  box when that gate passes.
- [x] Keep one latest pending frame and one KMS submission in flight per head,
  and prove the 2026-07-31 stage contract at p99 against the measured refresh:
  full chain below two refresh periods, queue dwell within 1 ms,
  dwell-to-submit within one refresh, and submit-to-page-flip within one
  refresh plus one millisecond of commit-and-completion jitter -- a press
  arriving just after a vblank waits the full period, and the atomic commit
  and completion event add time no pipeline can remove, so the flip stage's
  bound carries the allowance explicitly rather than pretending the period
  alone is achievable. Stage percentiles come from the pooled in-session
  press distributions, never from the one-shot correlation, whose stage
  split measures gap phase once presses are spaced.
  This row originally said "half a refresh period at p99" for the full chain.
  That is stricter than the one-refresh bound the 2026-07-31 entry already
  rejected on physical evidence -- a randomly phased input can spend nearly a
  whole refresh waiting for the next synchronized flip -- so the sentence is
  superseded rather than reinstated. It also said per output; a mirror output
  holds one submission per head by design, which is what `MirrorHeadPacing`
  authorizes.
  The mechanism was already enforced and model-checked
  (`FrameServiceArbitration`'s `OneSubmissionInFlight`); what was missing was
  evidence, now `max_in_flight_per_output` and `pending_frame_supersessions`
  in `sophia_live_native_resources` schema 9. Input-to-photon sampling no
  longer latches after one correlation, so a session carries its own
  distribution, and the reporter takes p99 over that population against the
  refresh the session recorded, refusing below two hundred samples.
  Proved on source `96b00d0d`, physical run `20260828T231430Z`:
  thirty-five sessions, two hundred forty-five independent presses, zero
  page-flip stalls, `status=passed failed_gates=none` -- full chain p99
  24 ms against the 34 ms two-refresh budget, queue dwell 1 ms,
  dwell-to-submit p99 7 ms against one refresh, submit-to-page-flip p99
  18 ms against one refresh plus the named jitter millisecond. Getting the
  measurement honest was the row: the correlation was accepting a flip
  carrying a composition older than the press, readiness was accepting a
  pre-content picture on glass, and the reporter was gating stages on one
  press's vsync phase. Each fix is pinned by regressions that reproduce the
  recorded defect shape. Evidence under
  `rendering-benchmarks/96b00d0d*/input-latency/20260828T231430Z`.
- [x] Coalesce all outputs in the same DRM/render-device group onto one shared
  renderer worker. Preserve one latest pending request per output, bounded
  response demultiplexing, explicit per-output retirement tokens, and bounded
  inter-output service skew under concurrent producers.
  Promoted on signed native archive `0003`, Sophia source
  `8806046462bdd2f8c23c2702427e0d8b9fd7cd1b` against Hagia
  `9c9a59061fd0d8e88310b764f7dd240e729fb035`: two heads of one card on one
  renderer thread, `renderer_workers=1` with `worker_result_misroutes=0`,
  `worker_max_service_skew=1` inside the one-per-sibling bound,
  `max_in_flight_per_output=2` for two presented heads, 207 worker requests
  settling as 207 completions with no deferral, no stale release, and no slot
  leased at completion.
  The model came first (`SharedWorkerService.tla`, three load-bearing negative
  controls), and it refused two drafts before it held: skew compared service
  counts directly, which fires on an idle head that simply has nothing to
  draw, and the environment forbade composing during an in-flight render,
  which is what the code does and whose absence made the submission gate's
  control unable to fail.
  The renderer context was the load-bearing change. One three-slot array
  serving two outputs rebuilds a bundle on every alternation between their
  sizes, so slots and the pixel proof taken from them are keyed per output;
  results are routed on a channel per output rather than correlated by
  position, and the misroute check is kept anyway. Renderer images are now
  imported once per device rather than once per head.
  Two instrumentation defects surfaced only under real evidence.
  `max_in_flight_per_output` read a field only the mirror path sets, so it had
  been structurally zero for every non-mirror session since schema 9 landed --
  this row's central claim had never actually been measured on an extended
  desktop. And the output key was unique only by an argument about scope,
  which a two-card guest reporting `head=1` twice disproved; it composes group
  with head now, and duplicate registration is refused rather than trusted.
- [x] Add atomic-test-gated direct scanout for one compatible opaque DMA-BUF
  layer. Engine proves the exact frame has no overlay or active effect that
  samples the scene, uses an offscreen group, or otherwise requires
  composition; the backend re-derives the same structure from the lowered
  layers and refuses on any disagreement. `PresentFlipOwnership.tla` owns the
  ownership half: the displayed buffer is released only by a successor's
  retirement, and the successor may be a composed frame. Mixed composition is
  the fail-closed fallback, and a refused validating commit, a rejected real
  commit, or any prepare failure re-queues the same content composed rather
  than reaching the terminal submit-failure path. ARGB8888 is offered where the
  layer covers the whole head, because nothing is behind it there and the
  atomic test still decides.
- [x] Promote the direct-scanout run under the archive discipline.
  Direct-scanout archive `0001` binds a signed commit with the Sophia binary,
  the client whose buffer reached the plane, and both configurations the
  session loaded: thirty-eight client buffers on the plane from one validating
  commit, no test rejections, no proof disagreements, no unsupported formats,
  no fallbacks, and a clean retirement and session end.
- [x] Prove that effect or overlay activation returns a directly scanned output
  to composition without a lost or stale frame, and that later removal restores
  eligibility only through a fresh Engine proof and a fresh backend atomic
  test. Direct-scanout archive `0002` promotes it: ten direct flips, an
  overlay opened by the session's own proof control through the same
  `set_descriptor_overlay` entry the shell uses, a composed successor built
  from the client's still-held planes and retired inside the window with the
  displaced buffer's snapshot promoted before its release, withdrawal, a
  second validating commit, and flips resumed to a clean bounded completion.
  Getting there fixed three boundary defects the offline suite could not
  reach: the retained requeue sourced a never-imported renderer image for a
  direct frame, the lowering carried an eligibility verdict that stopped
  being true when it substituted the snapshot, and the conformance readers
  anchored records to the line start and so had never actually run the
  episode-order rules against decorated hardware evidence.
- [x] Measure whether a direct frame costs less than a composed one. It does,
  by roughly half at the median and by three orders of magnitude in the tail.
  Direct-scanout archive `0003` measures both populations on one head in one
  session -- direct flips outside the overlay window, composed frames inside
  it -- with the client's buffer offered to the plane in 17 microseconds at
  the median against 35 for a composed frame, and a p99 of 22 microseconds
  against 12,883. The tail is the whole finding: a direct frame's cost is
  nearly constant because nothing is drawn, while a composed frame waits on a
  renderer that occasionally takes twelve milliseconds. Submit-to-flip is not
  the point but is measured beside it to check the assumption that the
  display engine does not care how the buffer arrived: 7,972 microseconds
  against 15,099 at the median, both dominated by this host's chronic DCN32
  flip stalls rather than by anything Sophia does, and neither population is
  penalised for arriving directly.
  The stated blocker turned out to be about the wrong harness. Input latency
  needs an input proof, whose session runs xterm, which never presents a
  DMA-BUF -- but this question was never about input, and the standalone
  probe whose eligibility archives `0001` and `0002` already proved answers
  it directly. No threshold gates the values: this host stalls modesets, and
  the row asks what a frame costs, not that it cost less than a number.
  One instrumentation defect surfaced only under real evidence. The
  classifier asked the flip counter whether the export it had timed was
  direct, but a flip happens later at submit, so every export answered no and
  the direct population was left with no offer samples at all. The emitter
  then dropped the half-measured population entirely rather than reporting
  it, so the only symptom was a comparison that said no direct frames
  existed.
- [x] Add the hardware cursor plane, with the per-output KMS transaction owner
  the next row introduces. Direct-scanout archives `0005` and `0006` promote
  it in two steps. `0005` put the cursor on a plane at all: twelve moves, the
  card accepting, no failures, direct scanout undisturbed -- and exposed that
  every update was a standalone commit, with worst-case motion-to-submit at
  21 ms against the legacy ioctl's 9. `0006` runs the same sweep with the
  owner complete: twelve moves produced only eight cursor-only commits,
  because the rest rode primary commits as one combined atomic request --
  the thing the next row asks the owner to be able to do, observed rather
  than claimed -- and the worst case came down to 17 ms.
  A rejected combined commit retries with the primary alone, prepared beside
  the combined request rather than rebuilt after failure, so a cursor can
  never cost a frame; the run needed no such retry, and the counters would
  have named it if it had. `updates_primary_in_flight=0` on both archives is
  the kernel's per-CRTC serialization observed, where the legacy path
  counted fifteen.
  **Superseded detail below.** Direct-scanout archive `0005`
  runs the same twelve-position sweep as `0004` with the cursor on an atomic
  plane: the card accepted the plane, twelve moves reached it, no hardware
  failures, and direct scanout was undisturbed -- 36 flips with 26 after the
  motion stopped. `updates_primary_in_flight=0` is the kernel's per-CRTC
  serialization observed rather than assumed, where the legacy path counted
  fifteen.
  What it costs is worst-case latency: motion-to-submit peaked at 21 ms
  against `0004`'s 9 ms, which is about one frame of waiting for a busy CRTC
  and is the expected price of joining the queue the primary waits in.
  What is **not** proven, and blocks the next row: the owner never combines
  primary and cursor state in one request. Every atomic cursor update is a
  standalone commit. The machinery exists end to end -- the submit policy
  carries a cursor, the request builder applies it -- but nothing populates
  it, so `plan_cursor_commit`'s `RideNextPrimary` branch is unreachable.
  Connecting it is what would remove most of that 21 ms, since a cursor
  riding a frame that is going out waits for nothing.
  The legacy cursor continues over directly scanned frames on its own ioctl,
  which archive `0004` establishes rather than assumes: twelve cursor positions driven through the same `Pointer::place`
  entry physical input uses, 519 hardware updates with no failures, the
  cursor never leaving `legacy_ioctl`, `composed_cursor` still zero, and
  twenty-six client buffers reaching the plane after the motion stopped.
  Motion-to-submit peaked at 9 milliseconds, which is the baseline the atomic
  cursor plane has to match.
  The three earlier archives had asserted this and tested none of it: their
  cursor records read `moves_coalesced=0 max_motion_to_submit_msec=0
  hardware_updates=1`, a cursor initialized once and never moved.
- [x] Replace the bounded legacy cursor baseline only after one per-output KMS
  transaction owner can combine primary and cursor-plane state in the same
  atomic request. Retain bounded cursor-only idle work and the pointer-motion
  GLX cadence gate. The precondition is archive `0006`, where four of twelve
  cursor moves rode primary commits as combined requests; the replacement is
  a continuous-motion shakedown on the atomic path holding 57.97 fps with
  p95 16.687 ms, no cursor failures, and no commit overlapping a page flip.
  The same day's accidental legacy run is what it replaces, and is the first
  legacy evidence under continuous motion rather than twelve synthetic
  moves: 298 hardware updates, 243 of them overlapping a flip. The atomic
  path did that work in 56 commits with none -- a five-fold reduction with
  pacing intact.
  A native session now prefers the atomic plane without being asked;
  `--legacy-cursor` opts out, and the two flags are mutually exclusive. The
  ioctl is not deleted and cannot be: the startup probe decides per card and
  a refusal keeps it, which is what makes it a fallback rather than dead
  code.
  Retained, concretely: bounded cursor-only idle work is the
  redundant-commit guard plus the model's `CursorWorkBoundedByAvailability`,
  which bounds commits by CRTC availability rather than by pointer events;
  and the GLX cadence gate survives holding both paths to their own shapes.
  That gate needed repairing to survive at all -- it demanded
  `wm_policy=external` from a benchmark that had become standalone, and
  matched a resources schema three revisions old, so it could only ever pass
  against its own fixture.
- [ ] Replace full immutable CPU presentation replacement for stable
  software-rendered X toplevels with lease-safe damage generations or
  copy-on-write backing. Preserve child composition, bounded storage,
  historical-handle immutability, and exact admission extents.
- [ ] Compare identical Kitty, Firefox, resize, launch-burst, and soak
  workloads against separate XLibre+xmonad and mature Wayland-compositor
  sessions on the same hardware. Comparative results are diagnostic; Sophia's
  absolute correctness and latency gates remain authoritative.

Milestone 14 exits with bounded warmed resource counts, no steady-state
allocation growth, refresh-relative latency evidence, and no change to
Sophia's native-X authority model.

---

<!-- END IMPORTED BODY -->
