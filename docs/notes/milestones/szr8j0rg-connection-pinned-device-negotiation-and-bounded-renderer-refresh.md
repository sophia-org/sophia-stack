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
