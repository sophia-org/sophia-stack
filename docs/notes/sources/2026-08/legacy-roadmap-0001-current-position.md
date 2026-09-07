---
id: legacy-roadmap-0001
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Current Position

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 28–83.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

<!-- BEGIN IMPORTED BODY -->

Sophia's product path is its native **Sophia X Server Frontend**. Engine owns
physical input, focus authority, scene state, rendering, presentation, and
scanout. X Authority owns X11 protocol semantics and private client resources.
One versioned, protocol-neutral WM API accepts native Sophia policy clients or
legacy-X11 policy translated through a private compatibility bridge. Xmonad is
the first mature bridge profile and current promotion vehicle; it is not
Sophia's architectural WM. XLibre and Wayland prototypes remain under
`research/` as architectural evidence.

The currently retained installed candidate provides:

- guarded two-output startup and exact TTY restoration;
- automatic Kitty, supervised xmonad, and optional unmodified xmobar;
- physical keyboard, pointer, focus, workspace, resize, clipboard, Firefox,
  floating-dialog, and normal-logout workflows;
- Engine-owned KMS presentation, protocol-neutral cursor and input policy,
  native chrome, and retained-frame recovery across VT release; and
- commit-pinned normal, fallback, watchdog, emergency, native-chrome, and
  switch-away/switch-back evidence with exact runtime identity.

Milestones 9 through 12 are complete historical evidence for the xmonad
compatibility profile and are archived in `docs/roadmap-history.md`. Their
bounded lifecycle, recovery, color, work-area, and soak artifacts remain
reproducible regressions, but elapsed wall time is not a current promotion
criterion. Milestone 13's installed product path is complete. Hagia is the
ordinary remembered installed session, records every real session
automatically, and leaves Kitty, xmonad, and the previous immutable release
available for recovery. The retained Triad behavior port is complete;
`sophia_wm_v1` interface major 1, wire revision 3 is frozen; and API v7 plus its
Engine-owned workspace policy are removed. Schema-5 packaged-promotion archive
`0002` binds signed Sophia source `66792329d90d64e26af839dfe494c74d94323c6a`
to Hagia's signed generic default, proves the repaired GLX first-pixel admission
through sustained final-extent presentation, and ends with normal Logout and
clean health. Mutable XDG policy remains confined to the ordinary dogfooding
entry. Milestone 14's three-slot boundary is promoted on signed native archive
`0001`, which also made Hagia the first proof client of the Sophia WM and shell
protocols with no compatibility bridge in the session. Bounded buffer-age damage
history is promoted on signed native archive `0002`. The one-in-flight and
refresh-relative latency row is proved on physical run `20260828T231430Z`
(source `96b00d0d`): full chain p99 24 ms against the two-refresh budget over
two hundred forty-five independent presses with clean stage percentiles.
Native archive `0003` promotes one shared renderer worker per DRM device
group, with both heads of one card on one thread and no result reaching an
output that did not ask for it. Direct-scanout archive `0001` promotes
atomic-test-gated direct scanout for one compatible opaque DMA-BUF layer:
thirty-eight client buffers reached the plane from one validating commit, with
no test rejections, proof disagreements, or fallbacks. Returning a directly
scanned output to composition on effect activation is the active product step,
with the hardware cursor plane behind it.

The current Void host has the required xmonad-configuration build and runtime
dependencies installed. Dependency installation is complete and is not an
active roadmap item.

<!-- END IMPORTED BODY -->

## Archived subsections

- [Active Critical Path](legacy-roadmap-0002-active-critical-path.md)
- [Immediate Next](legacy-roadmap-0003-immediate-next.md)
- [Production Readiness Infrastructure](legacy-roadmap-0004-production-readiness-infrastructure.md)
