---
id: legacy-active-0601
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy", "validation"]
---
# 2026-09-04: cursor qualification exposed stale public-policy focus

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19006–19040. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Owner-only run `cp14-schema4-5897b3be` remained at 0/36 and retained no
comparison attempt. Its excluded qualification physically proved the preceding
cursor-plane repair: both heads presented through the atomic path, pointer
motion was observed and routed, and the cursor crossed from output slot 0 to 1
and back. About twenty seconds after the qualification surface disappeared,
Hagia rejected the next complete transfer with `policy output focus is
invalid`. Sophia then exited through its bounded recovery path.

The public snapshot was internally inconsistent. Surface records came from the
current live layout, so the withdrawn qualification surface was correctly
absent, but each output copied focus from an older committed projection without
checking that same surface set. The reducer canonicalized its retained scene
after observing the snapshot, while the session sent the pre-canonical local
value to Hagia. Hagia's independent codec correctly requires focus to name a
live, focusable, non-minimized surface on the same output and failed closed.

Snapshot construction now derives focus from the same complete live surface
set and clears any stale, cross-output, non-focusable, or minimized identity.
The WM wire encoder and decoder enforce the same complete-transfer invariant,
and Engine policy-scene validation independently rejects it at the authority
boundary. Focus therefore cannot become a dangling identity even if another
producer or client bypasses the live-session repair. Targeted regressions cover
all three boundaries. This run is immutable diagnostic evidence and must not be
retried after the source change.

Cursor appearance remains a separate product gap. The current live renderer
uses one built-in bitmap; the target contract is semantic cursor shapes resolved
by Engine under a configurable theme and nominal size, with validated hotspots
and a deterministic fallback. This lets a shell or session profile select the
same default cursor theme as XLibre without giving a policy client renderer or
KMS authority. Desktop-comparison profiles must pin that choice so cursor style
cannot vary between stacks.

<!-- END IMPORTED BODY -->
