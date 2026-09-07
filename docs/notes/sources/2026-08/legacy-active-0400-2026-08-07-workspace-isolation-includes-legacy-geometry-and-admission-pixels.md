---
id: legacy-active-0400
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# 2026-08-07: Workspace isolation includes legacy geometry and admission pixels

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12137–12180. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Installed preliminary soak attempt `0054` ran for 413,133 milliseconds and
  returned through clean normal logout and exact TTY recovery. Native scanout,
  callbacks, protocol handling, application cleanup, and input drain were
  clean, but the immutable verifier correctly failed the run after four layout
  timeouts and four xmonad-bridge restarts.
- The workspace-specific defect was exact. A Kitty launch on workspace 2
  committed one visible surface while moving and configuring three; the two
  extra surfaces belonged to hidden workspace 1. A later Firefox launch on
  workspace 3 repeated the three-surface configure set and timed out. The
  bridge removed hidden windows from its mapped set and filtered their focus,
  but translated late `ConfigureWindow` requests retained by xmonad's private
  state into real Sophia configure and render commands.
- The compatibility boundary now rejects geometry from every known but
  unmapped synthetic window while continuing to reject unknown windows as
  protocol errors. A pure translation regression mixes hidden and visible
  configure and focus requests. A process-level synthetic-X regression keeps
  stale layout state across unmap, emits both geometries during the next
  admission, and requires only the visible surface to cross the bridge.
- The same run exposed an independent first-admission ordering defect.
  Firefox, glxgears, and vkcube had already produced complete pixels at
  500-by-570, 300-by-300, and 500-by-500 extents, respectively. The first WM
  proposal nevertheless requested the final tile immediately, waited for
  pixels the clients had not produced, timed out, and only then used the safe
  extent that made the retry succeed.
- Admission now primes that existing Engine-owned safe observation as a
  temporary fixed extent before constraint reconciliation. Sophia retains the
  blind WM's different size as a standing target, commits and retires the
  selected pixels, clears the temporary extent, and drives the target through
  the ordinary exact-pixel relayout path. This is an event-driven ordering
  repair; it does not lengthen the two-second deadline, weaken atomic visual
  admission, or teach Engine about X11 or application identity. A short
  installed successor proof remains required before another long soak.
- The previous formal suite did not cover either boundary. `PolicyProjection`
  began after compatibility reduction, while `AdmissionRecovery` required a
  timeout before fallback. `LegacyWmProjection` now explores delayed configure
  and focus around complete workspace replacement. `AdmissionRecovery` now
  separates proactive safe-pixel priming from timeout without an observed
  candidate and proves that observed pending admission becomes managed. The
  pinned TLC gate passes all models; the new projection model explores 270
  distinct states to depth 10 and the revised admission model explores 84
  distinct states to depth 12.

<!-- END IMPORTED BODY -->
