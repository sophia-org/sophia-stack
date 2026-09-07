---
id: legacy-milestone-0028
date: 2026-07-09
recorded_date: 2026-07-09
date_basis: first-heading-commit
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
date_commit: 2808ce2d28bf9c41887b73e50adcd32f3db50ac4
committed_at: 2026-07-09T19:16:11-04:00
---
# Completed Milestones

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 926–1019.
Date from the first addition of this heading in commit `2808ce2d28bf9c41887b73e50adcd32f3db50ac4`
(2026-07-09T19:16:11-04:00); it does not date every event or later edit.

<!-- BEGIN IMPORTED BODY -->

- [x] Milestone 4 X11 buffer and presentation semantics: standard DRI3 1.2 and
  Present feed renderer-private DMA-BUF/fence ownership, mixed CPU/GPU
  composition, Engine-prepared commits, KMS page-flip completion, Complete
  before Idle, and exact teardown. Retained paired X13 software and Vulkan
  evidence passes controlled acquire delay and rejection recovery without a
  private presentation extension.

- [x] Protocol-neutral authority boundary: Engine routed input now targets
  `SurfaceId`, visual layers carry `AuthorityLocalId`, and Engine source has no
  XLibre, X-window, Wayland, Smithay, or Kitty types. XLibre wire encoding is
  confined to its opt-in historical adapter.
- [x] Sophia Wayland Authority foundation: a frontend-only Smithay authority
  owns private sockets, client namespaces, compositor/xdg resources, ordered
  surface reducers, SHM snapshots, output advertisement, seat delivery, frame
  callbacks, buffer releases, and bounded DMA-BUF negotiation.
- [x] First native Kitty transaction proof: Kitty 0.47.4 ran with `DISPLAY`
  removed and software GL, submitted 16 changing nonzero SHM frames through the
  Sophia Wayland Authority, and completed without an X server process.
- [x] XLibre runtime retirement: release builds and the installed launcher use
  native Wayland; the XLibre bridge has no live feature and is isolated under
  `research/xlibre`, outside the workspace and production dependency graph.

- [x] Pointer and multi-output presentation: QEMU proved physical keyboard and
  pointer routing, independent content on two KMS outputs, per-output
  page-flip pacing, and clean retirement. DRM VRR discovery and fullscreen
  policy are implemented; activation proof remains deferred for capable
  hardware.
- [x] XLibre Kitty compatibility proof: a real Kitty X11/GLX client produced
  readable pixels through software GL and XComposite/MIT-SHM capture, accepted
  physical keyboard input including terminal navigation keys, met the bounded
  presentation-latency gate, and recovered successfully through the independent
  Ctrl-Alt-Backspace guard.
- [x] Generic legacy-WM bridge core: opaque layout snapshots, validated Engine
  commits, resizable Xterm transactions, configure acknowledgement, focus, and
  injected-input pixel change passed headless coverage. Remaining dedicated-TTY
  and second-WM demonstrations are deferred rather than architecture blockers.

- [x] Added the first `sophia-live-session --terminal=xterm` one-shot bootstrap
  around xterm authority transactions and deterministic composition lifecycle.
- [x] Split X11 Authority socket binding and serving into reusable one-client
  and persistent sequential entry points with authority state shared across
  accepted connections.
- [x] Proved the xterm request stream reaches committed drawing transactions;
  this evidence is now classified accurately as a transaction proof rather
  than an inspectable-pixel proof.

- [x] Engine-centered authority reframe: README, architecture docs, atomic
  rendering invariant, and XLibre prototype/reference status.
- [x] Data-oriented design and style rules, including domain-first file
  cohesion guidance.
- [x] Phase 0-2: repository shape, Rust skeleton, protocol/data model, and
  headless engine.
- [x] Phase 3-4: XLibre mirror probe, XComposite/Damage capture, CPU readback,
  and first X11 surface in headless frames.
- [x] Phase 5-6.5: blind WM protocol, bounded IPC codec, external WM demo,
  routed-input XLibre patch, and smoke/stress coverage.
- [x] Phase 7-8: portal reducers, compositor chrome action reducer, and polite
  X11 close helper.
- [x] Phase 9: process supervisor, restart policy, WM restart adapter, and last
  committed layout cache.
- [x] Session runtime assembly: runtime reducer, bounded observation intake,
  headless session driver, broker health/control packets, and live X/WM socket
  smoke.
- [x] Portal execution prototype: X11 `SelectionRequest` conversion, native
  denial, approved bounded text handoff, and live X smoke.
- [x] Portal request/grant lifecycle: bounded pending and active state,
  deadlines, completion, expiry, disconnect and executor revocation,
  source-generation validation, and broker-restart invalidation.
- [x] Native same-namespace selection handshake: per-client routing for core
  `SelectionRequest` and restricted `SelectionNotify` SendEvent, with a
  two-client socket proof and connection-local event sequences.
- [x] Protocol-neutral authority transactions: `AuthoritySurface`,
  `SurfaceTransaction`, readiness states, and committed surface projection into
  renderable layers.
- [x] Sophia X Authority design: namespace-scoped resources, event
  subscriptions, synthetic lifecycle, drawing updates, and selection portal
  conversion.
- [x] Sophia X Authority v0 runtime: internal request/response packets, bounded
  codec, reducer-backed runtime, Unix socket helper, and
  `x-authority-runtime-smoke`.
- [x] Sophia X Authority X11 wire start: setup parser, setup success/failure
  encoders, first core request decoder, minimal property table, and setup
  socket smoke.
- [x] Sophia X Authority client-visible output: bounded X error/event records,
  `ConfigureNotify`, `MapNotify`, `PropertyNotify`, `SelectionNotify`, and
  setup/create/map socket smoke.
- [x] Future Wayland Authority boundary documented as a later protocol
  authority, not the architectural center.
- [x] Backend skeletons: frame clock, renderer/import abstraction, DRM/KMS
  discovery, libinput polling, physical input routing, and page-flip timing
  seams.

<!-- END IMPORTED BODY -->
