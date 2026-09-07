---
id: legacy-active-0122
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "validation", "tooling"]
---
# 2026-08-06: Installed login proof accepts the production trace envelope

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4026–4040. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first normal installed run on `02505e81` retired nine asynchronous kernel
page flips, reported 16 ms maximum submit-to-flip latency, and drained cleanly.
Its recorder nevertheless marked the attempt failed because the focused login
verifier required the page-flip schema at byte zero. Production emits that
record through `tracing`, after its timestamp, level, target, and ANSI state.

The login verifier now uses the same whitespace-delimited structured-payload
boundary as the fallback verifier. It still requires a genuine retirement and
rejects a log with that payload removed. A fixture wraps the passing record in
the production trace envelope so formatting metadata cannot invalidate later
installed evidence. Archive `0002` remains an immutable failed attempt; a new
run must supersede it.

<!-- END IMPORTED BODY -->
