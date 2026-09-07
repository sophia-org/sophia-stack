---
id: legacy-active-0004
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "shell"]
---
# 2026-09-06: panel popout visibility and new-window camera focus

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 109–151. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Normal use of installed `8b750b30` exposed two issues. Clicking Panel test
blinked only the terminal border and did not show the counter popout. Opening
another terminal left the camera on the previous window. The live log was
copied to `/tmp/sophia-panel-camera-8b750b30/session.log`; it shows popup
descriptor admissions with no corresponding native popup sampling, a brief
unfocused/focused chrome transition, and no runtime fatal at capture time.
Terminal admissions retained the older focus (`AlreadyFocused`), while later
navigation actions changed focus. This is evidence of admission focus policy,
not evidence that Engine ignored a camera proposal.

The session's presentation filter required a client-positioned popup's owner
to have WM visibility, even when that owner was a client-positioned panel.
Panels intentionally bypass WM placement. Resolve their mapped owner chain
first, consulting WM visibility only at a managed ancestor. Missing generations,
unmapped ancestors and ownership cycles remain invisible. This keeps frontend
presentation facts in the session and workspace visibility in the WM; no
toolkit-specific exception or new protocol is needed.

Hagia now selects the newest eligible admission in the active view after
reconciling scene focus. Initial snapshot synchronization preserves existing
focus; background views, other outputs, minimized windows, popups and
non-focusable windows do not replace active focus. Unassigned admissions use
the active output instead of the first output. Existing camera projection then
reveals the selected window. Explicit request causes still run after
reconciliation, and rejection discards the complete focus/camera candidate.

Sophia's isolated `cargo xtask check` passes, including owner-chain visibility
regressions. The real software panel probe at `/tmp/sophia-popup-owner-probe`
passes content updates, popup withdrawal, reservation restoration and clean
exit. Its CPU content samples do not prove physical GPU visibility. Hagia
regressions cover repeated terminal admission, camera containment, rejected
candidate retry, initial/repeated snapshots and background admission exclusion.
Physical follow-up: the operator installed Sophia `05ef0eb8` with Hagia
`12f7493` and confirmed that the panel incrementer works and the camera follows
new windows in the scrolling layout. The log shows 240x112 popup sampling on
output 1, three added terminals receiving focus with committed layout moves,
and no runtime fatal at capture time. The session log and installed manifest
are retained in `/tmp/sophia-panel-camera-confirmed-05ef0eb8`. These two
normal-use checks are accepted; they do not claim both-output panel lifecycle,
vertical scrolling, close behavior or broader scrolling acceptance.

<!-- END IMPORTED BODY -->
