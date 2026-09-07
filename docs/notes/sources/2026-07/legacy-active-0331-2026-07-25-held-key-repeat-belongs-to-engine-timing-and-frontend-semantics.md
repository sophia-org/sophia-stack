---
id: legacy-active-0331
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-25: Held-Key Repeat Belongs To Engine Timing And Frontend Semantics

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10370–10401. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical run retained ordinary keyboard routing but exposed that
holding Backspace or an arrow produced only one action. This is expected from
libinput: it reports physical press and release edges; the display stack must
schedule held-key repeat. Sophia had no such scheduler.

Engine now owns a fixed-capacity, allocation-free repeat clock with one active
repeatable key per seat. The live owner binds that record to the exact focused
surface, seat, device, and physical key. The configured XKB map determines
repeatability, so editing and cursor keys repeat while modifiers and Super do
not. Existing focus, workspace, surface-removal, and VT release barriers cancel
the bound record; a repeat can never migrate to a newly focused client.

The X frontend receives an explicit repeat delivery mode. It emits another
KeyPress with current XKB modifiers without replaying a physical state
transition or reactivating a passive grab. This keeps Engine timing
protocol-neutral, preserves X11 delivery authority, and prevents xmonad global
shortcuts from repeating. Missed timer intervals coalesce to one pulse rather
than bursting after a slow frame. Completion evidence requires the scheduled
and routed counts to match, capacity exhaustion to remain zero, and every seat
to drain.

The first physical capture routed and acknowledged 66 held-key pulses with
zero missed-interval coalescing and zero repeat-seat capacity exhaustion. The
operator confirmed the editing/navigation behavior, and logout left no active
repeat seat or pressed-key debt after all 1,289 expected input deliveries
flushed. The run retained clean input, cursor, protocol, renderer, KMS, and
frontend teardown. It did not complete the broader promotion sequence: the
startup Kitty remained open and the clipboard peer performed no
`ConvertSelection`, so those independent gates remain pending.

<!-- END IMPORTED BODY -->
