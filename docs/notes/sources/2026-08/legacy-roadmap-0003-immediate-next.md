---
id: legacy-roadmap-0003
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Immediate Next

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 286–437.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0001-current-position.md).

<!-- BEGIN IMPORTED BODY -->

Ordering for the next few sessions. Each row points at where its detail already
lives rather than restating it; this is a priority index, not a second roadmap.

1. **Complete.** The work-area reservation coordinator is in production.
   Signed Hagia archive `0007` closed it on signed source
   `4e54dfc886d0a96737f3a1fcd3d0bbe4a8ca9edd` and signed Hagia source
   `8a38033aae2bd4470f66e474dcf4447f482df87f`: three claims admitted and
   presented, two reductions and two releases, no refusal, the claim retained
   across the shell's death and re-made at connection epoch 2, zero protocol
   errors, and clean teardown. Previews, icons, MRU policy, and generic
   textures remain out of scope.
   The substrate is complete and proven offline. `sophia_shell_v1` candidates
   carry an optional edge claim in bytes the pre-reservation encoder already
   wrote as reserved zeroes, so older frames decode unchanged; Rust, C, and Nim
   move together and a Nim test pins its encoder to the Rust-generated golden
   byte for byte. `ShellWorkAreaCoordinator` admits a claim against the realized
   topology, names every refusal, and reduces nothing until its bundle commits;
   `reduce_output_work_areas` now subtracts shell bands and X-side struts in one
   reduction. `check_shell_protocol.sh` drives the real Nim shell through the
   real coordinator and reports `reservations=1`, taking a 1440-tall output to
   1412 only after commit and restoring it on withdrawal.
   The live wiring is in place. The session's shell owner holds the
   coordinator, admits each candidate's claim against the realized topology
   before the candidate is prepared, commits it in the same step its pixels
   present, and retains it across disconnect; the WM session mirrors the
   committed bands and reprojects only when they change.
   The depth remains a configuration decision, validated against the wire's
   own reservation maximum and refused outright when no shell is enabled. The
   ordinary compiled profile no longer asks the window switcher to reserve a
   panel: a transient switcher stream is not a persistent panel. The signed
   archive remains the regression for an explicitly configured reservation.
   An earlier entry here named an
   environment variable instead; that variable existed only in the offline
   conformance host, so no session could set it and no session could raise a
   claim. The profile key is what makes the coordinator reachable.
   Two corrections to what this row previously claimed. The claim is not a
   persistent bar: one shell connection carries one candidate stream and one
   visible state, so the strip lives exactly as long as the switcher is
   visible. A panel that persists independently needs a second shell role and
   is separate work. And losing the shell does not restore the work area --
   the presented claim is retained beside the retained pixels, because growing
   the area while nothing can present into the strip is the incoherent desktop
   `ShellWorkAreaCoordination.tla` forbids. The guide and verifier now prove
   the retention rather than a restore.
   The guide, matcher fixture, and verifier carry that evidence, with negative
   cases proving each check fires. One fidelity note from the archived run: a
   public-policy session emits no `sophia_live_work_area status=applied` lines
   at all, so the reduction is evidenced by the shell's own
   `reservation_reduced` records rather than by a work-area line the fixture
   had briefly invented.
2. **Complete.** The checked-in Hagia
   command and pointer surface is closed by executable catalog coverage plus
   its deterministic and physical gates. Trusted one-shot launch placement and
   frame-fed atomic output activation are implemented and offline-proven, and
   the shared restart corpus is complete. The signed-harness candidate now has
   a one-shot proof control at the exact boundary after every KMS card accepts
   the startup candidate but before installation or publication. Its TTY4
   runner, paired evidence/archive verifiers, reference profile, and mutation
   fixtures pass offline. The first explicitly authorized run passed the atomic
   hardware preflight, then Sophia exited before either phase could be archived;
   the runner's error trap incorrectly erased that phase log. Its signed
   successor retained the retry and proved the whole success-side KMS apply,
   first presentation, frontend publication, and local commit. The session then
   rejected Hagia's configuration because the output-only runner omitted browser
   operation slot 2 while Hagia advertises all four session-operation slots. The
   runner now aliases that unused slot to its existing terminal application and
   has a regression requiring slots 1--4. The next signed run passed the complete
   success phase and reached the exact forced rollback, which restored and then
   presented DP-2 correctly. Later scene churn nevertheless submitted two more
   byte-identical DP-2 frames because retained-scene suppression ignored
   rendering, submitted, and displayed ownership; the last redundant flip lost
   its callback before the text proof. The reducer now suppresses only when the
   newest owned logical scene matches, with pending/rendering/submitted/presented
   and newer-different precedence regressions. Signed source
   `870ba46ae231081220b982ecc3a5a95517df7a90` then passed both phases, and
   frame-fed archive `0001` independently verifies the bound success/rollback
   pair, physical input, and clean teardown.
   Generic switcher archive `0006`, Tier-0 indicator archive `0005`, and
   reservation archive `0007` already close their retained slices. MRU policy,
   filters, previews, icons, persistent panels, Janet, broad portals, watched
   reload, and other excluded behavior remain explicit post-freeze work.
