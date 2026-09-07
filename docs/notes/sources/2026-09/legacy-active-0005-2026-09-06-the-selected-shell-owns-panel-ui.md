---
id: legacy-active-0005
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell"]
---
# 2026-09-06: the selected shell owns panel UI

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 152–174. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The operator requested a single panel and clarified that the shell, rather than
Engine, should provide it. Retire the automatic Tier-0 bar instead of adding a
compositor setting to hide it. The session no longer adds a 14-pixel reservation;
the live renderer no longer instantiates the fixed indicator strip or its input
occlusion/targets. Shell and X11 client reservations remain authoritative for
work-area reduction. Existing user startup selections already launch Quickshell,
so no personal config change is needed and Narthex stays enabled.

Policy indicator publication remains available for shell projections and tab
descriptors. Removing the strip must not stop that data from updating. No WM
identity, executable or toolkit-specific rule crosses into Engine, and no new
wire protocol is introduced. Historical Tier-0 fixtures remain reference/archive
evidence; the current isolated panel verifier expects reservation release to
restore the full 720-pixel output rather than the former 706-pixel work area.

Validation: `cargo xtask check` passed with isolated test directories (2,439
Rust test executions plus Clippy and repository checks). The real isolated
software panel probe at `/tmp/sophia-shell-owned-panel-probe` passed, with work
area transitions y=32/height=688 to y=0/height=720 twice, popup withdrawal and
clean teardown. Physical single-panel acceptance remains pending.

<!-- END IMPORTED BODY -->
