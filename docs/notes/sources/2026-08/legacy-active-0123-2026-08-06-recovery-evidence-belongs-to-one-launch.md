---
id: legacy-active-0123
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation"]
---
# 2026-08-06: Recovery evidence belongs to one launch

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4041–4055. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The live session rotated its launch, runtime identity, lifecycle, input-guard,
and session logs, but appended every TTY handoff to one `recovery.log`. That
file could grow without bound, and a later immutable attempt could inherit
recovery records from unrelated launches.

One shared lifecycle helper now rotates all active reduced logs to a current
file and one `.previous` generation. The runner creates an empty, private
recovery log before preflight, so even an early failure cannot reuse older
handoff evidence. The installed wrapper uses the same boundary for launch and
runtime identity. Regressions cover replacement semantics, private modes,
preflight isolation, and installed-wrapper rotation; promotion archives remain
immutable and checksummed rather than being silently pruned.

<!-- END IMPORTED BODY -->
