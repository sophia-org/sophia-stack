---
id: legacy-active-0355
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-08-01: the physical Firefox gate must isolate browser process state

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10976–10993. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first physical run after the stable browser binding launched Firefox and
  loaded the offline page, but only a white client area and the ordinary Sophia
  frame were visible. The parent window committed its CPU backing snapshot
  while a second Firefox surface continuously submitted a 1-by-1 DMA-BUF for a
  1276-by-1422 logical surface; the renderer correctly rejected every mismatch
  instead of scaling it. No interaction stage completed.
- The physical launcher had diverged from the passing QEMU workload: it reused
  the operator's normal Firefox profile and omitted QEMU's native-X,
  single-process, and XI2 controls. That also contradicted the XI2 wheel finding
  above, because the physical browser never received `MOZ_USE_XINPUT2=1`.
- Milestone 10 now creates a private run-local Firefox profile with the same
  bounded preferences used by QEMU, passes it explicitly with `--profile`, and
  forces native X11, disabled e10s/fission, and XI2 for that proof. A launcher
  regression retains the complete configuration. Renderer scale policy and
  protocol-neutral Engine routing are unchanged.

<!-- END IMPORTED BODY -->
