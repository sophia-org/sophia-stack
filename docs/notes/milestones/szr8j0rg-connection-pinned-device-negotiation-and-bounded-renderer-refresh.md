---
id: szr8j0rg
date: 2026-09-09
kind: milestone
status: recorded
tags: [milestone, rendering, x11]
---
# Connection-pinned device negotiation and bounded renderer refresh

## Result

The server implementation slice of [t069](../plans/6tewvlbh-universal-device-negotiation-across-sophia-clients.md)
landed in signed checkpoints `92d1f3ea` and `978d858a`. This records implementation
and deterministic/offscreen validation, not completion of the normal-launch
physical exit.

The frontend pins a connection to one immutable device/capability/allocator
bundle. New bundles affect new connections; old backings keep their originating
provider through final release. A refusing provider no longer prevents releases
owned by healthy providers. DRI3 1.3 device hints and generation-checked window
preferences remain advisory and preserve connection-specific screen modifiers.

The session prepares replacement bundles outside its owner loop and reconciles
exact inventory and acknowledgement generations. Inventory replay is idempotent,
failed observations retry, and auxiliary-device changes refresh renderer sources
without reconstructing output contexts or retained images. The renderer attempts
actual-FD direct import before fallback, retains source contexts, and reuses at
most three internal linear bridges after destination GPU completion. Unfinished
workers occupy a bounded registry without blocking owner teardown.

The current contracts are in [architecture](../../architecture.md),
[frontend negotiation](../../sophia-x-authority.md), and
[pixmap export ownership](../../pixmap-texture-exports.md). No application names,
browser flags or toolkit policy were added to Engine, renderer or frontend.

## Validation

`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check` passed on the implementation
candidate. It covers all-feature workspace tests and clippy, formatting, layout,
profile and reader checks, archive regression fixtures, hardware buffer-age
pixel equivalence, and GLX/EGL first-frame and pixmap-export pixels. Retained log:
`.artifacts/t069-validation/xtask-check.log`; candidate identity:
`.artifacts/t069-validation/candidate.json`.

Focused tests exercise immutable connection pins, cross-connection backing
ownership, release retry fairness, stale window preferences, overtaken bundle
preparation/install acknowledgements, inventory retry, worker saturation and
unfinished predecessor retention. Pixel tests exercise transfer, alpha over a
white background and retained images after context replacement. These are
private offscreen tests; none establishes physical hotplug acceptance.

An optimized benchmark captured 600 changing 1920x1080 images for each XR24/AR24
and GPU-direction pair: 2,400 exact pixel checks passed between renderD128 and
renderD129. Host capture p95 ranged from 3.086 to 4.068 ms; the largest capture
was 7.167 ms. Each pair allocated one internal bridge and reused it for all 600
warm captures. Capture timing excludes CPU source preparation and does not
measure display latency. The additional readback timing includes test composition
and readback and is not a presentation metric.

The benchmark log, binary digest, source manifest and summary are retained under
`.artifacts/t069-renderer-transfer/`. The measured source differs from `978d858a`
only in rustfmt ordering of native renderer module declarations; the final
repository gate rebuilt and checked the committed implementation. Earlier debug
baseline runs lack an exact source/binary manifest and support no release-profile
speedup claim.

## Bounds and remaining acceptance

A same-layout foreign-to-local GPU fixture could not be constructed on this
host. Direct-first ordering is pinned through the production policy helper,
while hardware transfer checks use the actual incompatible layouts. Cached idle
bridge storage remains charged to the image budget until replacement or inventory
refresh; it can cause a bounded refusal near that budget.

