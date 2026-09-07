---
id: legacy-active-0153
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-02: Rejected WM proposals require a bridge reseed

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4902–4923. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical run with control-time XI2 focus passed the repeated focus
handoffs and reached the second Firefox launch. Sophia then exited cleanly with
status 1 and `UnknownSurface`; there was no panic or memory fault. The earlier
Firefox popup admission had timed out, so Engine correctly preserved its prior
workspace state, but the xmonad bridge had already added the popup to its
private synthetic X11 model. When the second Firefox surface arrived, xmonad
returned tiling commands for that stale popup and strict workspace validation
terminated the owner loop.

An external WM necessarily applies a request before Sophia can prove the
resulting resize. A rejected or timed-out WM proposal therefore invalidates
the peer's speculative model even while Engine state remains correct. Timeout
results now retain their WM proposal source. The owner uses that evidence to
request the existing bounded transport restart, discards queued and in-flight
requests, and reseeds the restarted bridge from the last committed layout.
Non-WM resize timeouts remain local and do not restart the bridge. A regression
locks down source retention and the reseed decision; a fresh physical workflow
must confirm that popup timeout recovery and the second Firefox launch remain
live.

<!-- END IMPORTED BODY -->
