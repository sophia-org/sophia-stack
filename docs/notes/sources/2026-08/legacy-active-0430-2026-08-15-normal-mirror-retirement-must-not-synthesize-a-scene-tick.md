---
id: legacy-active-0430
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: normal mirror retirement must not synthesize a scene tick

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12997–13020. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Attempt `0010` on `36842020` completed native presentation and shutdown but
  failed operator confirmation. The next changes made mirror retirement visible
  to the frame service and corrected scaled GL sampling. Attempts `0011` through
  `0013` then failed consistently at first-surface arrival with
  `engine backend tick failed: invalid surface ID`; the final diagnostic named
  committed surface `2097164:1`, one committed surface, and zero templates.
- The paired-scene fixes in `1e459b30` and `29572b21` remain valid, but the
  failing input did not originate in the visual runtime. Normal
  `PollRetirement` called the mirror's full scheduling tick with
  `CompositorBackendTickInput::default()`. That lower layer manufactured an
  empty template list while the output assembly retained the xterm surface.
- Normal mirror retirement now performs callbacks, group joining, logical
  page-flip publication, cleanup retry, and successor promotion only. Engine
  projection and renderer/KMS submission remain exclusive to an explicitly
  supplied scene tick. All scene-producing paths derive templates from the exact
  committed slice installed into the output runtime.
- Controlled owner-loop failures now stop frontend intake and enter the same
  bounded native/presentation completion used for client-fatal failures before
  returning the original error. Diagnostic verification requires that cleanup
  evidence after native bootstrap; signal-terminated processes remain exempt.
  The physical tty4 gate remains required from a clean signed successor.

<!-- END IMPORTED BODY -->
