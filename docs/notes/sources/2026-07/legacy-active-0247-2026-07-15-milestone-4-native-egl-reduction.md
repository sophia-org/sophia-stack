---
id: legacy-active-0247
date: 2026-07-15
recorded_date: 2026-07-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-07-15: Milestone 4 Native-EGL Reduction

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8244–8264. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The remaining mixed-draw failure now has a bounded reproduction below KMS.
`native-egl-vkcube-mixed-smoke` launches the real native-X `vkcube` transport,
transfers its DRI3 planes through `LivePresentationResourceSession`, combines
them with the full-output CPU background, and invokes the persistent mixed
exporter directly. A watchdog parent reports child exit or timeout, while the
successful child emits schema-1 evidence only after disconnect drains all live
sources, fences, and presentations. Fixture-backed verification rejects a
missing CPU layer or cleanup debt.

Native composition now reports CPU upload, EGLImage create/bind, draw, finish,
and destroy failures separately. The mixed CPU background uses dedicated,
fixed-size texture storage and sub-image updates rather than reallocating the
fullscreen upload texture in the command stream that samples the imported
EGLImage. This is locally covered and the full offline all-feature suite passes,
but it is not hardware promotion evidence until the focused X13 smoke and the
normal schema-14 paired proof both pass. The paired wrapper now retains
privileged before/after kernel logs and driver environment identity so another
AMDGPU rejection cannot lose its validator record.

<!-- END IMPORTED BODY -->
