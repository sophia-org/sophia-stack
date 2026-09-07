---
id: legacy-active-0323
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-25: Retained Contexts Across Fresh Surfaces Are Unsafe

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10088–10111. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The fail-safe mixed path bundled four lifetimes into one target: EGL context,
GL pipeline, EGL window surface, and GBM surface. Destroying that bundle after
every successful export protected the leased front buffer but also rebuilt the
context and shaders on every frame. This explained the repeated composition
creation count and left input and page-flip latency coupled to driver setup.

The attempted lifetime split created a distinct GBM/EGL surface for every
export while retaining the EGL context and GL pipeline. Physical startup
presented the first two mixed submissions successfully. The third render then
aborted with `amdgpu: The CS has been rejected ... (-2)`. This falsifies the
assumption that avoiding same-surface reuse alone is sufficient on this stack:
the context and pipeline cannot safely be rebound across independent leased
window surfaces either.

The fail-safe path again destroys the composition context and pipeline after
every successful export while the exported buffer retains its surface through
KMS retirement. Resource evidence still distinguishes complete-target and
frame-surface creation so this invariant is machine-checkable. The next
optimization is a bounded generational pool of complete targets, not a pool of
surfaces behind one context. A slot may become free only through explicit
page-flip retirement, never through reference-count inference.

<!-- END IMPORTED BODY -->
