---
id: legacy-active-0164
date: 2026-07-30
recorded_date: 2026-07-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering", "validation"]
---
# 2026-07-30: GLX Animation Passed; Startup Evidence Lost a Valid Early Frame

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5430–5517. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The bounded six-second physical rerun visibly animated. Radeonsi reported
52.956 client FPS on the RX 7900 GRE. Sophia completed 242 renderer requests
with 242 completions, no worker failure, soft stall, hard stall, or
release-queue failure; native teardown drained without an abandoned scanout,
and X protocol errors remained zero. Neither the 500 ms page-flip watchdog nor
the independent 11-second session deadline fired.

The run still exited status 1 because the completion gate claimed startup
readiness was never reached. Transaction 46 had in fact produced 70,988
nonzero RGB pixels, retired through KMS, committed visual admission, and was
logged as a stable mixed scanout. That happened immediately before the X focus
control acknowledgement pinned the startup surface. The startup reducer
correctly rejected pre-pin evidence to prevent another client from satisfying
the gate, but the owner retained only the latest transaction per surface.
Continuous animation made every later retirement overlap a newer pending
frame, so the already-proved stable frame could not be reconstructed.

Startup presentation evidence is now retained monotonically in a
surface-keyed map until readiness or native recovery. It records only a frame
that was stable at its actual KMS retirement and preserves the maximum observed
nonzero count. Once focus pins the startup surface, only evidence with that
exact surface identity can supply visual-detail and stable-presentation
events. A status bar or another client therefore cannot satisfy the startup
application, while asynchronous focus and presentation ordering no longer
creates a false failure.

The next physical rerun showed that the map alone was insufficient. It
retained transaction 46 correctly, but the consumer redundantly required a
current base committed-surface record. Removing the earlier `BufferSource`
subtype check had left this surrounding membership gate in place. DRI3 Present
content is a presentation-layer lease; after admission, the base committed
surface may legitimately be absent. A third bounded run still animated at
53.542 client FPS, completed 242/242 worker requests with no stalls or failures,
and drained all imports, while this gate alone produced the false startup
failure. The stable nonzero KMS retirement already proves both GPU content and
surface identity, so startup visual-detail reduction now uses that evidence
even when no base committed record exists. A regression covers that exact
state instead of approximating it with a DMA-BUF-backed base surface.

The following run again animated normally at 52.981 client FPS and drained
241/241 renderer requests, but exposed the underlying control-flow split.
Transaction 46 retired while the owner was waiting for the next authority
batch. That authority-wait path logged the same stable scanout as the ordinary
lifecycle path but did not update startup presentation evidence. The two
nearly identical retirement blocks had drifted semantically. Retirement is now
recorded by one shared function used by both service sites; it owns admission
retirement, surface-keyed startup evidence, reducer input, and the structured
retirement/scanout records. Phase-local input-proof bookkeeping remains at the
call sites.

The corrected physical run reached GPU content readiness and full startup
readiness in 178 ms without native recovery. It visibly animated at 59.088
client FPS, accepted 352/352 renderer requests, completed 351 Present Flips and
Idle notifications, reported zero protocol errors, drained every imported
image, and exited status 0 with clean session and TTY recovery.

That successful run exposed a final benchmark-only mismatch: verbose tracing
was intentionally disabled, but the cadence reporter still parsed per-frame
Present diagnostic lines. Presentation cadence is now accumulated in bounded
owner state from routed retained-buffer UST values and emitted once at
completion. The report keeps exact sample/interval counts, nonadvancing and
overflow flags, mean FPS, and p95 frame time without adding per-frame logging
overhead. Reporter regressions reject insufficient, nonadvancing, or overflowed
summaries.

The final six-second run validated the aggregate path end to end. Startup
reached readiness in 161 ms without recovery. The client reported 59.197 FPS;
352 routed retained-buffer samples produced 351 advancing UST intervals, zero
nonadvancing observations, 59.953 presentation FPS, and 17.324 ms p95 frame
time. Sophia completed 353/353 renderer requests, 352 exact Flip/Idle pairs,
zero protocol errors, zero live imports, and clean exit/TTY recovery. The
schema-3 performance reporter returned `status=pass`.

Teardown also exposed stale worker metrics: `ClearImages` was queued without an
acknowledgement, then the owner immediately sampled the previous cache state.
The maintenance command now returns its eviction result and updated persistent
statistics over a one-slot bounded channel with a one-second deadline.
Changing GLX buffers legitimately produced 242 imports and zero cache hits:
Idle releases each generation before the client may mutate and reuse it. The
report therefore requires positive imports and clean zero-debt teardown, while
retaining cache hits as an informative counter rather than inventing reuse.
The benchmark no longer forces verbose per-stage tracing. Normal structured
presentation and resource summaries remain enabled, while diagnostic tracing
can be opted into explicitly, so the performance probe does not measure its
own high-volume logging.

<!-- END IMPORTED BODY -->
