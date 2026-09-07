---
id: legacy-active-0037
date: 2026-08-25
recorded_date: 2026-08-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "architecture"]
---
# 2026-08-25: xmonad and restart recovery now cross the public revision-3 boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1205–1237. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The xmonad session runner now selects `sophia_wm_v1` and a dedicated bounded
  desktop profile. Its bridge accepts profile activation, publishes the exact
  action catalog, consumes complete metadata-free scenes, runs the checked-in
  xmonad configuration behind the private synthetic X server, and returns
  canonical output projections. Terminal and close remain opaque
  session-operation slots; no client metadata or real X identity crosses.
- Exercising the real xmonad binary against the two-output scene exposed one
  compatibility bug that the old API-v7-shaped unit path hid: legacy geometry
  was clamped against the union root unless the request was a pointer gesture.
  Every request kind now scopes translation to its affected output. The full
  configured eleven-scene public corpus passes across five fresh processes,
  covering normal replacement plus timeout, stale, and invalid recovery.
- The shared black-box host now has a two-process restart mode. Rust, current C,
  and independent Nim clients pass all eleven scenes at fresh connection epochs
  while the canonical reducer proves the last committed projection unchanged
  across replacement. Hagia's complete policy/profile gate passes with that
  additional run.
- `protocol/archive/sophia-wm-v1-r3` pins a candidate C99 codec, client, schema,
  and fixed digests. Its gate compiles those copies without the live generated
  binding and proves both the retained and restart corpora against the current
  server. It becomes the permanent stable-client archive only when revision 3
  freezes.
- Installed xmonad releases now carry the bounded public-policy desktop profile
  beside the Engine theme. Release-manifest schema 4 pins both files, the
  packaged verifier requires their exact digests, and the installer regression
  rejects a changed desktop profile before promotion.
- The retained ledger closed on 2026-08-26 at 21 Complete, 0 Partial, 0 Open,
  and 7 Excluded. Signed frame-fed output archive `0001` supplies the final
  physical apply/rollback evidence. Interface major 1, wire revision 3 is
  stable; API v7 removal is the next critical-path tranche.

<!-- END IMPORTED BODY -->
