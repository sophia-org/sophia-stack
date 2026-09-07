---
id: legacy-active-0108
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "architecture"]
---
# 2026-08-07: The promotion policy and TrueColor contract are closed inputs

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3475–3514. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The Milestone 12 desktop policy no longer depends on mutable home
configuration or executable discovery. Sophia checks in one minimal xmonad
0.18.1 configuration using xmonad-contrib 0.18.2 and only the blind
`ThreeColMid`, `Tall`, `Mirror Tall`, `Full`, and `Spiral` geometry policies.
The configured real-xmonad smoke requires exact three-surface results across
all five layouts, wrap, focus, constraints, release, and restart. The existing
profile suite retains workspace, floating-pointer, work-area, and output-change
coverage. Tabbed decorations, metadata manage hooks, spawn/kill policy, and
dzen control remain excluded.

Release packaging now builds that checked-in configuration and the exact clean
`~/src/xmobar` revision, copies both executables and configurations, and records
their source identities and SHA-256 digests. Installed resolution accepts only
the packaged absolute paths. The package verifier rejects wrong source
versions, missing files, executable or configuration digest mismatch, and an
unrecorded xmobar revision; runtime identity and soak verification cover both
artifacts. Development builders remain available, but they are not installed
fallbacks.

X Authority now owns the complete fixed-TrueColor contract. XLibre's
`miResolveColor`, `AllocColor`, and `QueryColors` establish the retained
semantics: RGB16 allocation takes each high byte, returns that value expanded
by `0x0101`, packs the advertised masks, and supplies an opaque alpha mask for
the 32-bit visual; query rejects bits outside the visual mask and expands each
channel by `0x0101`. XLibre also establishes `BadColor`, `BadMatch`, `BadName`,
`BadValue`, and `BadIDChoice` behavior. Yserver confirms the value of passive
colormap records and a bounded normalized name table, while its permissive
query validation is not copied.

Sophia therefore stores only colormap ownership plus visual identity, never a
mutable TrueColor palette. Window depth/visual/colormap triples must agree,
client colormaps are released on disconnect, allocation and query replies carry
actual channel values, and unknown names fail instead of becoming white. Both
wire orders, XRGB and ARGB allocation, duplicate/invalid resources, advertised
masks, pixmap depth, and a non-gray XRGB upload palette are deterministic
regressions. The remaining color boundary is the visible physical capture on
the successor installed candidate.

<!-- END IMPORTED BODY -->