The [t069 exit](../plans/6tewvlbh-universal-device-negotiation-across-sophia-clients.md#scope-and-exit)
still requires normal unmodified clients without device overrides and physical
device-loss/recovery evidence. A client-internal import failure before submitting
a buffer is outside the server's transfer boundary. The working launch adapter
and installed session were not changed by this checkpoint.

SuboptimalCopy remains disabled. The [five conditions](../plans/6tewvlbh-universal-device-negotiation-across-sophia-clients.md#suboptimalcopy-gate)
are tracked in t070–t074; a sufficient exact alternate-layout flip proof precedes
opt-in plumbing and deduplication. Failed TEST_ONLY and IN_FORMATS absence alone
do not establish that counterfactual.

## Paired-layout implementation

The subsequent [atomic evidence checkpoint](../investigations/qfg6mjp2-atomic-test-history-is-not-alternate-layout-flip-proof.md#consecutive-candidate-tests-after-1b7c84c8)
retains an exact original allocation through bounded fallback rendering and
issues fresh paired TEST_ONLY requests against current state. It reuses the
ordinary rendered alternative, keeps temporary cleanup independent and serviced
while idle, and repairs composition fallback after early PRIME/AddFB refusal.
Its full repository gate and hardware pixel checks pass; physical paired-scanout
and copied-transaction attribution remain separate. Evidence and final candidate
identity are in `.artifacts/t070-paired-layout/`; the live release was not changed.

## Installed playback observation

Mason installed `76ed2fddf31a` and reported Brave Origin open with video playing
in session `00000001788951558950-1cb15054-f8c5-4096-b573-a10d6c7acbcd` on
2026-09-09. The session manifest and live Sophia executables match that release.
The recorder was running with zero discarded records and storage errors;
completed frame retirements continued during the observation.

The browser and GPU process both retained
`--render-node-override=/dev/dri/renderD128`. This accepts the reported playback
on the new installed candidate with the existing override. It does not establish
normal launch behavior without the adapter, hardware video decoding, or device
loss/recovery. Browser stderr was `/dev/null`, so no browser error-log result is
claimed. Retained diagnostic facts are in
`.artifacts/t069-validation/installed-76ed2fdd-observation.json`.

## Device-aware window preference correction

The subsequent native reference review identified a narrower allocation-advice
problem. Sophia intersected output modifiers with the connection's screen set,
but equal tiled modifier numbers did not establish that both devices were the
same. A missing client hint left that distinction untested. niri's native
feedback builder explicitly restricts cross-device scanout preferences to LINEAR
while retaining the ordinary rendering fallback.

The correction caches server-observed render-node filesystem device, inode and
device number in the immutable connection bundle and output preference snapshot.
Only equal available identities permit tiled preferences. Missing or different
identity permits LINEAR only where both measured sets already contain it.
Client hints can further restrict that result. Inconsistent identity/device-number
snapshots are rejected before replacing accepted preferences. Screen formats and
actual submitted-buffer import remain unchanged; no advice asserts that atomic
scanout will succeed.

The contrary-path review also found that resolving a held card's device number
through current sysfs alone could associate an old KMS group with a new node.
Inventory refresh occurs before topology-notice handling, and topology preparation
can defer that notice, so ordering does not exclude the interval. Physical
mapping therefore revalidates both held card and render-node identities around
sysfs resolution; missing or replaced nodes yield no output preference. This is
inventory-time metadata work, not per-query discovery or GPU work.

The [native feedback model](../plans/6tewvlbh-universal-device-negotiation-across-sophia-clients.md#native-feedback-model-and-reference-boundaries)
records reference identities and the mapping to Sophia's native X11 boundaries.
The separate [protocol comparison](../investigations/uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md#native-wayland-and-x11-comparison-on-2026-09-09)
shows successful native Wayland browser playback with internally propagated
device selection, and the same client-internal failure through two X11 server
implementations. Native XLibre was reviewed as source, not run. The overall
no-override and physical device-loss exits remain open.

Validation on the correction based on `40161f42` passed
`SOPHIA_FIRST_FRAME_REQUIRE_AUX=1 cargo xtask check`: all-feature workspace tests
and Clippy, conformance/reader/archive checks, hardware buffer-age pixel
equivalence, and GLX/EGL first-frame and pixmap-export pixels. The source was
unchanged during that successful gate. The initial gate stopped at export-order
formatting; the corrected file was checked by the successful full run.

Focused regressions passed: eleven DRI3 capability tests, five window-allocation
tests, one real-socket identity/connection-lifetime regression, one session
identity-propagation test and four held-device mapping tests. The socket test
also proves that repeated queries do not call the provider's identity callback.
The mapping tests inject metadata changes on either side of resolution; they
are deterministic identity tests, not physical hotplug acceptance.

The exact candidate manifest, focused logs and successful full-gate log are
retained in `.artifacts/t069-window-device-preferences/`. Browser source and launch configuration were unchanged.
