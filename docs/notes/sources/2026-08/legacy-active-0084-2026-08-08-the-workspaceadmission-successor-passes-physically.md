---
id: legacy-active-0084
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-08: The workspace/admission successor passes physically

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2872–2892. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed normal-session run `0055` binds its checksummed archive to release
`0.1.0-a2fdf4f69dfb` and commit `a2fdf4f6`. The automatic login, runtime-
identity, and lifecycle verifiers all pass: Sophia reached both outputs in 305
milliseconds, returned through normal logout after 350,850 milliseconds, and
left no application group, frontend worker, namespace, Xauthority file,
in-flight presentation, or pending WM/input work.

The operator confirmed that both `glxgears` and `vkcube` animated correctly.
Their two animated workload surfaces independently retired 5,367 and 5,384
frames, while the bounded cadence summary observed 8,192 advancing intervals
and no nonadvancing interval before its sample counter filled. Kitty and
Firefox action launches reached PresentedBuffer admission, and the session
retained seven workspace-away projections followed by visible workspace
returns. Forty-two WM projections committed with zero transport rejection,
stale response, or pending request. The reduced log contains no hidden-surface
configure/render command, layout timeout, resize abort, or WM restart; final
session and layout health are clean. This closes the short successor gate and
makes the two-hour interactive soak the next promotion gate.

<!-- END IMPORTED BODY -->
