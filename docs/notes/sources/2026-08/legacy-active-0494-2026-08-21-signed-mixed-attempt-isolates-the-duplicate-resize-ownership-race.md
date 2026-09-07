---
id: legacy-active-0494
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-08-21: signed mixed attempt isolates the duplicate-resize ownership race

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15132–15156. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- `/tmp/sophia-mixed-output-DP-1-20260821-224908.log` binds the run to signed
  source `7ff94e2082c9ee284e08bff4c70c9750d612cfaa`. Three-head KMS committed the
  two-output topology, and schema-3 renderer evidence agreed with schema-2 plans:
  the optimized head was exact and the smaller mirror member was a sharp
  downscale. The run is still failed evidence: the reference policy never
  settled the final two-output placement, repeated layout epochs timed out, and
  shutdown retained one WM layout plus one request.
- The first failed epoch made the race exact. Epoch 10 installed both
  `1276x1436` content rectangles and armed Present 545 for the second surface.
  Before that candidate retired, an ordinary scene refresh produced epoch 11
  with the identical target. `PersistentLiveLayout::stage` compared the request
  only with the surface's last retired `1920x1080` size, sent another configure,
  and displaced the very candidate on which convergence depended. The timeout
  consequently reported `1276x1436:1920x1080` for that surface.
- Resize filtering now recognizes a target already installed in the committed
  layer and owned by `ResizeVisualCommitTracker`. The duplicate request is
  removed, so the proposal can settle without a second configure while the
  original candidate retains its right to retire. The geometry check is
  load-bearing: a standing recovery candidate for a target not yet installed
  must still drive its configure. Paired regressions prove both decisions, and
  all 223 feature-complete CLI tests pass. A successor signed physical run is
  still required.

<!-- END IMPORTED BODY -->
