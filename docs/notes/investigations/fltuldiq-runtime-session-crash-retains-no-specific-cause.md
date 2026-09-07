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

## 2026-09-07: signer paste narrows a recurrence to session control

The user ran `echo "cache" | gpg --clearsign > /dev/null`, opened the configured
`pinentry-egui` 0.1.1 signer, and attempted Ctrl+V or Ctrl+Shift+V after copying
a passphrase from Firefox's Bitwarden extension. The exact shortcut is uncertain.
The investigation did not read clipboard contents, passphrases, or key material,
and did not reset or restart the GPG agent. Signing later succeeded; commit
`bed135ea` and the two preceding commits were pushed to origin/master.

The failed live session still ran `e8573cf1`, without the
[click-grab repair](744uylx4-explicit-pointer-grabs-must-replace-their-own-click-lease.md).
Its ID is `00000001788753082918-0a84405d-9e24-433d-950f-d3dce25a2607`; the binary
SHA-256 is `c3bb971a6925a9366480bb25eb6155d5fa1ebf8e177705d08f47d7f1787fa74f`.
The preserved record is
`/home/niltempus/.local/state/sophia/session-investigations/00000001788753082918-0a84405d-9e24-433d-950f-d3dce25a2607-85e4e9dc-77dd-4c24-82c5-85e9258ff9c3`.

At boot millisecond 262156818 a pointer batch was observed but not immediately
routed. A focus-handoff marker and geometry controls followed. At 262156927,
a later pointer batch was suppressed by policy. At 262157320, 502 ms after the
first batch, the owner loop failed with `failure_code=unclassified`. The session
boundary retained `phase=control`. Native scanout drained, cleanup completed,
and the session returned to the display manager with status 1. TTY and keyboard
recovery succeeded. Recorder health reports no discarded records or storage
errors. This establishes a session failure; it does not establish a kernel crash.

The timing matches the 500 ms session-control queue/acknowledgement deadlines,
but the record cannot distinguish timeout from other failures in control
service or subsequent layout progress. `service_session_controls` converts its
typed errors to strings; the recorder does not retain the original text or a
specific control outcome. A missed acknowledgement is a hypothesis, not the
confirmed root cause. The earlier grab repair has no installed acceptance for
this signer trigger and must not be called its crash fix.

Namespace and portal rules do not explain a session exit by themselves.
`XAuthorityRuntime::apply` routes `ClipboardSelectionDispatch::SameNamespace`
directly and creates a pending portal transfer only for `CrossNamespace`.
The user's configured application catalogue uses `trusted-host`; no retained
portal-denial record identifies this incident. A refused transfer should fail
the paste, while session control and recovery must retain their own invariants.

A bounded headless probe at `/tmp/sophia-pinentry-paste` ran a direct signer UI
with a dummy prompt, a private X endpoint, and hard-coded nonsecret clipboard
text. It did not invoke GPG or the user's agent. Initial attempts needed private
file permissions and Hagia's required application-catalogue slot. Once admitted,
the session exited cleanly, but the helper's synthetic core key `SendEvent` was
refused with `BadValue`; no clipboard text was served. It therefore did not
exercise either paste shortcut and is not clipboard or crash acceptance. The
next reproduction needs admitted input routing with dummy text, plus a retained
typed control failure identifying the missing or rejected acknowledgement.
