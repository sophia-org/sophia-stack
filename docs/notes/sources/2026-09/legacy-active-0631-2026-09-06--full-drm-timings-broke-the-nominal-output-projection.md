---
id: legacy-active-0631
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-09-06 — Full DRM timings broke the nominal output projection

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20317–20337. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The following login passed argument validation and discovered both heads, then
exited with `InvalidTopology` for DP-1. Read-only DRM connector inspection found
several distinct modes sharing a resolution and nominal refresh, including three
1920×1080 modes at 60 Hz. Retaining complete modelines made these backend entries
distinct, but their projection into the profile's nominal timing table contained
duplicates and failed its uniqueness check.

The profile projection now deduplicates nominal timings while the backend retains
complete modelines. Profile requests without a modeline resolve to the first
advertised nominal match; requests carrying a modeline require an exact match.
Opaque output-authority mode IDs resolve through the same bounded backend table
that advertised them, preserving the selected modeline instead of reconstructing
an incomplete timing. All three cases have regression coverage without hardware.
The three new regressions, 42 existing session output tests, and all 277 backend
tests passed. The release build and complete `cargo xtask check` passed. The full
gate ran outside the tool sandbox with isolated XDG configuration because its
Unix-socket fixtures cannot bind inside that sandbox. The installed release still
needs replacement before another physical login.

<!-- END IMPORTED BODY -->
