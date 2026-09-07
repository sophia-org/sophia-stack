---
id: legacy-active-0580
date: 2026-08-31
recorded_date: 2026-08-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-31: quiescent frontend disconnect is a drain signal on every receive path

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18234–18269. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The terminal gate on signed commit
`de032461b5b648191af7a9dc58de64c3f309816c` completed the bounded xterm
workload and drained X Authority promptly. It then failed because
`drain_queued_authority_batches` returned a fatal error when its opportunistic
`try_recv` observed the frontend sender disconnect. The blocking
`recv_timeout` branch already classified that same state correctly during
session quiescence. Which receive path discovered closure therefore changed
shutdown semantics. The subsequent fatal cleanup sent `StopAccepting` again
after normal quiescence had stopped intake, producing a second false cleanup
error against the closed command channel.

Channel draining now returns a typed open-or-disconnected observation and one
owner transition classifies it for blocking and opportunistic callers.
Disconnect remains fatal before quiescence. During quiescence it marks frontend
authority drained exactly once, while already buffered batches remain subject
to the ordinary empty-queue, CPU-settlement, and native-idle completion
predicate. Frontend admission stopping is idempotent: a real first-send failure
is retained, but cleanup after an earlier successful stop performs no second
send. Deterministic regressions preserve the last queued batch before
disconnect, require it to settle before completion, retain pre-quiescence
failure, and cover both stop outcomes.

The retained run showed no long compositor pause: CPU source progress had a
16.403 ms maximum gap and changed primary retirement had a 33.097 ms maximum
gap. The visible burstiness instead matched the probe emitting eight terminal
rows per 16 ms iteration. The physical visual gate now defaults to one row
every 16 ms so motion maps directly to display cadence, while
`SOPHIA_XTERM_LINES=8` remains an explicit stress override. CPU visual
progress schema 2 adds exact microsecond gaps. Terminal performance schema 5
requires source gaps within the greater of three producer intervals or two
refresh periods plus one millisecond, and display gaps within two refresh
periods plus one millisecond. The one-second first/last liveness bounds remain
separate.

<!-- END IMPORTED BODY -->
