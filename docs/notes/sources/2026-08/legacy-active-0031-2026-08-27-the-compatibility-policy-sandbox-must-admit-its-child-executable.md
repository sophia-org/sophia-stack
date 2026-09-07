---
id: legacy-active-0031
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-27: the compatibility policy sandbox must admit its child executable

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1040–1058. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first signed four-Kitty run after the three-slot change never reached the
  workload. Native startup and exact retirement were clean, but the public
  xmonad bridge failed three supervised starts because its separately built
  xmonad executable was outside the bridge's bubblewrap filesystem. The
  executable existed on the host; `canonicalize` correctly returned `ENOENT`
  inside the policy domain. This is launcher evidence, not a frame-slot
  failure.
- Public policy configuration now has an explicit, repeatable
  `--wm-process-executable-grant` capability. Each value must be an absolute,
  executable file and enters the spatial-policy domain as one read-only
  binding. The xmonad compatibility runner grants exactly the binary returned
  by Cabal; it does not expose the build directory or the user's home. Hagia
  needs no grant because it does not launch a child policy executable.
- Parser, protection-domain, and runner regressions keep the grant conditional
  on a configured policy process and reject relative paths. A new signed
  physical four-Kitty run remains required.

<!-- END IMPORTED BODY -->
