---
id: legacy-active-0497
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-21: kernel hotplug reaches a stale connector snapshot

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15200–15220. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- `/tmp/sophia-output-topology-20260821-232830.log` closes the event-ingress
  question: monitor completion reports `observed=2 coalesced=0 delivered=2`,
  and both events advanced the input security epoch, quiesced scanout, rebuilt,
  presented, and settled cleanly.
- Both publications nevertheless reported `outputs=3 changed=false` at the
  original generation. Native KMS selection called `get_connector(..., false)`;
  libdrm-rs documents that callers must force-probe at startup and after a
  hotplug so connector status, modes, and EDID are current. The rebuilt runtime
  therefore consumed the precise stale cache the hotplug was meant to replace.
- Native KMS discovery now requests the forced connector probe. Completion also
  distinguishes a generation-advanced topology replacement from a steady
  session: newly recreated heads may finish on their verified synchronous
  modeset without satisfying the steady proof's per-current-head nonzero
  asynchronous lifecycle. The caller-specific cable gate remains responsible
  for proving two changed publications, loss generation 2, return generation 3,
  both policy settlements, and post-policy presentation.
- This run remains failed evidence. A signed physical rerun is required before
  the roadmap item can be promoted.

<!-- END IMPORTED BODY -->
