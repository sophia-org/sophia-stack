---
id: legacy-active-0028
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "shell", "architecture"]
---
# 2026-08-27: Sophia's WM and shell protocols are the product critical path

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 951–991. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The xmonad bridge was retained as a mature behavior oracle, compatibility
  profile, and convenient four-window workload while the public protocol was
  being frozen. It is not Sophia's architectural WM. Native Hagia already
  speaks `sophia_wm_v1`, so further bridge-specific repair does not belong on
  the product critical path unless the same failure reproduces across the
  Sophia WM or shell protocol boundary without the bridge. Hagia is the first
  proof-of-concept client of those interfaces, not a protocol or the product
  architecture.
- Signed source `c681f762a4c8d8adada444749c90762b1ceef212`
  confirmed the surface-generation correction in the xmonad workload: stale
  responses fell from 19 to three, and all three coincided with a newly
  appearing surface. Each recovered through one private-adapter rebuild;
  ordinary Kitty repaint traffic no longer caused the stale/rebuild storm.
- That run then failed for a compatibility-only reason. Stable
  `SceneChanged` cycles produced `legacy WM did not configure all N synthetic
  windows within 3000 ms (configured 0)` for retained window counts 1, 2, 3,
  and finally 4. The bridge currently treats each synthetic
  `ConfigureNotify` as requiring a ConfigureRequest reply, while real xmonad
  may remain silent when geometry is already unchanged. Direct launch and
  close operations appeared slow only because they were ordered behind those
  three-second bridge waits. The fourth occurrence exhausted the supervisor
  after one successful close and ended the session with exit 1. Native
  scanout drained with zero abandonment, protocol and frontend cleanup stayed
  clean, and TTY recovery was exact. No action-1 record was present, so this
  attempt did not re-prove `Super+J`.
- Retain that exact defect as a compatibility follow-up at
  `LegacyX11WmBridgeRuntime::handle_request_once`: a future regression should
  allow a quiet stable relayout across one through four retained windows while
  preserving the fail-closed fence for a new or resized window that genuinely
  owes layout. Do not increase the three-second timeout; zero replies arrived.
- Resume product work with the native Hagia login/session and the same bounded
  interaction: three terminal launches, visible focus-next, one close, and
  normal logout. Use Hagia to exercise the Sophia WM and shell protocols, and
  inspect their authority separation, Engine layout/focus integration, native
  frame slots, latency, and cleanup directly. Only a defect reproduced across
  those product interfaces blocks Milestone 14. A clean protocol-level run
  promotes the three-slot boundary and unlocks bounded buffer-age damage
  history.

<!-- END IMPORTED BODY -->
