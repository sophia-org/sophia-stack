---
id: vwo9wmie
date: 2026-09-07
kind: investigation
status: awaiting-physical-acceptance
tags: [investigation, rendering, x11]
---
# Window switches reveal delayed visual updates

## Question

Brave accepted navigation but displayed the result only after a window switch.
The user also reported intermittent clicks and stale UI in Okular and Thunar;
Kitty remained responsive. Does missing visual progress explain an apparent
input failure? These observations do not establish one cause across clients.
This is a rendering follow-up to [the t066 input investigation](37xvg0y7-qt-menus-fail-to-open-while-pointer-leases-are-rejected.md).

## Evidence and limits

The observed session ran `c1e827e0a89085e081c8fcdddddc7a20d2d9e4a2`, installed
as `0.1.0-c1e827e0a890`, with executable SHA-256
`c090ac03e708d1ec91f03812a4bdaefa36409c9b5b26ce7b408b882d156d67e4`.
Its record is `~/.local/state/sophia/sessions/00000001788819836482-32a69abb-6c1f-4f92-babd-7ec11a92d629`.
Brave surface 25165827 had discrete retirement gaps while other surfaces kept
retiring frames. This establishes neither missing client content nor a failed
input route. A gap without keyboard records cannot be classified as idle:
pointer-only interaction remains possible. Rotated logs must be read together.

Brave's stdout and stderr were connected to `/dev/null`. There was no retained
browser error log to inspect. The absence of a session protocol refusal or
fatal error does not establish that Brave emitted no errors.

The implementation candidate is the working tree based on that commit, including
the earlier wheel repair. The retained source patch, new source files, checks
and their hashes are in
`~/.local/state/sophia/development-evidence/t066-visual-progress-20260907`.
This is a deterministic development candidate, not an installed acceptance run.

## Findings and implementation

### Immediate NotifyMSC could precede its request's sequence publication

A real socket probe paused the request observer on request 5. The old frontend
already delivered its immediate NotifyMSC event, stamped sequence 4, in both
byte orders. This is a reproduced frontend ordering defect. It can matter to
clients that correlate an event with their request cookie; it is not proof of
what the installed Brave process was waiting for.

Immediate requester events now travel with that request's normal output and
carry its exact sequence. Later replies cannot overtake them. Other subscribers
still receive asynchronous events using their own connection sequence. Future
NotifyMSC scheduling remains separate: the existing clock is advanced by
Present activity, and this repair does not establish independent future-target
wakeup or divisor/remainder conformance.

### A backend-deferred CPU compose could lose its repaint obligation

A visible DMA-BUF makes the native output preserve GPU ownership. An otherwise
admitted CPU-only content update can therefore be retained in the scene without
composition. After a quiet cadence interval, the pacer had not marked a repaint
pending, and the session reported only successful composition to it. A compiled
state probe reproduced requested defer false, actual defer true, and no pending
repaint. This establishes the scheduling gap, not physical mixed-scene pixels
or attribution to a particular application.

The pacer now records both production outcomes. A refused CPU composition keeps
a cadence repaint obligation. Ordinary repaint uses the same guard as retained
publication: suspension, an in-flight Present or an unsettled software binding
postpones it. Postponement retains the request and advances the retry deadline,
avoiding a busy loop. Forced startup and topology repaints keep their existing
path. DMA-BUF Present traffic alone still cannot arm synthetic CPU repaint.

### Feedback could miss the owner turn's early exit

Lifecycle service can retire the first native frame before the service timer
is armed. If no work remains, an authority-only batch can skip the later feedback
drain without first arming that timer. The session now drains authorized feedback
after lifecycle service, before that exit. Shutdown and the ordinary tail drain
use the same consuming helper, preserving retirement ordering and avoiding
repeat delivery.

### Progress observations separate the stages

`SOPHIA_LIVE_VISUAL_PROGRESS=1` enables reduced offered-content, committed-surface
and native-head snapshots plus feedback detail. It does not alter rendering
policy. Snapshots may coalesce intermediate states; their counters and baseline
flag describe that limit. Feedback queued to a connection is not proof of
client receipt. The [operations contract](../../operations.md) defines the fields
and the existing recorder's loss accounting. No pixels, titles or checksums are
added to these records.

## Validation and remaining acceptance

The public socket regressions cover observer blocking, pipelined request/reply
ordering, sequence wrap, invalid-window rejection, and another subscriber's
sequence in both byte orders. Against the old library, four tests failed and
the rejection test passed; all five pass on the candidate.

Pacer regressions cover refusal after admission, deferred retry without spinning,
and settlement by later composition. Runtime tests cover all three publication
guard clauses, including exact first-Present retirement. The feedback regression
uses the real frontend router and checks Idle/Complete ordering with a disarmed
service deadline and no-engine-work batches. Telemetry tests cover snapshots,
coalescing and recorder redaction. Mutation checks demonstrate that the relevant
arming, deadline, guard and feedback assertions fail when their protections are
removed.

Final combined validation: `cargo xtask check` exited 0, including the source
layout ledger and install/archive fixtures; `cargo fmt --check` and
`git diff --check` passed. The real Qt receiver also passed again against the
combined frontend: four wheel events, one press/release, zero duplicate core
button events. The all-feature visual-progress test filter executed all four
new telemetry cases (plus fourteen existing CPU-progress cases). Task IDs and
open-task links were checked. `zk` still reports two existing links to excluded
`validation/tla` files as unresolved; both targets exist on disk, and the new
note's links resolve.

The native call sites remain source-reviewed: these tests do not acquire DRM/KMS
or prove final pixels in the installed mixed scene. Shared-helper tests do not
execute the complete native owner loop. The next acceptance is ordinary use in
a newly installed candidate: Brave navigation without switching windows to
reveal it, Okular clicks and wheel, and Thunar menu redraws. Record the candidate
identity and an incident marker if a stall remains. Preserve browser stderr on a
diagnostic launch; do not infer browser health from session logs alone.

`t066` remains open in [todo.md](../../../todo.md). Its physical gate is distinct
from the deterministic repair. No live session was replaced during this work.

## Connections

[Architecture](../../architecture.md) owns cadence and retirement guarantees;
[the X frontend contract](../../sophia-x-authority.md) owns NotifyMSC sequencing.
[The earlier blank-menu investigation](ce2b55uy-blank-thunar-menus-and-frozen-brave-need-separate-pixel-and-delivery-evidence.md)
explains why input delivery and pixel publication need separate evidence.
