---
id: legacy-active-0431
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy"]
---
# 2026-08-15: mirror reduction needs a text-preserving sampling policy

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13021–13047. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The latest physical run on signed commit `515d0f52` brought both heads up at
  their native modes and showed the same centered scene, but the 1920x1080 head's
  6x13 terminal text was visibly blocky. The renderer rasterizes the logical
  2560x1440 scene once and reduces it by 0.75 for that head. Bilinear filtering
  avoids nearest-neighbor row loss, but it still softens the contrast of
  one-pixel bitmap-font stems enough to make the scaled terminal materially worse
  than the native-size head.
- Composition sampling is now explicit and observable: exact-size draws retain
  nearest sampling, reductions use a fixed 4x4 Catmull-Rom reconstruction, and
  enlargements use linear filtering. If the sharp shader cannot compile, the
  renderer falls back to linear but emits fallback evidence; the physical gate
  rejects that fallback and requires exact sampling on the 2560x1440 head plus
  sharp reduction on the 1920x1080 head. A CPU reference regression exercises
  mixed-case X.Org 6x13 glyphs at the physical 0.75 ratio.
- The same attempt's status 1 was separate from image quality: one client key was
  still pressed when the runtime deadline ended. Deadline completion now stops
  accepting physical input, synthesizes releases through the existing ordered
  input path, and waits up to 500 ms for delivery and release-barrier
  acknowledgements before native teardown. The gate requires a clean key record
  as well as operator confirmation that both heads show identical, stable text
  and that the reduced head is not blocky or stair-stepped.
- These changes have deterministic renderer, reducer, and verifier coverage, but
  they are not physical acceptance evidence. A clean signed successor must rerun
  the tty4 gate.

<!-- END IMPORTED BODY -->
