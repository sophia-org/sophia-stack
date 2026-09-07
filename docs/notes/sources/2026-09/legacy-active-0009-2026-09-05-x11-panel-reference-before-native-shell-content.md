---
id: legacy-active-0009
date: 2026-09-05
recorded_date: 2026-09-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "shell"]
---
# 2026-09-05: X11 panel reference before native shell content

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 281–306. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Implement the bounded Quickshell panel first; keep DMS and the independent
native shell content contract outside this milestone. The
[launcher and focused check](../../../quickshell-x11-panel.md) use the production normal
session path with Hagia. OpenGL is the requested live default; software is an
explicit isolated probe, with no physical input or DRM acquisition.

The live-path exercise found that input/output profile readers still asserted
Prepared after public-policy admission had activated their slots. Runtime setup
now selects the payload for Prepared or Activated as appropriate. A second race
occurred when a dock strut changed work area while a terminal resize was waiting
for pixels: canonical revalidation returned RejectedStale and ended the session.
The old resize epoch now follows normal recovery and returns a stale settlement,
rearming policy without committing obsolete geometry.

The accepted local software evidence (`/tmp/sophia-panel-probe-v7`) contains
changing committed panel/popout content, correct anchoring, a terminal below the
panel, reservation release/reacquisition, zero protocol errors and clean control
teardown. This is not GPU, exact-pixel, physical-input or two-output acceptance.
The forced-deadline run v5 retains a separate one-control drain failure; normal
client exit in v7 does not resolve it. A default xterm invocation in v3 exposed
unsupported LookupColor; the witness uses the existing compatibility flags.
No shell wire revision, Quickshell fork edit or installed-session change was
needed. Keep standalone smoke results at their existing trace-only confidence.

<!-- END IMPORTED BODY -->
