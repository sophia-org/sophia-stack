---
id: legacy-active-0354
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-01: xmonad application keys identify semantic slots

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10958–10975. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first self-guided physical run proved both Kitty checkpoint clients and
  then rejected every `Super+F` request as `UnavailableSessionAction`; Firefox
  never spawned. The session descriptor contained terminal and browser launch
  actions but no application-menu launcher.
- The compatibility bridge previously selected the first, second, or third
  launch action remaining in the negotiated descriptor. The xmonad bindings
  themselves are semantic—`Super+Enter` is terminal, `Super+P` is the launcher,
  and `Super+F` is the browser—so filtering an unavailable middle application
  incorrectly shifted later meanings while leaving their keys fixed.
- Translation now maps those three profile actions to stable application IDs
  1, 2, and 3, then applies the existing descriptor admission check. A focused
  regression requires `Super+F` to launch application 3 when applications 1
  and 3 are present and requires the absent application-2 binding to fail
  closed. Engine still receives only the negotiated protocol-level session
  action and remains unaware of xmonad key semantics.

<!-- END IMPORTED BODY -->
