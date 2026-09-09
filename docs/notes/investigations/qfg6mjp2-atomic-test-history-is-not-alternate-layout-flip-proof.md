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

## Connections

The [device negotiation checkpoint](../milestones/szr8j0rg-connection-pinned-device-negotiation-and-bounded-renderer-refresh.md)
records native allocation-preference and resource-lifetime work. The
[Brave investigation](uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md)
separately identifies the client-internal X11 media-selection gap. See
[architecture](../../architecture.md) for the authority and test-history contract.
