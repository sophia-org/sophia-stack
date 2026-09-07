---
id: legacy-active-0600
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# 2026-09-04: output-policy reconstruction dropped the atomic cursor plane

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18977–19005. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first row of owner-only run `cp14-schema4-ba438555` retained no attempt and
left the matrix at 0/36. Sophia reached its two-head synchronous baseline,
selected the accepted atomic cursor path, prepared and committed the desktop
profile topology, and published topology epoch 2. Immediately afterwards the
owner loop failed closed with `atomic cursor head lost its selected plane`,
drained native scanout, stopped the frontend and supervised policy processes,
and returned through the bounded display-manager recovery path.

The failure was deterministic state loss, not cursor-plane rejection. The
topology planner reconstructed an enabled head selection from its fixed
connector, CRTC, and primary-plane handles plus the candidate mode. The
constructor correctly defaulted its optional cursor plane to absent, but the
planner did not reattach the cursor plane already discovered for the same CRTC.
The runtime had selected the atomic path before applying the startup policy, so
the first post-commit cursor service observed an impossible half-state: atomic
policy and cached cursor properties without the selected plane.

Output-policy transactions do not rediscover or reroute KMS objects. Candidate
selection now carries the current head's optional cursor plane beside the
unchanged connector, CRTC, and primary plane. The previous selection already
serves as the rollback route, so both sides retain the same discovered cursor
identity. The public topology-planner regression supplies a current selection
with a cursor plane and requires both candidate and rollback selections to
preserve it. The complete backend-live all-feature test set passes. A fresh
signed candidate and prepared run are required; the zero-row run remains
immutable diagnostic evidence.

<!-- END IMPORTED BODY -->
