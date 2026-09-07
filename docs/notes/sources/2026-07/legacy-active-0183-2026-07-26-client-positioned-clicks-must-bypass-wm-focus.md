---
id: legacy-active-0183
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-07-26: Client-Positioned Clicks Must Bypass WM Focus

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6254–6268. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first external-config promotion run completed every ordered reload phase,
then terminated when the operator clicked the status bar. Hit testing correctly
selected the `ClientPositioned` surface, but the generic primary-button path
started a managed focus handoff because the bar differed from keyboard focus.
The blind WM correctly had no workspace registration for that excluded
surface; treating its rejection as fatal ended the session.

Physical input now consults the existing protocol-neutral presentation-role
table before starting a focus handoff. A `ClientPositioned` target receives
button input directly while the current keyboard focus is retained.
`PolicyManaged` targets keep the ordered WM/Engine/frontend focus handoff.
This is role policy rather than a status-bar or xmobar exception.

<!-- END IMPORTED BODY -->
