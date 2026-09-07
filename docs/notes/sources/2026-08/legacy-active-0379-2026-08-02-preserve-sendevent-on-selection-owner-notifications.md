---
id: legacy-active-0379
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-02: preserve SendEvent on selection-owner notifications

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11559–11579. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The detailed follow-up again proved Firefox-to-Kitty PRIMARY through the
  coordinator's exact-token checkpoint. On the reverse transfer Firefox sent
  its target negotiation and content conversions; Kitty changed the Firefox
  property and Sophia routed successful SelectionNotify events, but Firefox
  never followed them with GetProperty. This placed the failure after routing
  and before the requestor's property read.
- XLibre `ProcSendEvent` unconditionally adds `SEND_EVENT_BIT` to the delivered
  event type. yserver follows the same rule by copying the supplied event and
  setting bit 7 before per-recipient fanout. Sophia's SendEvent decoder instead
  converted the event into a typed SelectionNotify without retaining that
  semantic, and its encoder consequently wrote ordinary event type 31 instead
  of synthetic type `0x9f`. Simple wire clients tolerated the difference;
  Firefox did not accept the owner notification as the expected SendEvent.
- SelectionNotify now carries an explicit synthetic flag. Client SendEvent
  decoding sets it regardless of the template bit, while server-generated
  negative and clipboard-proxy notifications remain ordinary events. The
  existing bidirectional same-namespace regression now asserts exact `0x9f`
  delivery in both directions before reading and deleting each property.

<!-- END IMPORTED BODY -->
