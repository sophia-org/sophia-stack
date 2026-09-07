---
id: fltuldiq
date: 2026-09-06
kind: investigation
status: investigating
tags: [investigation, session, rendering]
---
# Runtime session crash retains no specific cause

## Question

Why did the installed session fail during ordinary use, before logout? The
operator does not remember the action immediately before the crash.

## Evidence

Session `00000001788746137544-5f2416f8-5881-4130-8fea-82813c882929` ran installed
release `4b4f28418829d03191d53e533d7903d07d433633`, executable SHA-256
`ef721e789aab1e06eee46ebde04dfcd65d818aa1a0953de83d2d1d665596358b`.
Its preserved snapshot is
`/home/niltempus/.local/state/sophia/session-investigations/00000001788746137544-5f2416f8-5881-4130-8fea-82813c882929-b16c7d96-4ee2-4f7e-a72a-56331ddf5f3c`.

The last scanout completed at boot millisecond 228740184. A resource sample
280 ms later reported RSS 481200 KiB, one CPU buffer, six leased frame slots,
seven snapshot entries, and fourteen import-cache entries. Event 18556, at
228742087, records an owner-loop runtime fatal error with `failure_code=unclassified`.
Native suspension drained 238 ms later. The fatal cleanup record reports
frontend intake stopped, native scanout drained, renderer images cleared, and
presentations shut down. Process teardown followed, then exit status 1 and
successful TTY recovery without emergency intervention.

Recorder health ended at sequence 18572 with zero discarded records and zero
storage errors. There is no preceding VT, seat, topology, or client-fatal event
in the retained tail. Protocol-tally rows retain counts 5, 3, and 1, but the
installed recorder removed their opcodes and error codes.

## Finding and limits

This is distinct from the [ordinary logout policy defect](64o6l37g-normal-logout-reports-failure-after-x-protocol-errors.md).
The runtime error precedes completion policy. Removing the normal-session
protocol-count failure cannot correct this crash.

The evidence identifies the owner loop but not its failing operation. Worker
stall detection, control acknowledgement failure, and runtime invariants remain
possible; elapsed time alone does not select one. Successful cleanup does not
prove that rendering or application execution was correct before the failure.
No specific root cause or reproduction has been established.

## Current correction and remaining work

The checkout adds approved phase records at the session boundary, preserving
runtime phase across cleanup and retaining safe causes before request-tally
context wraps errors. Existing runtime-fatal records remain the source for
original typed causes when cleanup has converted an error to text. Arbitrary
error messages and application data remain excluded. New records cannot
recover details missing from this installed archive.

Continue t019 in [the workflow plan](../plans/queue-06-4-exercise-real-development-workflows.md#t019):
install a candidate containing the diagnostic correction, retain any recurrence,
then fix and regress the identified failing boundary. This incident does not
reopen the accepted t015/t016 diagnostic workflow or demand another full matrix.
