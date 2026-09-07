---
id: legacy-active-0177
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session"]
---
# 2026-07-26: Resize Timeout Recovery Is An Engine Replan

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6075–6116. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Launching a non-cooperative Vulkan top-level exposed an architectural failure,
not an application quirk. This recovery model remains valid, but the later
pre-map admission audit above supersedes it as the root-cause diagnosis for the
blank initial `vkcube` window. Xmonad proposed a tiled epoch, existing clients
and the new surface did not all publish the exact requested extents before the
deadline, and the CLI compensated back to old sizes. Preselecting startup
dimensions would still have encoded application policy.

Resize/admission recovery now belongs to a protocol-neutral Engine coordinator.
It retains safe authority content extents, fences pixels from abandoned sizes,
and stores declared constraints separately from temporary exact recovery
constraints. A timed-out first admission is retried once with
`min_size == max_size == safe_extent` and `resizable = false`; the blind WM
still chooses placement. The unsafe slow-client option that synthesized visual
truth from timed-out pending pixels was replaced by a replan-at-committed-extent
decision.

The legacy compatibility bridge generically exposes effective constraints as
synthetic ICCCM `WM_NORMAL_HINTS`. When manage-time constraints change it
remanages the private synthetic window so stock xmonad reevaluates fixed-size
policy. No Vulkan, Kitty, xmonad-client, XID, namespace, title, class, or PID
fact enters Engine policy. Default `vkcube --wsi xcb` is the physical
compatibility proof, not an implementation branch.

The real unmodified-xmonad bridge smoke now follows its sequential three-window
tiling proof with a manage-time transition of one opaque node to an exact
500-by-500 constraint. Xmonad returned that floating placement after the bridge
remanaged the private synthetic window, proving the ICCCM path without client
metadata. Physical authority/presentation admission remains the open roadmap
gate.

The first physical run then found one remaining unit-boundary error. Recovery
retained a 500-by-500 client buffer but sent 500-by-500 as the WM's outer
constraint. The active two-pixel clearance correctly inset that allocation to
a 496-by-496 client configure, which the application did not satisfy. Engine
now owns the inverse conversion as well: committed geometry and content
constraints become 504-by-504 outer facts before the WM boundary, and the
existing inset returns exactly 500-by-500 to the authority. Focus ring/frame
width is therefore handled generically rather than encoded in recovery policy.

<!-- END IMPORTED BODY -->
