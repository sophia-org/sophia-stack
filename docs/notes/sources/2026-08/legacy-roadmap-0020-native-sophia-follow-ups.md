---
id: legacy-roadmap-0020
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Native Sophia Follow-Ups

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 2376–2563.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0017-post-promotion-capability-roadmap.md).

<!-- BEGIN IMPORTED BODY -->

- [x] Ratify target-resolved input as a pre-schema prerequisite. The contract
  resolves against per-output presented snapshots, admits targets only inside
  their owner's visible allocation, gives non-recyclable authority/session
  identity and device/contact-bound per-seat capture, and paces normalized
  continuous values. Exceptional coordinates require an independently issued,
  revocable region/precision/rate capability. The target, pacing, and
  cross-authority arbitration TLA+ models precede any `sophia_shell_v1` schema
  or runtime work; see `docs/target-resolved-input.md`.
- [x] Admit the first complementary architecture-model gate. Alloy checks
  bounded role/namespace/portal and presented-target topology; Z3 checks target
  geometry/disclosure arithmetic and schema-generated `sophia_wm_v1` wire
  bounds. Every promoted rule retains a satisfiable negative control. Keep
  temporal ownership in TLA+ and keep Spin, dependency policy, and fuzzing
  deferred until they have concrete retained artifacts; see
  `docs/architectural-alignment.md`.
- [x] Ratify the WM/shell hardening prerequisites. Blind WM policy cannot share
  a protection domain with metadata-bearing shell, broker/portal, or frontend
  roles; opaque actions bind issuer/recipient epochs, operation class, and
  target generation; and a tier-1 shell reservation, derived work area, and WM
  projection promote as one exact presented bundle. Alloy and
  `ShellWorkAreaCoordination` retain focused negative controls. This is target
  architecture, not a shipped shell or sandbox.
- [x] Enforce protection domains in session supervision before admitting a
  metadata-bearing shell. The Bubblewrap backend rejects blind-policy domains
  composed with shell, broker/portal, or frontend roles; launches public Hagia
  and the production metadata broker with cleared environments, stdio-only
  inherited descriptors, private PID/user/network/IPC/UTS/cgroup namespaces,
  no ambient home or temporary tree, and only explicit read-only socket/profile
  paths plus Hagia's private writable checkpoint directory. Exact host peer PID
  is recovered for role-socket admission, and the protected broker smoke is the
  executable negative control. Bubblewrap 0.11.2 is now a checked Hagia install
  prerequisite. The live Hagia Shell now receives that separate supervised
  role/domain. Compiled-profile enablement is now complete; its physical proof
  remains a later gate, and this checkbox alone does not claim it.
- [x] Require that domain where roles are admitted, not only where domains are
  constructed. The shell and metadata-broker sockets refuse a supervised PID,
  refuse an expected peer identity at bind time, and admit only the launch
  evidence their supervisor produced, which must carry that role's
  protection-domain role. The metadata broker transport publishes no PID-only
  call, so a caller that spawns it unprotected fails to compile rather than
  admitting quietly. The blind spatial-policy and output roles are unchanged and
  still admit on a supervised PID; requiring a domain everywhere remains the
  separate `bwrap`-availability decision. Evidence is a passive record whose
  fields any caller can write, so this closes silent omission rather than
  deliberate misreporting.
- [ ] Implement issuer-scoped action-capability validation and the atomic
  shell-reservation/work-area/WM coordinator with the eventual shell schema.
  Preserve the prior complete presented bundle on ordinary failure and keep
  lock/session security takeover independent of shell or WM acknowledgement.
  Experimental `sophia_shell_v1` revision 1 now carries nonzero, nonreused
  broker action grants through exact broker, revocation, recipient, target, and
  descriptor generations; its model and transport revoke interaction on shell
  loss while retaining prior pixels. Issuer-side live dispatch now revalidates
  the broker grant before submitting an ordinary WM focus request. The atomic
  reservation/work-area/WM coordinator remains open.
