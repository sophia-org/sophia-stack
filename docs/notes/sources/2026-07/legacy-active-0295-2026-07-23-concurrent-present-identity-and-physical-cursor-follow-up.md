---
id: legacy-active-0295
date: 2026-07-23
recorded_date: 2026-07-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-07-23: Concurrent Present Identity and Physical Cursor Follow-up

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9167–9200. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next guarded TTY3 run proved that Super-Enter launched a second terminal,
then the persistent frontend exited with `X11 route queue is full for client
9`. Each X client worker had derived the Engine transaction identity from its
connection-local 16-bit X sequence. Two clients could therefore submit the
same global `TransactionId`; the pending Present registry treated that
collision as queue pressure, and worker completion propagated the client error
to the whole frontend supervisor.

The listener now allocates monotonically increasing 64-bit transaction
identities shared by all of its workers while preserving the X wire sequence
as a client-local `u16`. Duplicate transaction identity and real per-client
pressure are distinct failures. A full Present queue waits for bounded
Complete/Idle progress for up to two seconds, and removal wakes waiters; no
feedback is dropped or reordered. A client-local Present failure disconnects
only that worker, while panics, poisoned shared state, and supervisor failures
remain service-fatal.

The same capture exposed the private legacy-WM server's 1280x720 setup fixture
on physical 2560x1440 and 1920x1080 outputs. Production bridge startup is now
lazy until the first Engine layout request, advertises those actual bounds in
X setup, and emits root ConfigureNotify on later changes. The compatibility
fixture remains available only to standalone smoke callers.

Cursor ownership remains protocol-neutral. A bounded passive snapshot models
visibility, global position, hotspot, ARGB image, and generation without an X
or application identity. The current software path now coalesces motion,
avoids repaint submission while native scanout is in flight, polls at 1 ms
while motion is pending, and reports maximum motion-to-submit latency.
Client-selected classic X cursor shapes and optional KMS cursor-plane
presentation remain separate follow-ups; the Engine contains no Kitty-specific
branch.

<!-- END IMPORTED BODY -->
