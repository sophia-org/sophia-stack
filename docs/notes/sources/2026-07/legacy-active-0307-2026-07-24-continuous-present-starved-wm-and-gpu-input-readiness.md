---
id: legacy-active-0307
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "rendering", "policy"]
---
# 2026-07-24: Continuous Present Starved WM And GPU Input Readiness

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9544–9610. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The installed xmonad proof still appeared to lose both keyboard and pointer
input after tracing was disabled. The retained session showed fourteen active
libinput devices, stable mixed Present retirement, a visible hardware cursor,
and Engine focus, but no WM layout commit, applied X11 focus, or physical-input
readiness marker. Disabling tracing changed timing only; it could not repair
either missing state transition.

Initial WM management waited for 500 milliseconds without any authority work.
Kitty's continuing Presents reset that global timer, so xmonad could remain
ready yet never receive its first opaque surface. Unmanaged surfaces are now
submitted whenever no WM transaction is pending, one at a time, independent of
application frame cadence. Startup and proof readiness also track the surface
whose X11 focus was actually acknowledged instead of treating the presence of
an external WM as proof that focus was applied.

The same stable DMA-BUF retirement that satisfied visible startup did not set
the older CPU-only terminal-content flag. The proof therefore never armed and
the session deliberately skipped libinput polling. Focused content readiness
now accepts either CPU visual detail or a stable retired Present belonging to
the focused surface. Libinput is always drained: before proof readiness,
pointer motion updates only the compositor-owned cursor while ordinary keys
and buttons are discarded without entering the exact-text matcher or X11
route. The installed gate now requires exact `sophia` input followed by routed
pointer motion and one button. All decisions use opaque surfaces and generic
presentation facts; Engine contains no Kitty-specific behavior.

The first installed rerun confirmed the WM, focus, cursor polling, and stable
Present changes, but physical input still remained in cursor-only mode. The
proof had a second baseline predicate requiring nonzero CPU-scene pixels even
after the focused stable DMA-BUF surface was accepted. GPU-only Kitty therefore
reached content readiness without satisfying the duplicate CPU gate. Baseline
readiness now consumes the same focused-content fact used by startup and input
arming; CPU composition remains an alternative source rather than an additional
requirement. A regression fixes the GPU-ready/CPU-empty combination.

The next installed run proved that exact physical `sophia` input reached the
shell and all fourteen X11 events flushed. Thirty later authority batches and
stable native Presents followed, but the post-input verifier still required a
CPU-buffer checksum or generation change and timed out a GPU-only terminal.
A stable retired Present on the exact proof surface after input delivery now
provides the corresponding GPU presentation evidence.

That run also exposed a separate cursor ordering error. Once text input was
ready, pointer motion entered full routing mode, but application pointer
delivery remained gated until the later pointer-proof phase. Cursor placement
was incorrectly behind that delivery gate, freezing the compositor-owned
hardware cursor. Placement now occurs before the application-delivery decision,
so cursor motion remains responsive without prematurely routing pointer events.

The following physical run displayed a blinking prompt but did not echo input.
Its retained evidence contained `key_observed` without `key_routed` and ended at
`focus_control_pending`. The WM layout committed before the corresponding
surface entered Engine's committed-surface set; the first focus attempt returned
`UnknownSurface`, but the owner loop consumed the one-shot focus request anyway.
WM focus requests now remain pending on that transient result and are consumed
only after Engine focus and X11 client focus both succeed.

The retry run then committed focus, armed physical input in 800 milliseconds,
and routed keys. It stopped only because the proof matcher required strict
press-release pairs and rejected ordinary keyboard rollover when `a` was
pressed before `i` was released. The proof now validates exact character press
order while independently tracking balanced releases, accepting natural
overlap without weakening modifier, repeat, unexpected-release, or submit-key
checks.

<!-- END IMPORTED BODY -->
