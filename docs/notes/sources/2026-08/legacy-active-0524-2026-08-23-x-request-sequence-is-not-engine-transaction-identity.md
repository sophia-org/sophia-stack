---
id: legacy-active-0524
date: 2026-08-23
recorded_date: 2026-08-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "architecture"]
---
# 2026-08-23: X request sequence is not Engine transaction identity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16016–16038. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The corrected route run reached the browser's first CPU-backed frame, selected
it as admission evidence, and then ended with a pre-admission group failure.
The validator's generic owner-loop message hid the exact mismatch. The batch
carried the frontend's global transaction, while core drawing still derived its
surface transaction from the causing connection's 16-bit X request sequence.
Those values happened to agree for the first client. Helium opened later, so
its sequence restarted after the listener's transaction counter had advanced.

Dispatch now carries both identities explicitly. The global `TransactionId`
labels every Engine-visible response and surface transaction; the local X
sequence labels only X11 replies, events, and errors. Core drawing, window,
property, resource, text, and MIT-SHM paths no longer synthesize transactions
from the X sequence. Admission remains strict and now returns its exact
validation error to the owner loop.

A direct drawing regression separates the two values, and the classic-shared
two-client regression advances the listener with client 1 before client 2
draws at local sequence 3. Its batch and surface transaction both retain global
transaction 4. The X authority suite and exact malformed-admission regression
pass locally. The signed installed switcher rerun remains the promotion gate.

<!-- END IMPORTED BODY -->
