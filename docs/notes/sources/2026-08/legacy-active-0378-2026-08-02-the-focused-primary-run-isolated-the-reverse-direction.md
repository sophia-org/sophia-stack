---
id: legacy-active-0378
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-02: the focused PRIMARY run isolated the reverse direction

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11535–11558. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The live pointer trace delivered physical button 2 to Kitty after the focus
  handoff. Kitty issued PRIMARY conversions; its GLFW diagnostic therefore did
  not implicate the evdev-to-core-button mapping. More importantly, the
  focused coordinator wrote `checkpoint-primary-received` at 20:28:00. That
  checkpoint is emitted only after Kitty reads the exact Firefox token, so the
  Firefox-to-Kitty same-namespace transfer passed during the run.
- After Kitty exposed and selected its return token, owner-change evidence
  advanced and Firefox issued new conversions, but the Firefox confirmed-title
  checkpoint never appeared. `Ctrl+Shift+C/V` exercised CLIPBOARD and was not
  evidence for this remaining PRIMARY direction. The old value-free counters
  cannot distinguish a negative SelectionNotify, a completed property read, or
  an exact-token mismatch.
- XLibre `ProcConvertSelection` and yserver `handle_convert_selection` both
  route SelectionRequest to the current owner and rely on the owner to change
  the requestor property and send SelectionNotify. Sophia's existing wire test
  covered that sequence only in one direction. It now reverses ownership and
  performs the complete property/notify/read/delete sequence back through the
  original requestor. The focused launcher also enables the already-redacted
  live stages (`request_routed`, property notification, notify, and property
  read), while the page reports a nonempty mismatched token separately from no
  paste. This preserves exact-token acceptance without another combined gate.

<!-- END IMPORTED BODY -->
