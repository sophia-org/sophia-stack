---
id: legacy-active-0521
date: 2026-08-23
recorded_date: 2026-08-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "shell"]
---
# 2026-08-23: explicit X grabs join application and shell arbitration

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15921–15955. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Core `GrabPointer` and master-pointer `XIGrabDevice` now enter Engine through a
bounded passive control path. The X worker resolves the request to the exact
admitted surface, or to a deterministic presented surface for the root window,
then asks Engine to prepare an explicit application lease before changing
frontend grab state. A provisional explicit lease routes no physical input.
Only a successful frontend mutation may activate it and expose X grab success;
failure aborts the candidate. Same-client re-grab replaces the exact active
lease, while another owner receives `AlreadyGrabbed` without mutation.

Explicit ungrab performs the inverse ordered handshake. Engine first marks the
exact lease releasing, the frontend clears its X owner, and only then does the
owner retire the lease. Scope exit, client removal, topology change, and
VT/seat security transitions continue through the existing release and epoch
paths. Queue saturation, a missing owner, timeout, stale identities, wrong
admission, and old control epochs fail closed. XI2 event masks remain in the X
frontend: Engine arbitrates ownership but does not acquire X protocol policy.

The ordinary compiled Hagia profile now enables the shell and binds `Super+P`
to `session:window-switcher`. The one-shot physical gate builds and hashes the
exact `hagia-shell` binary and carries a proof-only restart after the second
visible switcher presentation. Its verifier requires three output-local
nonzero presentations, two broker issuer validations and withdrawals, a fresh
recipient epoch, one click against retained inert pixels, and clean broker,
shell, session, topology, namespace, and frontend teardown. Synthetic verifier
fixtures and Rust functional suites pass. Hagia's functional verifier also
passes and still stops at the unchanged host dependency mismatch: Z3 4.16.0 is
required, while the host provides 5.1.0.

This completes the arbitration and compiled-enablement prerequisites. It does
not promote the shell row: the new signed installed archive has not run.
Lock-authority epoch integration, reservations, previews, icons, MRU policy,
and generic texture transfer remain separate work.

<!-- END IMPORTED BODY -->
