---
id: legacy-active-0334
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "architecture"]
---
# 2026-07-26: External WM Focus Ownership Starts Before Reconciliation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10509–10544. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The guarded TTY3 pointer-focus run physically confirmed both plain-click focus
and click-drag focus. Each selected the intended Kitty, cross-window copy/paste
worked during the drag workflow, and Engine-owned focused borders made the
handoff visible. Workspace transitions exposed a separate transient: the
status bar briefly received a focused-border outline before the empty
workspace settled.

The defect was in generic initial-focus reconciliation, not xmobar or xmonad.
After external policy cleared Engine and X11 focus, the owner selected and
focused the first committed surface before checking whether an external WM
owned focus policy. It then returned without applying matching client focus.
The result was a hidden Engine focus, a one-frame compositor border, and
`control_plane_only` input suppression because Engine and frontend focus no
longer agreed.

Initial-focus candidate selection now rejects external-WM sessions before any
Engine mutation. A deterministic regression supplies a committed hidden
surface and requires no candidate. The two-output `xmonad-m7` guest then
switched to an empty workspace, recorded `focus=none`, suppressed both primary
button edges with `reason=no_target`, emitted no pointer-focus request or
client delivery, returned with Super-1, preserved focus through one
compatibility-bridge restart, and completed the independent click and drag
proofs plus clean logout. Reduced policy-suppression evidence remains available
for diagnosing future Engine/frontend focus transitions; it carries only mode
and counts.

The follow-up physical session exercised 36 workspace projections. Ten empty
projections retained `focus=none`; 26 populated projections restored focus.
No focused-border composition occurred between an empty projection and its
next legitimate focus restoration, no pointer button was suppressed by the
focus-transition policy, and the session completed with clean protocol, WM,
input, native-scanout, frontend, and namespace state. This closes the transient
status-bar border regression without adding client-specific chrome policy.

<!-- END IMPORTED BODY -->
