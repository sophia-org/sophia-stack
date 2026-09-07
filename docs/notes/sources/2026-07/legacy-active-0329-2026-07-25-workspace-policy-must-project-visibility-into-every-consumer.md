---
id: legacy-active-0329
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-07-25: Workspace Policy Must Project Visibility Into Every Consumer

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10227–10299. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The post-cursor physical cycle validated the KMS repair with zero native submit
failures and a 12 ms maximum cursor update, then exposed an independent
workspace defect. Super-2 committed workspace policy and cleared focus, but the
owner continued presenting all four workspace-1 surfaces. The private xmonad
server also kept their synthetic windows mapped. Launching Kitty on workspace 2
therefore made xmonad tile five cross-workspace windows; all five resize
requests timed out, rollback rejected four queued Presents, and logout later
detached one unretired scanout. Shortcut IDs remained distinct: the log recorded
workspace actions 257 and 258, application action 768, close action 769, and
logout action 772.

Workspace focus is now stored by workspace and projected onto the outputs that
currently display it, so hiding a workspace clears client focus without
destroying the focus that should return with it. The live owner filters
presentation layers through Engine's workspace visibility before composition;
the same order bounds hit-testing and retained mixed layers. Relayout requests
contain only nodes assigned to the active workspace. The blind xmonad bridge
tracks its active workspace and mapped synthetic-window set, issuing explicit
unmap/map transitions so hidden windows cannot influence legacy layout policy.
CPU composition consumes the same ordered visible-surface projection, including
the empty-workspace case. A Present targeting a surface outside that projection
settles as a skip before any native submission; it cannot append itself back
into the mixed frame. These are protocol-neutral state projections; no terminal
or application identity enters Engine or rendering.

The first physical workspace run then proved that filtering composition is not
enough by itself. Super-2 committed action 258, cleared the focused client's
keys and focus, and stopped accepting its Presents, but submitted no replacement
KMS frame. The previously scanned-out workspace therefore remained visible
while keyboard routing was correctly disabled. The CPU-cycle preservation rule
had examined every committed DMA-BUF surface rather than the visible projection
and suppressed the empty-workspace repaint.

GPU preservation now reduces only the ordered visible transaction set. A
visibility-order change cannot be discarded by ordinary CPU coalescing. An
empty projection queues a black CPU frame; a returning projection queues its
retained mixed DMA-BUF frame when available, and otherwise paints the bounded
CPU background until the client supplies new pixels. Thus a workspace commit
always has a concrete scanout consequence instead of leaving the old workspace
on screen.

The follow-up physical cycle exercised workspace 1 seven times, workspace 2
nine times, and workspace 3 twice. Empty projections submitted the stable
blank CPU frame; populated projections restored their retained mixed frame and
focus. The run recorded no layout timeout or resize abort, zero native submit
and retirement failures, a clean native drain, and bounded completion after
122 submissions and 120 asynchronous retirements. The operator confirmed all
three workspaces visually.

Future captures no longer rely on that visual statement alone. Every committed
WM policy state now emits a reduced projection record containing only
transaction, output, workspace, visible-surface count, and whether focus is
present. The strict verifier requires workspace 2 and 3 to commit empty,
requires workspace 1 to return with visible surfaces and focus, and correlates
the focus-clear, suppressed-key, workspace-3, return, and focus-restore order.

The first capture with projection schema 2 populated Kitty independently on
workspaces 1, 2, and 3, committed 25 layouts without a timeout, and closed one
workspace-2 client without disturbing the others. It completed with zero
native submit, retirement, callback, control, or protocol failure. The control
ledger drained 22 enqueued and delivered commands with 17 ms maximum queue
dwell and 14 ms maximum acknowledgement latency; the input ledger drained
2,258 events with no pending key state.

This was a valid workspace and control-ledger proof, but not the complete
standard workflow. The startup Kitty remained open, and the capture contained
no click/drag button route, focus-next action, next-layout action,
hidden-workspace key suppression, or VT lifecycle. The strict verifier
correctly stopped at the missing startup exit instead of treating a clean
logout as evidence for steps that were not performed.

<!-- END IMPORTED BODY -->