- [ ] Repair native application input before shell coexistence.
  - [x] In the installed primary-output pointer domain, derive hit-test layers
    from the immutable output-frame snapshot only after accepted page-flip
    retirement. Committed/submitted moves, removals, and stacking changes no
    longer become selectable before their pixels.
  - [x] Introduce output-local pointer coordinate domains and per-output
    presented interaction epochs rather than merging independently retired
    heads into one global projection.
  - [x] Advance Sophia `SurfaceId` generations when a client reuses an XID and
    retire the exact frontend route on successful surface removal. Frozen or
    deferred generation-N input cannot resolve to generation N+1.
  - [x] Isolate a non-reading X client's private input queue. Saturation now
    removes that endpoint's sender set, rejects tracked delivery, and leaves
    the shared frontend broker available to healthy clients.
  - [x] Revalidate deferred pointer-focus handoffs before release. Every exact
    generational target must remain in the last-presented input projection and
    frontend route table; otherwise the complete buffered sequence is dropped.
  - [x] Preserve ordered client keyboard input across asynchronous focus
    acknowledgement without retargeting it. Reserved controls resolve first;
    the remaining keys retain their exact seat, generational surface, order,
    and libinput timing in a capacity- and timeout-bounded handoff. Focus,
    target, topology, or security invalidation drops the complete sequence.
  - [x] Turn ordinary and passive-grab pointer sequences into exact
    Engine-visible profile-scoped leases with ordered release acknowledgement.
    VT and seat transitions advance a shared epoch, clear frontend active
    ownership, and reject queued or frozen old-epoch input without waiting.
  - [x] Reduce client-initiated explicit `GrabPointer` and admitted XI grab
    requests into the same Engine lease handshake. The passive bounded control
    path prepares before frontend mutation, activates before success is
    exposed, orders release, rejects saturation, and shares the existing
    disconnect, presentation, admission, and control-epoch revocation path.
  - [ ] Bind lock and future security-authority takeover to the established
    epoch barrier.
- [ ] Add bounded policy interactions for move, resize, drag, and scrolling.
  Engine owns hit-testing, grabs, raw physical input, cursor state, and
  animation; Hagia receives only opaque targets and reduced geometry updates.
  Revision 3 now permanently fixes `Drag = 3`, `Scroll = 4`, and the in-place
  `interaction_axis` values (`None = 0`, `Horizontal = 1`, `Vertical = 2`) in
  Rust, generated C, and Hagia's independent Nim decoder. The codecs reject
  ambiguous geometry/axis combinations. Engine-captured move/resize now send
  ordered Begin/Update/End values; only the latest matching queued Update is
  retained behind an in-flight request. Output-topology, VT, and seat security
  transitions clear capture, purge its queued values, and prioritize Cancel;
  policy restart clears capture before the next physical input drain. Hagia
  applies continuous geometry and treats Cancel as a spatial no-op. Live
  drag/scroll production remains open, so this row is intentionally not complete.
- [x] Make exact desktop-profile activation mandatory for every public
  `sophia_wm_v1` launch. The pre-graphical gate prepares all seven authorities,
  activates the six Sophia-local owner slots, admits Hagia against the exact
  Policy fragment, and promotes only after its matching completion. Timeout,
  disconnect, rejection, or any local failure rolls the generation back before
  graphical construction. The old opt-in switch remains only as a harmless
  compatibility argument. This closes startup visibility; watched reload still
  requires its separate message-53+ visibility and durable-recovery protocol.
- [ ] Model and publish `sophia_shell_v1` through the same formal, schema, C
  client, and permanent-compatibility process. Keep its endpoint and
  capabilities separate from `sophia_wm_v1`. Begin experimental modeling and
  the Hagia Shell port before the WM freeze so retained workflows can falsify
  both contracts. Derive the vocabulary from a driving client with a retained
  scene graph rather than from first principles; see
  `docs/sophia-shell-v1-direction.md`. Revision 1 is now published as an
  explicitly experimental descriptor-switcher schema with strict Rust codecs,
  a generated golden/malformed corpus, an independent C proof client, and a
  23,582,243-state lifecycle check. Permanent compatibility, broader display
  lists, reservation coordination, and signed installed evidence remain open.
