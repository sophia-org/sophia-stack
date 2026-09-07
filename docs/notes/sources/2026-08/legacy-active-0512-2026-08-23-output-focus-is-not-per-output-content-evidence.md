---
id: legacy-active-0512
date: 2026-08-23
recorded_date: 2026-08-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "policy", "validation"]
---
# 2026-08-23: output focus is not per-output content evidence

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15669–15693. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The corrected-chord Hagia run on signed source
`de0daad96cac0285e56602c0254642f7ba0ed84e` completed the causal epoch-two
restart, checkpoint recovery, fullscreen, layout, maximize, minimize/restore,
both output-focus actions, all 34 exact text events, a 13 ms physical-input to
page-flip chain, clean protected-broker shutdown, and bounded session cleanup.
The run still failed, correctly, before archival. DP-2 retired eight ordinary
submissions after its synchronous startup modeset, but all were blank;
`nonzero_exports=0` could not satisfy independent per-head presentation.

The procedure had asked Hagia only to change the active output. Focus policy
does not move a surface, so the one-window session had no way to present
nonzero content on DP-2. The gate now exercises the separate public
move-to-output actions: `Super+Shift+Right` moves Kitty to DP-2, the guide waits
for a nonzero schema-2 native-head submission there, and `Super+Shift+Left`
moves it back before the existing focus actions. The verifier requires opaque
actions 5 and 6 and binds the nonzero DP-2 submission between their commits.
The generic per-head completion rule stays strict.

This is another fixture-only correction. Sophia's executable, compiled policy,
and supervised application set remain unchanged, so mirror archive `0008` and
mixed archive `0002` remain candidate evidence. The next Hagia run must bind
the corrected signed source and reproduce their Sophia binary digest.

<!-- END IMPORTED BODY -->
