---
id: qfg6mjp2
date: 2026-09-09
kind: investigation
status: investigating
tags: [investigation, rendering, validation]
---
# Atomic test history is not alternate-layout flip proof

## Question

What can Sophia prove before advising an X11 client to reallocate a buffer?
The [universal negotiation plan](../plans/6tewvlbh-universal-device-negotiation-across-sophia-clients.md#suboptimalcopy-gate)
requires more than a format-table mismatch or a failed atomic test. A different
advertised allocation must permit a flip with all other eligibility conditions
satisfied. The native Wayland feedback model supplies allocation preferences;
Present SuboptimalCopy additionally makes a claim about a completed copy.

## Source and physical evidence

The candidate starts from signed commit `4b00c790`. Both local agents audited the
production path. Claude's read-only XLibre reference audit was supplied through
Herdr; an additional implementation-review message was rejected by automatic
approval review, so local agents reviewed integration instead.

`exporter/discovery.rs` preserves `direct_scanout_tested` across consecutive
eligible direct frames. It resets after a composition, refusal or disabled
policy; it does not compare the next buffer, modifier or cursor state. This
saves a TEST_ONLY ioctl in steady state. Every actual submission still performs
a real atomic commit and can fall back. The cached boolean cannot prove that a
particular candidate was tested.

`native_scanout/prepare.rs` previously reduced every test failure to WouldBlock
or Rejected, discarding the raw errno. Real-commit errors still have that reduced
contract. A real commit may also retry without its cursor and add an out-fence
pointer after request lowering. Neither change was part of the earlier test.
The [kernel atomic-check contract](https://www.kernel.org/doc/html/latest/gpu/drm-kms.html)
allows failures from state constraints, memory allocation and interrupted or
restarted operations; the commit step can still fail after successful checking.
Preserving errno improves diagnosis without making it a layout verdict.

The native XLibre reference is `~/src/xserver` at
`56be9f4320ef121dc5d4bc40a6365d995512d3bc`. Its modesetting
`present.c::ms_present_check_unflip` uses the current buffer format/modifier and
`drmmode_is_format_supported`; that predicate consults per-CRTC IN_FORMATS or
IN_FORMATS_ASYNC tables. It performs no alternate-framebuffer TEST_ONLY. The
generic `Xext/present/present_scmd.c` continues checking geometry and redirection
after a buffer-format refusal before preserving that reason. This ordering is
useful, but table membership alone does not prove kernel acceptance. Sophia
retains the stronger proof gate rather than importing the heuristic.

A read-only libdrm query on 2026-09-09 found **11 XRGB8888 modifiers on each of
two active primary planes**, including LINEAR. These are card0 plane53/CRTC84
and plane59/CRTC80. The query reads the same IN_FORMATS source consumed by
`preferred_xrgb8888_scanout_modifiers_for_selection`. It establishes that the
current active-plane tables are nonempty, not that a particular live window
received every modifier or that an alternative allocation would pass TEST_ONLY.
The probe did not acquire DRM master, submit an atomic request, change focus or
alter the screen.

Evidence is retained in `.artifacts/t070-test-evidence/`: `plane-formats.c`, its
binary, `live-plane-formats.jsonl`, source hashes, candidate manifest and check
logs. The installed session was not replaced by this candidate.

## Implementation

Owned primary-plane request lowering captures at most 32 canonical
object/property/value rows inline, sorted with the DRM request's last-write-wins
semantics. The exact same raw values feed both the kernel request and the
collector. Overflow invalidates only the evidence; it never truncates a request.
Raw request constructors remain unproven. A comparison permits only a different
primary framebuffer value with all other captured properties and effective
flags identical. This comparison describes requests, not driver causality.

The detailed TEST_ONLY validator retains the actual error kind, raw errno,
request flags and canonical properties. It issues one ioctl and returns the
same affine prepared owner, still owed submission or cancellation. Rendered and
tracked submission results carry Some(report) only for an actual test; cache
bypass, pending submission and preparation-only results carry None. A passing
test remains recorded if the subsequent real commit fails, rather than being
rewritten as a failing test.

`sophia_live_atomic_test schema=1` records output, scene generation, test status,
errno, request scope and policy flags at the test boundary. The session
sanitizer preserves that bounded vocabulary without admitting resource payloads.
No test ioctl, pixel copy, framebuffer allocation or property-discovery call was
added to the steady-state path. Canonical capture uses fixed-capacity inline
storage; diagnostics are emitted at actual tests, not cached frames.

## Validation and remaining limits

Focused tests cover canonical primary/cursor/VRR values, signed coordinates,
changed geometry/cursor/framebuffer/mode/flags, raw unknown requests, duplicate
property replacement and capture capacity. Validator tests cover success,
EINVAL, EBUSY, EIO, EAGAIN and non-OS errors while retaining exactly-once cleanup.
Integration tests distinguish an actual test from the next cached buffer,
pending submission and refusal, and verify error propagation into tracked
results. Diagnostic tests prove approved fields survive and invalid or private
values do not.

`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` passed with exit0,
including all-feature tests and clippy, source-layout checks, conformance,
20 archive fixtures, buffer-age equivalence and real GLX/EGL first-frame and
pixmap-export pixel checks. After adding one further real-commit-history
assertion, the complete native feature target passed **281/281** and formatting
remained clean. Production source hashes are identical to those checked by the
full gate. `candidate.json` separates those hashes from the final test/doc
updates. No install, session restart or physical flip test was performed.

This is evidence infrastructure, not an alternate-layout proof. It does not
retain both candidate resource owners through a differential test, identify a
sole layout blocker, or bind an alternative to current device/topology/Present
identity. Request evidence does not include kernel state omitted from the
request, and cannot be reused after a device or card-state change. A completed
compositor copy may differ in geometry, composition, synchronization or device;
it is not a substitute. Direct scanout also remains subject to its existing
policy and startup gates. SuboptimalCopy stays disabled and the original
normal-unmodified-client acceptance requirement is unchanged.

## Frame correlation and plane snapshots after `8c952695`

The follow-up separates capability observation from proof. One native plane
blob read feeds a bounded strict snapshot for XR24/AR24 and the unchanged
legacy allocation-preference reduction. Unknown, unsupported and supported are
distinct results. Unsupported versions, malformed extents or record masks,
implicit modifiers and capacity exhaustion never publish a partial negative
answer. Each native head retains the result; ordinary frame submission and
DRI3 preference queries perform no additional property discovery.

The native reference is Smithay `13738f8f2cc18224c229e7e8309ccdaa34e92e2a`,
`src/wayland/dmabuf/mod.rs`. Its feedback builder keeps a main-device sampling
tranche alongside ordered preferences, constructs an immutable shared table and
publishes the complete feedback before `done`. Sophia borrows the separation
between stable usable capabilities and advisory preferences at its existing
frontend/native boundaries. No Wayland server dependency or application
classification is introduced.

Worker submission now captures request, scene/head/output trace and composition
verdict. The worker derives returned correlation from the owned input frame;
the facade checks that result against the submission. The returned lease, export
and prepared scanout preserve it. A newer pending frame is not consulted when
attributing the older result. Unknown/raw exporters carry no correlation, and
normalization clears it when a valid descriptor or owner is absent. The worker
request counter refuses exhaustion instead of saturating into repeated identity.

This review also found a separate release defect: stale exported results were
discarded without returning their slot, and a result arriving after hard-stall
quarantine was not drained. Rejected exported results now owe their exact
output/lease/slot release. A quarantined poll performs bounded draining; a full
command queue retains one release for retry before consuming another result.
Accepted-lease Drop retains its existing release behavior; this change does not
claim to repair every possible release-queue saturation path.

The lifecycle audit found no in-place card or plane replacement in the current
topology planner. It preserves connector/CRTC/plane handles while changing mode
and logical binding. Seat recovery constructs a new native owner and rereads
its tables. Differential proof nevertheless needs stricter invalidation:
topology rollback restores the previous target generation, so numeric equality
alone has ABA. Its deadline must also be independent of `frame_offered_at`,
which fallback and pending-frame replacement restamp. The source FD owner and
current-state comparison remain the next implementation gate; neither this
snapshot nor a successful composed export authorizes SuboptimalCopy.

The follow-up based on `8c952695` passed
`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check`: all-feature tests and
clippy, layout and conformance checks, 20 archive fixtures, buffer-age pixel
equivalence and GLX/EGL first-frame and pixmap-export pixels. Twenty new tests
cover seven format-snapshot cases, eleven worker correlation/release cases and
two export/preparation ownership cases. Existing worker tests were moved to the
external private harness; the combined worker suite passes 26 tests.

The first full run passed tests/clippy but refused two modules newly exceeding
the source-layout limit. Plane queries and worker export handling were split by
domain; the subsequent full run passed without changing the debt ledger.
Passive composition trace data also moved out of the optional GBM implementation,
and existing GBM-only imports/cursor helpers gained their missing feature gates.
The libdrm-events-only backend now compiles; its two unrelated dead-code warnings
remain. Exact source hashes and both gate logs are in
`.artifacts/t070-frame-correlation/`. No production files changed during the
successful gate. No install, live-session restart or physical flip test occurred.

Correlation is explicit through primary-plane preparation. Successful topology
conversion and ordinary submission reports do not retain that field; worker
leases continue to own their own correlation. This is not complete retirement
or topology-proof tracking. No card-wide mutation token was found, so the next
differential test should freshly prepare and test both owners synchronously
before another owner service step. Temporary-source cleanup needs an independent
bounded obligation: the existing single cleanup slot may already belong to a
failed alternative submission and cannot be overwritten.

## Consecutive candidate tests after `1b7c84c8`

The native exporter now retains one structurally eligible original allocation
after a real direct TEST_ONLY refusal, where its strict plane snapshot proves
that explicit layout unsupported and has alternatives for the same fourcc.
It binds the source to the actual fallback worker request, or explicitly to
the owned inline frame. Deferred resubmission clears only that request binding;
the one-second absolute deadline never restarts. A new external offer invalidates
the source even when its trace matches. Disabling direct scanout, changing
target/owner/inventory or entering topology preparation invalidates it too.

A successful fallback must match the original format and extent and have a
supported explicit modifier. Native ownership permits probing only with no
pending sibling commit on the same card and no topology preparation. Both
framebuffers are prepared from the current selection, cursor and VRR request.
The paired helper compares canonical TEST_ONLY requests before either ioctl,
then tests original followed by alternative and returns both affine owners.
Unknown provenance and any difference besides the primary framebuffer produce
no test. Page-flip preparation allocates no mode blob, so independently prepared
requests do not acquire different MODE_ID values that would defeat comparison.

The alternative is the ordinary composed framebuffer; no extra render or pixel
copy is added. Capture and actual probe admission each have a separate one-second
rate limit, preventing a slow completion followed by a fast one from producing
a burst. The original's temporary framebuffer/import cleanup has one independent
slot. Failed cleanup neither replaces the alternative's cleanup nor blocks
ordinary frames; it is retried on idle native service as well as frame, topology
and shutdown service. Its owner remains held until cleanup succeeds. A timeout
at shutdown remains a reported failure.

The integration audit also confirmed a separate fallback bug. PRIME import or
framebuffer creation could fail before the direct TEST_ONLY gate and be reported
as terminal, despite the exporter retaining the composed form. Those failures
now return the frame to composition while preserving partial resource cleanup.
They produce no invented test result or layout-cause claim.

The renderer's allocator tries XR24 before AR24. XR24 success cannot establish
AR24 modifier support: the format table is keyed by fourcc and composition also
normalizes alpha. The current probe therefore requires equal formats. Broader
AR24 coverage needs a real output-format request on the existing fallback job,
including its persistent-target reuse key, rather than relabeling its descriptor.

`sophia_live_layout_probe schema=1` preserves pair status, both actual test
outcomes and errnos, format/modifiers and output/scene identity through the
sanitizer. The report is passive history. It does not carry an accepted
retirement, exact copied Present transaction or immutable frontend preference
generation. These remain gates before SuboptimalCopy; neither t070 nor t069 is
closed by a deterministic paired test alone. No physical flip acceptance is
claimed for this checkpoint.

Validation passed `SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` with
exit0: all-feature tests and clippy, layout/conformance checks, 20 archived
fixtures, hardware buffer-age equivalence and real GLX/EGL first-frame and
pixmap-export pixels. Twenty-five new tests cover the request pair, raw
provenance, early resource refusal, retained-source lifetime, actual exporter
wiring, generic paired submission/cleanup and diagnostic sanitization.

The first full run passed tests but reported one clippy simplification and two
test attributes placed in production modules. The attributes moved into the
external test files, following the existing private harness pattern; the debt
ledger did not change. Both full logs, source manifests and focused results are
retained under `.artifacts/t070-paired-layout/`. No Rust file changed during
the successful gate. No install, live-session restart or physical TEST_ONLY/flip
probe was performed.


## Alternative ownership through retirement after `b20c5fe4`

The full comparison is retained only on the prepared alternative. Before the
real commit, the reducer checks current request properties and correlation,
requires the original fresh TEST_ONLY to reject with EINVAL and the alternative
to pass, and excludes unknown errno/provenance. Successful unchanged submission
carries a compact witness with the original renderer image. Cursor-drop retries
and failed submissions carry none. Waiting callbacks retain ownership; matching
physical presentation consumes the witness once, including when cleanup of the
previous resource subsequently fails. Head loss discards it.

The native owner binds that witness to its actual device identity, head, frame,
submission cycle and target context. Topology, device inventory and owner changes
clear queued and outstanding evidence even if rollback restores the old target
generation. Retirement records now own content and disposition alongside timing;
they no longer consult the head's later displayed content. The quiet callback
path was found to clear submitted content before processing retirement, opposite
to the ordinary tick. Its ordering is corrected so the last frame can retain its
identity without requiring later rendering activity.

The production Present owner snapshots the exact submitted surface/backing key
before Engine settlement. A retired witness survives only an actual committed
copy with the same image, image-to-transaction mapping, format and single-output
frame. An old displayed image, stale Engine candidate, different format or
multi-output join remains inconclusive. The session records `RetiredCopy` through
the existing bounded diagnostic vocabulary; it does not send SuboptimalCopy.

The normal fallback can now request the original fourcc. Required requests are
strict. Optional preferences may abandon unavailable config/GBM/EGL target
admission before the first draw and use the original candidate order; context,
import, device and post-draw failures cannot take that branch. No additional
completed render or copy is introduced. Actual format governs retained-target
reuse and proof eligibility, so disappearance of an optional preference does not
recreate an otherwise compatible target.

Real renderD128 tests produce exact AR24 and XR24 pixels and check inline and
worker-slot cache reuse across required, preferred and ordinary requests. The
host's AR24 allocation reports an implicit modifier, while XR24 reports LINEAR.
Thus the AR24 render succeeds but still cannot establish the explicit-layout pair.
The unavailable-AR24 fallback is tested through the production admission reducer,
not forced on this driver, which supports AR24.

This implements the stronger native feedback lifetime pattern without importing
Wayland protocol or application policy into Sophia. XLibre remains the X11
semantic reference. Frontend effective-preference identity and exact alternative
membership are still missing from the client-advice join; raw plane support is
not a substitute. No physical KMS pair, install or normal-launch acceptance is
claimed for this checkpoint. The broader t069/t070 exits remain open.

Validation passed `SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` with
exit 0 on an unchanged Rust tree: all-feature workspace tests and clippy,
format/layout/conformance checks, 20 archived regressions, hardware buffer-age
pixel equivalence and real GLX/EGL first-frame and pixmap-export pixels. The
change adds 29 deterministic tests and one separately enabled GPU test; the
latter checks 18 actual pixel renders and cache transitions. Native owner tests
cover device/frame/cycle/context changes, queue ownership and invalidation.
Present tests include an actual stale Engine commit. The outer DRM-owning quiet
callback method has source review, not an isolated CPU integration fixture.

The initial full gate passed, but a separate DRM-only build exposed the passive
renderer image ID being hidden behind the GBM feature. Its unchanged type and
methods moved to the always-built renderer boundary; native snapshots remain
feature-gated. The reduced-feature checks and full gate were then repeated.
The default backend build retains its existing unused callback-capacity helper
warning, unrelated to this change.

Source hashes, full and focused logs, and the final signed candidate identity
are retained in `.artifacts/t070-retired-layout/`. The feature build checks
are recorded separately. No install or live-session restart occurred.

## Exact-format preference publication after cf8188cd

The native Wayland comparison exposed a missing input to the eventual advice
decision: Sophia published XR24 preferences only, although its strict plane
snapshot already retained independent XR24 and AR24 pairs. Niri's native
`surface_dmabuf_feedback` intersects plane pairs with renderer importability and
qualifies scanout preferences by device. Sophia now applies that pattern through
its native owner and DRI3 frontend: exact-format rows come from the cached strict
snapshot, and the requesting connection supplies the immutable import set.
Unknown or empty rows give no preference. The renderer's separate legacy XR24
allocation policy and candidate ordering are unchanged.

A read-only query of this host's active card0 primary planes, 53 and 59, found
eleven explicit modifiers for each format, including LINEAR. The current lists
happen to match; the regression fixtures deliberately give XR24 and AR24 distinct
ordered lists, and also exercise an AR24-only plane. The query took no DRM master,
performed no TEST_ONLY or commit, and proves table membership alone.

The frontend query now resolves its own connection's exact-format catalog rather
than accepting a caller-supplied screen list. Its borrowed resolver allocates only
the returned reply list, and binary-searches canonical screen modifiers instead
of repeatedly scanning them under the authority lock. A healthy foreign device
can still receive measured LINEAR preferences. An unavailable pinned bundle
receives none; it can no longer masquerade as the healthy cross-device case.
Device loss serializes under the same runtime-to-device lock order as installation.
The already advertised screen inventory stays immutable, and existing connections
are never redirected to a newer bundle.

Seven new tests and strengthened socket cases cover independent format rows,
strict unknown snapshots without changed renderer admission, no invented rows,
canonicalization, device loss, old/new pinned catalogs, absent-device admission,
and exact-format legacy queries. Focused results and the read-only plane queries
are retained in `.artifacts/t070-exact-format-preferences/`.

`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` passed on ten unchanged
Rust source files: all-feature workspace tests and Clippy, layout and conformance
checks, twenty archived regressions, hardware buffer-age pixel equivalence, and
real GLX/EGL first-frame and pixmap-export pixels. The evidence directory retains
the source hashes, full gate log, and final signed candidate identity.

This still emits no SuboptimalCopy. The completion join must capture the exact
successful Present identity and revalidate its current effective preferences
before consuming that pending completion. The 250 ms asynchronous publisher
cannot establish currency by itself: the owner's immediate placement/native
context must still match the acknowledged snapshot. Native rollback also needs
a monotonic context identity at that boundary. A mismatch must suppress optional
advice without delaying ordinary Copy. No installation, physical paired-test
acceptance or normal-launch acceptance is claimed here.

## Current preference comparison after 820f7694

The retired witness now travels with its exact copied Complete. Backend settlement
requires one matching Complete and leaves the existing feedback order, disposition
and idle-fence outcome unchanged. The session compares the current committed
surface/backing, unambiguous placement and native context against its acknowledged
preference snapshot. An asynchronous snapshot awaiting acknowledgement cannot
stand in for accepted state. Both normal owner-loop feedback drains perform this
comparison; teardown remains deliberately conservative.

Native allocation contexts use a nonrepeating process-wide generation. Owner
reconstruction, device/topology transitions, numeric rollback and head loss
invalidate that context. Exhaustion disables optional evidence without disabling
ordinary rendering. The scalar context query performs no capability discovery or
list cloning. Physical head and target identities remain in backend/session;
the frontend receives only an opaque context generation and logical output.

The frontend captures the successful DMA Present's requested window incarnation,
presenting connection and original buffer/layout while the authority owns the
dispatch. A later pixmap free or XID reuse cannot change those facts. Immediately
before consuming Complete, it checks current mapping, geometry, accepted
generation, device availability and exact-format membership in the presenter's
pinned screen/window intersection. It does not borrow a parent's preference for
a child Present or the preference of a Complete subscriber. The alternative must
remain preferred and the original must not. This is a current-state comparison,
not request-time preference history or permission for a later flip.

The authority lock is attempted without waiting. Missing, stale, unavailable,
poisoned or busy state rejects only the optional comparison; ordinary Copy,
completion-clock advancement and Idle still proceed. Successful comparison emits
the bounded `PreferenceMatched` diagnostic, not SuboptimalCopy. One rare boxed
record carries evidence; normal completions allocate no additional evidence or
format lists. Frontend membership uses the same canonical borrowed resolver as
DRI3 queries.

Review also found a capability-owner defect: construction sorted heads and their
exporters while retaining plane-format snapshots in discovery order. A reordered
head could therefore publish another head's formats. The snapshot now belongs to
the physical head record and moves with it. Tests deliberately reorder heads and
give their format sets different values.

Twenty-two new tests cover native context identity and head association, exact
feedback attachment, current owner snapshots and frontend comparison. The frontend
suite includes real socket Present capture, window/pixmap XID reuse, independent
Configure/Unmap changes, busy-authority completion, device loss, Idle-first order
and runtime binding lifetime. Focused backend, session and frontend suites pass.
Evidence and the final candidate manifest are retained in
`.artifacts/t070-completion-preferences/`.

`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` passed with exit 0 on
31 unchanged Rust files. It covered all-feature workspace tests and Clippy,
format/layout/conformance checks, twenty archived regressions, hardware buffer-age
pixel equivalence, and real GLX/EGL first-frame and pixmap-export pixels. The
initial focused Clippy run found two test-table type-complexity warnings; a shared
test type alias resolved them before the successful full gate.

Physical paired-test/retirement acceptance remains open. These deterministic
tests do not acquire DRM master or prove that this host produces a qualifying
alternative in a normal scene. No installation, live-session restart or
SuboptimalCopy signaling is part of this checkpoint.

## Controlled allocation and evidence collection after 41ab89f9

The generic `dri3_layout` probe allocates an explicitly requested XR24 or AR24
layout through DRI3's device. GBM supplies the actual format, modifier, plane
strides, offsets and descriptors; a substituted layout is refused. List-only
queries an unmapped window and can measure an allocation without import or
Present. The presenting mode fills through GBM, uses two bounded buffer slots
and waits for both Complete and Idle before reuse. X setup and event waits share
an absolute process deadline. No input or desktop capture is involved.

The diagnostic join now carries the renderer-owned source image from paired
TEST_ONLY to RetiredCopy, and native context generation from RetiredCopy to
PreferenceMatched. The latter also reports the effective preference generation.
These are scoped numeric diagnostics, not protocol fields or new authority.
The privacy filter still strips arbitrary payloads and paths. Its existing
substring rule rejects keys containing `text`, so the context nonce is emitted
as `native_generation`; exact sanitizer tests pin all three added fields.

`verify_layout_comparison.py` requires a unique paired test for that source and
layout, the exact retired transaction/context, current preference match, and a
unique routed Complete clock received by the probe as ordinary Copy. It also
matches the probe's serial, pixmap and original allocation metadata. Repeated
attempts with indistinguishable keys are rejected rather than inferred from log
proximity. The caller must supply one bounded, identified session/run and verify
capture health; this reader validates the join, not those prerequisites or the
task exit.

Eleven reader tests and three compiled-client CLI/deadline tests pass. An ignored
private frontend test measures real LINEAR XR24 and AR24 allocations and verifies
two DRI3 Opens, four exact-format queries and zero map/import/Present requests.
The full gate includes these checks. Artifacts, source hashes and candidate
identity are retained in `.artifacts/t070-physical-probe/`.

`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` passed with exit 0 on
13 unchanged source/tool files. This includes all-feature workspace tests and
Clippy, formatting/layout/conformance, the new reader and process tests, twenty
archived regressions, real buffer-age and GLX/EGL pixel checks, and the private
XR24/AR24 allocation fixture.

Two authorized small live runs used the installed `76ed2fddf31a` session
`00000001788951558950-1cb15054-f8c5-4096-b573-a10d6c7acbcd`. Each presented four
96x64 LINEAR frames at 64,96, received four ordinary Copy completions and four
Idle events, then exited successfully. XR24 completion latency was 8–17 ms;
AR24 was 9–25 ms. These establish the probe's feedback/reuse path on the old
owner. They neither run the new comparison code nor prove scanout eligibility,
physical pixels or a qualifying alternate layout. No installation or session
restart was performed.

## Live allocation preferences on 9611131e

The next installed session, `00000001789000283295-eca6d35b-d625-47a8-ae46-f8fa1a3f5667`,
runs signed candidate `9611131e7bb268156e8fe1da89c0fd0f518ff7fb`. Its running
executable and session manifest match the release SHA-256
`cf3253f169aae5e15cd85e8f88a04cff0ebc33e6757171f18ef2db0bf804ec1f`.

Six authorized 96x64 probe windows completed 120 frames each. Output 1 covered
both XR24 and AR24 with LINEAR and explicit `0x0200000028a6bf04`; output 2 covered
both formats with that compressed modifier. GBM reported one plane for LINEAR
and two planes for the compressed allocation. All 720 submissions received
ordinary Copy and Idle with matching serial/pixmap identities, then the clients
exited successfully. Median submit-to-feedback time was 8 ms on the 120 Hz
output and 17 ms on the 60 Hz output. This measures client feedback, not input
latency or physical pixel correctness.

Each connection's XR24 and AR24 screen catalogs stayed at eleven entries before
mapping, after mapping and after the final completion. Once the owner published
the window snapshot, both window catalogs contained seven entries. A fresh
read-only query of both active primary-plane format blobs proves those seven
are exactly the measured import/plane intersection. The compressed modifier is
absent from both plane sets and both window preferences, but is advertised for
import and successfully composed. Thus this run establishes the intended
distinction between usable imported storage and preferred scanout allocations.

One acceptance precondition was missed in the previous handoff: normal startup
does not enable direct scanout. The owner has no `SOPHIA_ENABLE_DIRECT_SCANOUT`
setting, and `LiveProductionNativeScanout::direct_scanout_enabled` admits that
path only for value `1`. Without it exporters derive no direct candidate, so
there can be no original TEST_ONLY, paired comparison or retired witness.
`SOPHIA_LIVE_VISUAL_PROGRESS` controls success-feedback logging only. Neither
variable can be enabled by putting it in the probe's environment. The existing
opt-in remains unchanged; the controlled physical gate must set both before
starting its owner. This corrects the earlier implication that an updated
normal session alone would exercise the pair.

The bounded capture spans sequences 3688 through 15545, with no gaps or duplicate
sequences, no discarded records or storage errors, and the recorder still
running. There are no atomic-test or layout-probe records in that interval,
consistent with the independently established disabled path. Exact source,
binary, output, plane, allocation and feedback evidence is retained in
`.artifacts/t070-live-9611131e/`. No pixel capture, forced output change, input,
installation or session restart was performed during these probes. The alternate
flip remains unproven. A separate [same-browser comparison](uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md#installed-9611131e-comparison)
still fails without a device override and passes its existing device-selection
control; normal composition evidence does not close that client-side gate.

## Direct-scanout controls and missing trace transport on 5110bb7d

Session `00000001789002269795-7f4905bd-faf7-4f20-87ec-8f7c7b38d36a` runs
installed `5110bb7d80942c6d718c9b598c4ce78525aa72d4`, with the same executable
SHA-256 `cf3253f169aae5e15cd85e8f88a04cff0ebc33e6757171f18ef2db0bf804ec1f`
as the preceding `9611131e` allocation tests. Both owner startup variables,
`SOPHIA_ENABLE_DIRECT_SCANOUT=1` and `SOPHIA_LIVE_VISUAL_PROGRESS=1`, were verified
in the live process. The separate Layout Test login entry enabled them; the
normal entry was not changed.

A 2560×1440 XR24 LINEAR window covering the primary output completed 120 Copies.
The composition plan retains underlying mapped surfaces even when fully covered,
so this was not an isolated direct-scanout control. A geometry-only X query found
the secondary output contained only its 1920×32 panel. Each subsequent probe
temporarily unmapped that panel, monitored its resource lifetime, and restored
the same live window in cleanup. Restoration to mapped state was verified after
every run. Kitty remained open, and no input or output-topology change was used.

The isolated 1920×1080 controls produced:

| Format | Actual modifier | Planes | Complete events | Idle events before teardown |
| --- | --- | --- | --- | --- |
| XR24 | LINEAR | 1 | 120 Flip | 119 |
| AR24 | LINEAR | 1 | 120 Flip | 119 |
| XR24 | `0x0200000028a6bf04` | 2 | 120 Copy | 120 |
| AR24 | `0x0200000028a6bf04` | 2 | 120 Copy | 120 |

All five probes exited successfully. The final linear buffer remained owned
until connection teardown, which explains its missing pre-teardown Idle event.
These are received protocol completions, not captured-pixel evidence. Separate
linear and compressed runs do not establish the required equivalent-request
atomic comparison, and they do not close t070 or the broader t069 exit.

The missing atomic/layout records exposed a concrete instrumentation defect.
Backend `tracing::info!` events used the ordinary console subscriber. The daily
recorder accepted only Session's installed output callbacks, while the native
launcher deliberately redirected console stdout/stderr to `/dev/null`. Thus
backend records never reached `capture_line`, even though its sanitizer already
understood their schemas. Zero capture drops could not detect this missing
producer connection. The linear Flip control confirms that interpreting the
missing atomic record as an unexecuted direct path would have been wrong.

The repair adds a CLI-owned tracing layer and an explicit
`sophia_scanout_evidence` target on the two backend emitters. Only the exact
atomic-test and layout-probe message names are forwarded, without spans or
other event fields. Filtering is per layer, so console filtering cannot disable
evidence or cause unrelated debug callsites to run. Formatting has the same
4096-byte limit as Capture, with one extra byte reserved to report whole-record
overflow through its existing discarded counter. UTF-8 boundaries are preserved.
No backend-to-Session dependency or child-output collection is introduced.

The subprocess regression exercises the real subscriber and Capture with
`RUST_LOG=off` and stdout sent to `/dev/null`. It checks exact-once records,
redaction, target/name exclusions, unformatted private fields, disabled unrelated
debug callsites, the exact size boundary, and counted ASCII/UTF-8 overflows. A
second subprocess checks ordinary console logging still works. This tests the
transport; it does not invoke native atomic commits.

`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` passed with host permissions
for its socket and offscreen GPU fixtures. It includes the new transport
regression, all-feature workspace tests and clippy, formatting and reader checks,
archive fixtures, buffer-age pixels and GLX/EGL first-frame/pixmap-export pixels.
The initial sandbox run was refused at local socket creation. The complete
passing log is `full-check-host.log`; source identities remained unchanged
through the gate. The repaired owner has not yet been installed or exercised
against a physical atomic comparison.

Evidence is retained in `.artifacts/t070-live-layout-5110bb7d/`: candidate and
owner identity, real allocations, feedback, geometry/restore records, and
sequences 9052–135481 inclusive. All 126430 sequence numbers are present exactly
once. Capture reported zero discarded records, zero storage errors and no lost
rotation bytes. The recorder remained running. A candidate containing the
transport repair must repeat the isolated comparison before the compressed
Copy can be attributed to a particular preparation or atomic-test refusal.


## Framebuffer rejection on 7ac6f1ea

Installed session `00000001789004130781-aa0cb0a1-abdf-4e85-8d15-b7806e096b68`
runs signed `7ac6f1eac239b4dde29c66cbd55fc4219ae190f4` with both owner test
variables enabled. The running binary, manifest and immutable installed package
agree on SHA-256 `b3745c4e9412627c03dbab69eee8b4df2978291aa9901fef50b0834b90d27aa3`.
The package uses `native-session`; the developer build used its
`atomic-scanout-live` feature alias and has a different hash. No source condition
branches on that alias, but the two binaries are not claimed byte-identical.

The isolated secondary-output XR24 controls each completed 120 submissions.
LINEAR received 120 Flip completions and 119 pre-teardown Idle events; the real
two-plane `0x0200000028a6bf04` allocation received 120 Copy and 120 Idle events.
The same bar was restored after each run and Kitty remained open. The new trace
transport captured one actual atomic `Submitted` result for LINEAR, at sequence
56779. The compressed run contains no atomic or layout-probe record. Its copy
path therefore still lacks a captured original preparation result; this absence
alone does not identify which earlier gate refused it.

A separate, non-master DRM diagnostic supplies the driver discriminator. It
obtains the render FD through DRI3, resolves the PCI-identical primary node,
allocates and fills the same explicit layouts through GBM, imports each plane
through PRIME, and calls AddFB2 with the actual modifier. It performs no atomic
commit, modeset, master acquisition or window mapping. Both compressed planes
import successfully into the same GEM handle. AddFB2 then returns EINVAL (22),
while the LINEAR control succeeds. Every created framebuffer and unique imported
handle is released. This reproduces a framebuffer-stage rejection for the actual
layout; it is not a trace of the running owner's ioctl.

Read-only XR24 and AR24 queries cover all ten planes on the DRM device. Each
active primary plane's eleven-modifier set equals the union across all planes;
the tested compressed modifier is absent. Linux v6.18's
[AMD framebuffer constructor](https://raw.githubusercontent.com/torvalds/linux/v6.18/drivers/gpu/drm/amd/amdgpu/amdgpu_display.c)
requires some device plane to support a format/modifier before creating the
framebuffer. The [DRM plane check](https://raw.githubusercontent.com/torvalds/linux/v6.18/drivers/gpu/drm/drm_plane.c)
uses the driver's format callback. These are pinned reference sources, not a
verification of every change in the installed 6.18.46 kernel. Together with the
measured AddFB2 refusal, they expose why requiring an original atomic rejection
cannot exercise this particular layout on this device.

The repair preserves that stage distinction. An explicit non-linear AddFB2
failure retains errno and error kind after complete PRIME import. It can retain
an otherwise eligible source for the existing bounded fallback comparison.
When the exact fallback is ready, original preparation runs again. If it now
succeeds, the existing paired-atomic path applies. If it again refuses at AddFB2
with EINVAL, the shared production request builder combines the original's
selection, discovered properties and policy with the alternative's real
framebuffer. That intended request must exactly equal the alternative's test
request, including cursor, VRR and flags. Only the alternative is tested in this
arm; there is no invented original TEST_ONLY result.

Unknown stages, PRIME failure, resource-pressure/access errors, changed layout,
selection or request suppress the optional witness. Passing TEST_ONLY still
needs an unchanged actual commit, exact retirement and the existing current
preference/transaction join. No extra render or pixel copy is introduced. The
one-second candidate lifetime, rate bounds and separate cleanup obligation stay
in force. Schema 3 identifies the original stage as `Atomic` or `Framebuffer`;
the reader retains legacy schema-1 atomic records without reinterpreting them.

Physical evidence is retained in `.artifacts/t070-live-7ac6f1ea/`, including probe
and source identities, private framebuffer results, full plane inventories and
per-run sequence intervals. The LINEAR interval is 56772–58285 (1514 records);
the compressed interval is 64550–66267 (1718 records). Both are contiguous and
unique. Capture reports no discarded records, lost rotation bytes or storage
errors and remains running. No physical pixel capture was performed. The new
framebuffer proof arm still requires installed-owner acceptance; neither t070
nor the separate Chromium no-override exit is closed by these controls.

The implementation passed `SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check`
with host permissions for local sockets and offscreen GPU fixtures. This covers
101 backend library tests, 300 native-feature integration tests, all-feature
workspace tests and Clippy, thirteen comparison-reader cases, twenty archive
fixtures, buffer-age pixels and GLX/EGL first-frame/pixmap-export pixels. The new
tests exercise actual preparation and submit hooks, shared-plane cleanup,
non-EINVAL refusals, changed cursor/VRR/request state, independent cleanup
retries and exact retirement. The final run has no compiler or Clippy warnings.
Its log and the unchanged 22-file source manifest are retained as
`full-check-final.log` and `final-source-before-gate.json` in the same artifact
directory. The manifest is captured after the final test-fixture mount repair;
the earlier gate passed with a duplicate-module warning. These deterministic
checks do not substitute for an installed physical comparison.

## Connections

The [device negotiation checkpoint](../milestones/szr8j0rg-connection-pinned-device-negotiation-and-bounded-renderer-refresh.md)
records native allocation-preference and resource-lifetime work. The
[Brave investigation](uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md)
separately identifies the client-internal X11 media-selection gap. See
[architecture](../../architecture.md) for the authority and test-history contract.
