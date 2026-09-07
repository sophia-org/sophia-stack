---
id: legacy-active-0039
date: 2026-08-25
recorded_date: 2026-08-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering", "security"]
---
# 2026-08-25: startup output activation now enters the frame-fed authority path

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1261–1283. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- An accepted startup desktop-output plan no longer stops at validation. The
  session projects it into the same resource-free topology candidate used by the
  live output authority, including normalized logical geometry, requested modes,
  transforms, scale, VRR, mirroring, and primary/focus selection.
- The public WM admits that candidate as a private startup transaction. It is not
  sent to an output-policy peer and is not abandoned when that peer disconnects;
  nevertheless it uses the ordinary quiescence, committed-state composition,
  native renderer preparation, atomic apply/rollback, first-presentation, and
  publication sequence. Hardware state is not published while either the startup
  transaction or a peer transaction is active.
- This is the frame-fed rather than scratch-fed activation promised by the
  renderer-import boundary: output effects cannot dispatch before the visual
  runtime exists, so every KMS candidate names a frame composed from committed
  scene state. A failed apply restores both hardware and the candidate frame;
  successful publication waits for presentation.
- `cargo test --offline -q -p sophia-cli --all-features`, formatting, and the diff
  check pass. The projection regression covers profile modes, negative-origin
  normalization, transforms, scale, VRR, and focus. The remaining evidence is a
  deliberately authorized physical session applying and, under an injected
  failure, rolling back the candidate on the reference DRM hardware.

<!-- END IMPORTED BODY -->