- [x] Ratify one developer-facing Sophia native protocol family rather than a
  collection of role-specific transports or language tooling. The common
  contract now owns the envelope, negotiation, capabilities, epochs, bounded
  complete transfers, explicit outcomes, recovery, extension discipline,
  source hierarchy, and independent-implementation requirement. WM, shell, and
  output retain separate endpoints, disclosures, schemas, and stability status;
  see `docs/sophia-policy-ipc.md`.
- [ ] Audit `sophia_wm_v1`, `sophia_shell_v1`, and `sophia_output_v1` against
  the common family lifecycle. Align hello/welcome negotiation, effective
  bounds, capabilities, epochs, transaction identity, complete-transfer
  behavior, outcomes, recovery, and extension handling. Every intentional
  difference must be role-specific and documented rather than an accidental
  transport fork. This audit does not block the active Milestone 14 direct-
  scanout proof, but it gates broad shell vocabulary and shell stabilization.
- [ ] Add one family-level conformance entry point that invokes each role's
  retained valid, malformed, codec, and lifecycle corpus. Every stable role
  must retain a complete independently implemented non-Rust lifecycle proof;
  shell stabilization specifically requires the existing C proof plus Hagia's
  independent Nim client. The proof must be implementable from normative prose
  and the checked-in schema without Sophia Rust crates, generated bindings,
  generators, or implementation-source inspection.
- [x] Ratify visual styling and compositor-effect extensibility without moving
  compositor authority out of Engine. WM and shell authors own role-appropriate
  visual policy; public protocols carry bounded semantic intent rather than
  shader code; Engine owns validation, animation timing, damage, rendering, and
  presentation. The first provider boundary is a private version-coupled Rust
  trait for separately maintained modules linked into the trusted renderer
  build and selected by the immutable packaged profile. Runtime shared-library
  loading and a sandboxed effect host remain evidence-gated later designs; see
  `docs/compositor-graphics.md` and `docs/sophia-shell-v1-direction.md`.
- [ ] Before broadening the shell schema, model effect capability admission,
  bounded parameters, generation supersession, Engine-clock cancellation,
  provider absence or failure, deterministic fallback, and atomic multi-head
  presentation. Retain negative controls that expose unnegotiated capabilities,
  stale animation, missing fallback, unequal-head semantics, and an effect that
  incorrectly survives provider or authority revocation.
- [ ] Implement one protocol-neutral Engine effect registry and the private
  build-linked provider interface, then prove backdrop blur as a scene-sampling
  effect and a focus transition as Engine-clocked animation. Keep provider code
  outside the core renderer module, bind it to the installed release identity,
  add deterministic reference lowering and native shader/pixel/damage gates,
  and prove that effect activation disables direct scanout without stale pixels.
  Only then admit capability-gated semantic effect intents to the experimental
  shell contract or a future outbound-gated WM extension; do not reopen frozen
  `sophia_wm_v1` revision 3. Require the Rust and independent C/Hagia corpora and
  one independently packaged provider proof before considering a stable
  provider interface.
- [ ] Settle the remaining display-list vocabulary before schema work. Admit
  generic target regions and a desktop-background surface class, evaluate analytic
  screen-corner and indeterminate-progress primitives, and refuse per-widget
  visual novelty in favor of client-rasterized textures. Record the damage,
  bandwidth, and power cost of that texture path before relying on it.
- [ ] Build `hagia-shell` as one ordinary separately authorized shell client
  for tabs, overview, switchers, previews, and other visible furniture. Shell
  metadata must never leak into Hagia's blind spatial-policy projection. The
  standalone Nim executable and reducer now pass the shared corpus and a
  protected title-only cross-process proof without linking Hagia policy.
  Shell-enabled live launch, shortcut admission, exact activation, issuer-side
  dispatch, withdrawal, reconnect, explicit X-grab arbitration, and compiled
  profile enablement are implemented. Installed evidence and richer furniture
  remain open.
- [ ] Add trusted classification, launch, lock, capture, output, and transfer
  services through brokers, session capabilities, and portals. Hagia may
  request opaque actions but may not receive executable paths, client
  metadata, portal payloads, or compositor authority.

<!-- END IMPORTED BODY -->
