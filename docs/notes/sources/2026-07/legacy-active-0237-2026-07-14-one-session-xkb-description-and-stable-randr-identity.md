---
id: legacy-active-0237
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11"]
---
# 2026-07-14: One Session XKB Description And Stable RandR Identity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7928–7977. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The native frontend now compiles one immutable xkbcommon snapshot from the
session RMLVO. Core `GetKeyboardMapping`, XKB `GetMap`, and per-seat event
translation consume that configuration instead of combining a handwritten US
wire map with an independently compiled state machine. The live command accepts
bounded `--xkb-rules`, `--xkb-model`, `--xkb-layout`, `--xkb-variant`, and
`--xkb-options` overrides; a German-layout regression proves that core and XKB
views change together.

RandR CRTC and output identities now derive from Engine `OutputId`, while mode
identity derives from the mode tuple. Reordering a topology snapshot therefore
does not renumber an unchanged output. Focus state is also namespace-local and
window destruction resets only its namespace. Dynamic RandR event diffs,
complete XKB state/name notifications, grabs, and XI2 event delivery remain
Milestone 3 work.

The follow-up dynamic path now acknowledges newer Engine snapshots, populates
`GetMonitors`, and sends mask-selected RandR screen, CRTC, output, and resource
notifications through each client's bounded protocol queue. A deterministic
`--inject-output-size=WIDTHxHEIGHT` live-session hook applies a validated
generation update after client startup, so update behavior can be retained as
evidence without requiring a physical connector hotplug.

The live resize rollback fence is now an exported coordinator rather than
private layout bookkeeping. It owns committed sizes, monotonic compensating
transaction IDs, abandoned-size filtering, and disconnect cleanup. Integration
tests cover successful advancement, timeout rollback construction, rejection of
late abandoned pixels until the old size is confirmed, and cleanup while a
rollback is pending. The live layout uses this coordinator for its existing
geometry-plus-pixels quarantine and compensating configure path.

Core input grabs now have connection identity and namespace-scoped authority
state instead of validation-only request handling. Active pointer/keyboard,
passive key/button with Any detail/modifier conflict checks, implicit button,
owner-events routing, synchronous freeze with bounded deferred input and
`AllowEvents`, ungrabs, and namespace-local `GrabServer` ownership all clean up
on disconnect. Engine still chooses the ordinary target surface and local
coordinates; the authority redirects only when X grab semantics require it.
XI2 generic-event delivery remains the next input-compatibility boundary.

That XI2 boundary now advertises XGE 1.0 and XI 2.0, reports master pointer
button/valuator classes plus the master keyboard key class, retains bounded
per-client selection masks, and emits selected Key, Button, Motion,
Enter/Leave, and Focus generic events. Device events preserve Engine-provided
root/local coordinates as FP16.16 values and follow core grab redirection. One
input delivery acknowledgement is returned only after the writer flushes the
core event and every selected XI2 record generated from it. Raw, touch, and
gesture events remain deliberately outside Milestone 3.

<!-- END IMPORTED BODY -->
