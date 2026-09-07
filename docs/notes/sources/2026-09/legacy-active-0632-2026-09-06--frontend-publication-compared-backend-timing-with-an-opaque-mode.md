---
id: legacy-active-0632
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-09-06 — Frontend publication compared backend timing with an opaque mode

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20338–20366. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Release `3b0d66580813` passed the repaired mode projection, applied both heads,
and presented their candidate frames. It then rolled back with `candidate
authority and native topology projections disagree`. The publication gate
compared whole snapshots: native resolution now retained the real modeline,
while the logical output authority correctly supplied no DRM timing metadata.
The preceding regression stopped at mode resolution and missed this later gate.

Output publication now checks every shared topology field against the authority
and publishes the native snapshot with its retained timing. DRM metadata stays
in the session/backend boundary; the output authority protocol gains no fields.
The projection functions live together in `desktop_output_publication`, used by
the real publication path and its hardware-free integration test. The test now
follows both same-rate modelines through candidate projection and frontend
publication, checks that timing survives, and rejects geometry or nominal-rate
drift. Synthetic outputs may still lack measured timing.

The targeted output tests, release build, and complete `cargo xtask check` pass.
The gate ran on the host with isolated XDG configuration for its socket fixtures.
Physical acceptance remains pending installation and the next operator login.

The operator reported a live session after installing `3fc0ab14d264`. Logs from
the 19:26 UTC login confirm revision-4 shell negotiation, frontend candidate
publication, committed topology for both heads with no pending cleanup, and
startup readiness with zero recovery attempts. This accepts installed startup.
Subsequent launcher use and a browser typing test exposed the failures below;
startup acceptance does not establish continued session reliability.

<!-- END IMPORTED BODY -->