3. **Complete.** API v7 and Engine-owned workspace policy are removed. The
   public Hagia, xmonad adapter, archived-client, and protocol gates remain
   green. Row 4 promotes their schema-5 installed successor.
4. **Complete.** Promote the next schema-5 successor through `Sophia Hagia
   Promotion (Packaged Default)`. The first intended login instead archived
   attempt `0006` from old schema-3 release `0.1.0-66a279286bdd`; its emergency
   exit is recovery evidence, not a promotion result. Installation recovery now
   verifies and re-activates an already-present immutable release, repairs
   operator links and greetd entries, and preserves rollback. That repair
   activated `0.1.0-50c7cb2d2d54`, and its packaged-default attempt `0001`
   completed one normal Logout with clean lifecycle health, but the operator's
   `glxgears` window did not present. Its 300-by-300 first DMA-BUF arrived after
   policy had staged a 1278-by-1424 launch layout; the pixel-silent surface was
   correctly deferred from that epoch, but candidate selection neither primed
   its admission extent nor queued a successor while any epoch was pending.
   Candidate selection now primes its measured safe extent and queues recovery
   unless the current epoch owns that exact surface and size. Signed successor
   `66792329d90d64e26af839dfe494c74d94323c6a` produced independently verified
   packaged-promotion archive `0002`: the 300-by-300 first GLX candidate primed,
   armed, committed, and retired, its 1278-by-1424 standing target committed,
   772 GLX presentations retired, and normal Logout finished with zero protocol
   errors and clean lifecycle health.
5. **Complete.** Milestone 14's bounded visual-retirement model and the
   retirement-fed three-slot implementation are complete, and the three-slot
   boundary is promoted. Public snapshots source surface state from
   X-authority lifecycle facts rather than raster generations; signed source
   `c681f762` confirmed that ordinary repaint no longer causes the
   stale/rebuild storm.
   Signed native archive `0001` closed the row on signed Sophia source
   `05d98e44981f5086fc8d2bd3ee4580944029a952` and signed Hagia source
   `9c9a59061fd0d8e88310b764f7dd240e729fb035`, against Hagia's tracked default
   profile. Hagia was the first proof client of the Sophia WM and shell
   protocols with no xmonad bridge in the session. The bounded workflow ran in
   order: three terminal launches, each committing its layout before the next
   was requested and reaching an admitted surface; one visible `Super+J`
   committing focus to a surface; one close; and a normal logout. Committed
   layouts moved 0 to 1 to 4 surfaces and back to 3 after the close.
   The three-slot evidence is exact. Renderer-worker requests settled as
   `worker_requests=263 == worker_completions=263 + frame_slot_deferrals=0`,
   with `frame_slot_stale_releases=0` and `frame_slots_leased=0` at
   completion: no stale release and no leaked lease. The aggregate watermark
   of 6 is three slots on each of two presented heads, both reaching full
   occupancy.
   Everything else the row required is in the archive: separate protected
   broker and shell admissions with clean ready-to-stopped lifecycles, the
   34-event physical text proof, 2 ms session-control queue dwell and 1 ms
   ack against a 100 ms budget, drained native scanout with no abandoned
   scanouts, clean session/topology/cleanup health, zero unexpected protocol
   errors, and exact TTY recovery with `emergency=false`. Four stale policy
   responses were ordinary scene races; every one re-armed and committed.
   Bounded buffer-age damage history is now the active step.

Completed since this ordering was written. The Pnut Landlock empty-allowlist fix
was submitted upstream as [mikedanese/pnut#3](https://github.com/mikedanese/pnut/pull/3)
from signed commit `70c8ea8a9fb419ca9808caae4615d6bbeb5dd973`; see
[`docs/pnut-evaluation.md`](../../../pnut-evaluation.md). Protection domains are now
required where roles are admitted rather than only where domains are
constructed: the metadata-bearing role sockets refuse a supervised PID, refuse
an expected peer identity at bind time, and admit only the launch evidence
carrying their domain role. A caller that builds no domain no longer gets
admission without a boundary and without a complaint. See the ratified entry
below.

Not yet: making the protection domain the default rather than opt-in. It reads
as the obvious next step now that the metadata-bearing roles require one, and it
still owes a deliberate decision about hosts that have no `bwrap` rather than
arriving as a side effect. The blind spatial-policy and output roles are what
that decision governs; they still admit on a supervised PID today.

<!-- END IMPORTED BODY -->
