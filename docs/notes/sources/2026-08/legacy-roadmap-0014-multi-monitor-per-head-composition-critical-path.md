---
id: legacy-roadmap-0014
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Multi-Monitor Per-Head Composition Critical Path

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 1153–1320.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0013-134-prove-the-boundary-and-port-triad.md).

<!-- BEGIN IMPORTED BODY -->

The normative design, ownership boundaries, and acceptance rules live in
[Multi-Monitor Per-Head Composition](../../../multi-monitor-composition.md). This
roadmap tracks only the remaining executable slices. Completed mirror,
per-head-planning, topology-transaction, output-IPC, and authority-raster
foundation is summarized in [Roadmap
History](legacy-milestone-0029-2026-08-16-multi-monitor-per-head-composition-foundation.md#2026-08-16-multi-monitor-per-head-composition-foundation).
Detailed physical-run diagnoses remain in
<a href="../../../research-log.md">Research Log</a>.

- [x] Make accepted core X11 `PutImage` replayable in authority-owned density
  stores. Decode and retain bounded owned pixels, destination geometry, format,
  depth, byte order, GC semantics, and source generation inside X Authority;
  never expose X requests or XIDs to Engine. Raster each requested density
  independently with the document's deterministic rational-edge/area-coverage
  rules. A full opaque replacement may establish a new replay baseline and
  discard older journal commands only when doing so preserves the canonical
  protocol-visible drawable.
  Core and MIT-SHM uploads now retain bounded owned pixels behind a fail-closed
  subset gate: tight ZPixmap depth-24/32 rows, no left padding, unconditional
  GXcopy, full visible plane mask, no clip rectangles. Replay projects those
  retained 1x pixels per density as a per-channel rational area average, so a
  fully covered destination pixel keeps its source color exactly. A full-window
  qualifying upload replaces the journal as a new baseline; a partial one does
  not.
- [ ] Extend replay to cross-drawable `CopyArea` with explicit source drawable
  and generation dependencies. Preserve clipping, overlap, and GC semantics;
  reject stale, destroyed, cyclic, cross-namespace, or over-budget dependencies
  without poisoning unrelated surfaces. Until this is implemented, publish the
  canonical raster with an explicit sampled-fallback reason.
  The explicit reason is in place: a cross-drawable copy now poisons its journal
  with the `unsupported_cross_drawable_copy` cause rather than an unnamed
  fallback. Replay itself remains open, and the re-run gate's causes decide
  whether it must precede a passing unequal-density run.
- [ ] Keep authority raster storage bounded and fail visible. Cover payload,
  command-count, variant-count, and canonical-plus-derived byte limits; late
  density demand; fractional targets; baseline replacement; source destruction;
  and allocation failure. Classify fallback telemetry by cause (including
  unsupported `PutImage`, unsupported cross-drawable copy, stale dependency,
  journal capacity, backing capacity, and transform mismatch) and coalesce
  repeated warnings without hiding counts. Requirement-admission staleness is
  reported as two distinct causes, stale content generation and logical extent
  mismatch, because collapsing them hid which check a physical run had hit.
  Cause classification and coalescing are implemented: an authority-private
  cause accompanies every sampled-fallback outcome, and a bounded per-surface
  coalescer emits the first occurrence and each subsequent power of two with a
  cumulative count. Deterministic coverage exists for unsupported `PutImage`,
  unsupported cross-drawable copy, stale dependency, journal capacity, backing
  capacity, stale content generation, logical extent mismatch, absent
  canonical raster, and transform
  mismatch. Source destruction and allocation failure remain open.
- [x] Add deterministic authority regressions for the real xterm sequence:
  startup `PutImage`, later ImageText8/PolyText8 and line drawing, same-drawable
  scrolling, late 750-density demand, canonical plus derived publication, and
  generation races. Require exact-density output to differ in pixel identity
  where density differs while retaining the same logical content generation.
  Add negative controls proving an unsupported or over-budget command cannot be
  mislabeled as exact.
  The wire sequence drives opcodes 72, 76, 74, 65, and 62 in the traced order,
  then requires 750 and 1000 to publish distinct native-size authority rasters
  with zero sampled fallback. Pixel identity is proven by a source split whose
  boundary does not align with a 0.75 pixel edge, so replay produces boundary
  values absent from the uploaded palette. Negative controls cover XYPixmap at
  the wire, non-copy function, partial plane mask, clipping, absent semantics,
  journal capacity, transform mismatch, and a generation race.
- [x] Re-run the signed unequal-mode mirror gate after the replay slice lands.
  Require DP-1 to select its exact 1000-density variant and DP-2 to select a
  distinct exact 750-density variant for one common logical generation; require
  zero sampled fallback, causal plan/queue/submit/callback/retire records,
  clean suspend, zero abandoned ownership, and an archived verifier-approved
  result. Do not accept visual similarity produced by downsampling the
  canonical head.
  Attempt `0025` satisfies every telemetry condition: both heads select their
  own exact variant for one logical generation, with zero sampled fallback and
  zero stale responses. The original "positive native-size text evidence on
  both heads" clause is withdrawn as unreachable for this workload rather than
  left open, because a fixed 6x13 cell becomes 4.5 pixels at 0.75 density: no
  stem can occupy a whole pixel, so the result is soft however it is produced,
  and thresholding it crisp yields the blocky rendering the same clause
  rejects. A deterministic comparison retains the reasoning — replay keeps a
  one-pixel line fully lit where resampling the canonical raster cannot, while
  replayed and resampled bitmap glyphs land within a few levels of each other.
  Visual acceptance of native-density rendering moves to the extended
  topology below, where a window is rendered at its own head's density and
  nothing is resampled. See
  <a href="../../../research-log.md">Research Log</a> for the ink-density evidence.
- [x] Prove the same architecture for a mixed mirror-plus-extended topology
  driven through `sophia_output_v1`. This now also carries the visual
  acceptance withdrawn from the mirror gate: a window resident on the
  lower-density head must be rendered at that head's density with no
  resampling, and must read as sharp rather than soft. Unlike a mirror, this
  case is reachable, because the surface is composed for one head only.
  Prefer content with resolution-independent form for that judgement; a
  fixed-cell bitmap font cannot be crisp at a fractional ratio. The privileged output role hosted by the
  shell or selected WM process must independently select each opaque head's
  mode, scale, transform, position, and mirror membership. Ordinary
  `sophia_wm_v1` policy remains logical-output-only and receives no head or
  connector identity. Require spanning-surface coverage, complete candidate and
  rollback ownership, first-presentation publication, head-loss recovery, and
  clean teardown on the physical target.
  The Rust reference policy now hosts the exclusive output role for this gate.
  It negotiates the owner-only socket, accepts only an exact three-connected-head
  snapshot, preserves every current mode, and submits one complete candidate
  containing a two-head mirror group beside one extended group. Extra connected
  heads fail closed instead of being disabled implicitly. In the same supervised
  process, the public-policy client completes exact profile activation and
  configuration, then partitions two proof surfaces across the resulting logical
  outputs using only policy-visible geometry; connector labels never cross into
  blind policy. `tools/run_mixed_output_gate_tty4.sh` retains signed source and
  binary identity and arms the real modeset only from a recovery-safe TTY. One
  client proposal now waits behind an owner-local two-second frame-quiescence
  barrier; ordinary authority and policy intake remain queued while the native
  owner drains existing frames. The evidence verifier accepts an empty first
  DP-2 topology frame, then requires the later exact active DP-2 frame after
  blind policy partitions the two surfaces, correlated through queue, submit,
  callback, and retirement. The next physical run reached that quiescence
  barrier, then exposed a candidate-composition mismatch: provisional topology
  frames used the CPU-only lowerer even though both committed Kitty surfaces
  were retained renderer images. Candidate and rollback planning now reuse the
  ordinary mixed source set, preserving Engine membership and renderer-image
  ownership without a second DMA-BUF import. The following run rendered both
  proof surfaces on the initial large output and left the two initial secondary
  outputs black, but never submitted an output proposal: the proof client waited
  for a redundant scene echo after its committed two-surface proposal and timed
  out. It now starts directly from that committed proposal. Restart acceptance
  is also paused across spawn-to-PID reauthorization, closing the secondary
  unauthorized-peer race exposed by the timeout. The next run reached candidate
  renderer preparation and showed that the new mirror member did not own the
  retained renderer-image IDs created on the original large head. Topology
  preparation now realizes retained images per physical head by restoring a
  compositor-owned snapshot from a live donor before it queues candidate or
  rollback work; a missing donor rejects before KMS. Unchanged preparation
  progress is no longer logged on every owner turn. The following run committed
  the physical candidate and both first presentations, then exposed a stale
  private raster journal when post-publication density demand arrived. Standard
  pixmap and DRI3 Present now invalidate semantic replay, so stale extent or
  unsupported-command demand becomes a bounded sampled fallback instead of an
  X Authority process failure or falsely exact pixels. Signed attempt
  `3d19e2e67cfe2e43eb643d219be11a3251fe7176` then passed the physical runtime,
  archive verifier, and visible-pixel acceptance: two logical outputs settled,
  the extended head's exact draw retired, topology input quarantine released,
  health and cleanup were clean, and the operator confirmed matching mirror
  content plus sharp extended text. Signed head-loss/return source
  `66bc0dd71a40e249eb00cd98f6080cf0f6aa9c54` then passed the physical
  `3 -> 2 -> 3` cable gate: both kernel notices produced changed,
  generation-advancing publications, Hagia policy commitments, later
  presentations, released input quarantine, and clean topology and session
  teardown. That closes the combined item.
- [x] Run one black-box conformance corpus against the Rust reference WM,
  Hagia, the X11 bridge, and the independent C client. This is draft boundary
  evidence while the Triad port is incomplete; it does not publish or freeze
  `sophia_wm_v1`.
  The authenticated behavior host now runs the Rust reference, independent C,
  immutable revision-3 C snapshot, Hagia, and configured public xmonad bridge
  through the same sequential eleven-scenario corpus:
  constrained single output, two-output partition, output loss/migration, and
  generational return, followed by an ordered focus action, timeout discard,
  and successful post-timeout recovery. Stale-scene and invalid-candidate
  outcomes are also discarded before later successful cycles. Rust, C, and
  Hagia additionally run the corpus across two supervised processes. The real
  configured xmonad bridge negotiates profile activation and its action catalog
  over the public wire, then passes the same scenes across five epochs covering
  normal replacement and each noncommitted recovery. The candidate archive
  retains its own C codec, client, schema, and fixed digests; its permanent
  compatibility status begins only when the remaining physical ledger row
  closes and revision 3 freezes.

<!-- END IMPORTED BODY -->
