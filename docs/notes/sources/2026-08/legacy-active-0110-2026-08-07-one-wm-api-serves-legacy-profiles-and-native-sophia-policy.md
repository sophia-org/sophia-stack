---
id: legacy-active-0110
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-07: One WM API serves legacy profiles and native Sophia policy

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3643–3681. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Xmonad is Sophia's first mature classical-X11 compatibility profile and the
current daily-driver promotion vehicle. It is not Sophia's architectural
window manager. Engine exposes one blind, versioned WM policy API. A native
Sophia policy process speaks that API directly; a classical X11 WM speaks only
to the compatibility bridge's private synthetic X server, whose bounded
profile translates policy into the same API. Future i3, dwm, qtile, or other
profiles must each retain their own request and action evidence without seeing
real X Authority clients, metadata, pixels, or physical input.

The immediate xmonad configuration work therefore remains profile-local. The
installed session will compile and package a fixed xmonad executable rather
than loading mutable home configuration, package the exact xmobar executable,
and verify both digests. Geometry-only layouts may enter after deterministic
bridge coverage. Title-aware tab decorations and metadata rules may not widen
the fake X server or leak into Engine. The current Void host already has the
required configuration build and runtime dependencies; dependency installation
is closed.

Hagia remains the intended first demanding Sophia-native policy and shell
family. Its spatial-policy process will own private tags, layout structures,
focus policy, scrolling, and Janet layouts while remaining blind. An optional
`hagia-shell` will own authorized visible furniture through a separate shell
projection. Engine retains hit-testing, input, animation, rendering, and
scanout; session services, portals, and trusted classification brokers retain
launch, lock, capture, transfer, and metadata authority. The direct and legacy
paths must share semantic conformance tests so compatibility work strengthens
rather than forks the WM boundary.

The same roadmap review exposed an incomplete X Authority TrueColor contract.
Sophia correctly advertises 24-bit XRGB and 32-bit ARGB TrueColor visuals and
maps arbitrary `AllocColor` components into the advertised masks. However,
`QueryColors` currently reports every nonzero pixel as white, and
`AllocNamedColor` treats every name except black as white. Completing mask
round-trips, bounded retained-client color names, validation and error paths,
and a non-gray physical pixel proof belongs in X Authority. Engine must continue
to receive only normalized XRGB8888/ARGB8888 pixels and opacity facts.

<!-- END IMPORTED BODY -->
