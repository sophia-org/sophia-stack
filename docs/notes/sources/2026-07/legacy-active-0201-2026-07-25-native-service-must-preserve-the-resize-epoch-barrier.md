---
id: legacy-active-0201
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-25: Native Service Must Preserve The Resize Epoch Barrier

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6888–6922. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical four-Kitty run isolated the remaining corruption. Xmonad
requested three 1280-by-480 stack buffers, and Kitty produced them, but the
asynchronous native service scheduled the queued Presents while the WM layout
was still pending. The per-batch defer flag did not survive that service
boundary. Those buffers were compared with the old 1280-by-720 or
1280-by-1440 geometry and rejected; the 300 ms policy deadline then expired,
and the fourth surface remained visible at its 80-by-60 staging offset.

Presentation scheduling is now persistently blocked for the lifetime of the
pending layout. KMS retirement continues, but queued Presents cannot enter
scanout until the layout commits. A timeout rejects the complete quarantined
queue, preserves the prior displayed surfaces, does not focus the uncommitted
surface, and leaves admission eligible for one retry after rollback pixels
arrive. The xmonad bridge uses the existing bounded two-second maximum so a
multi-client resize is not failed merely because three clients repaint
serially. This state is surface- and transaction-based; no application-specific
branch was added.

The first validation run exposed a necessary refinement: the startup buffer
may precede the first WM configure. Holding that wrong-size Present withheld
the feedback Kitty needed before allocating the configured buffer, producing a
startup deadlock. A Present whose pixels conflict with the pending requested
size is therefore completed as a controlled skip immediately. Only matching
pixels, or pixels for a moved-only surface with no size request, enter the
quarantine.

The successful four-window transition also invalidated cloning the primary
mixed frame onto every output. The primary composition was 2560 by 1440 while
the secondary scanout target was 1920 by 1080, producing one controlled export
failure per primary Present and a final teardown error. Mixed X11 composition
now stays on its owning primary output; other outputs retain their own
output-sized frames until Engine has an explicit per-output scene projection.

<!-- END IMPORTED BODY -->
