---
id: legacy-active-0360
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-01: Xorg marks button-derived smooth Motion as emulated

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11094–11117. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The next live physical run used the release binary built after commit
  `a12d5bd3`, with `MOZ_USE_XINPUT2=1`, and again remained at Firefox step 4.
  The operator then generated five deliberate wheel notches while the browser
  stayed open, but the session log could not distinguish them: schema-3
  `axis_observed` and `axis_routed` were intentionally lifetime one-shot
  markers. The new two-packet physical and QEMU checks had incorrectly treated
  those markers as per-packet evidence.
- The local X server implementation shows a second wire mismatch. When a
  physical Button4-Button7 press is converted into a smooth XI2 Motion event,
  Xorg sets `POINTER_EMULATED`; its XI2 encoder carries that through as
  `XIPointerEmulated`. The later compatibility XI2 ButtonPress/Release events
  are marked the same way. Sophia marked only the button pair, leaving its
  button-derived smooth Motion distinguishable from Xorg.
- Sophia now derives XI2 device-event flags from the protocol-neutral axis
  event and marks both the smooth Motion and compatibility button pair with
  `XIPointerEmulated`; ordinary pointer Motion remains unflagged. The owner
  loop additionally emits schema-9 `axis_batch` records containing only
  observed/routed counts. QEMU waits on the summed routed count, and the
  physical verifier sums only batches ordered between PRIMARY and DOM scroll.
  Mutation fixtures reject a single routed packet, without logging direction,
  values, coordinates, or timing.

<!-- END IMPORTED BODY -->
