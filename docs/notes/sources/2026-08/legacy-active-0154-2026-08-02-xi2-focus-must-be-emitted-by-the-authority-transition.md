---
id: legacy-active-0154
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "security"]
---
# 2026-08-02: XI2 focus must be emitted by the authority transition

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4924–4956. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The latest physical Firefox run reached resize stage 5 and then committed every
repeated `Super+J` action, xmonad layout response, Engine focus reconciliation,
and X control acknowledgement. It still ended at 6/8 because the page never
observed a DOM blur/refocus pair. This disproves the WM and Engine paths as the
remaining cause.

The cross-client broker fix delivered core `FocusOut` to the old client and
core `FocusIn` to the new client, but XI2 focus remained synthesized lazily by
the input writer on a later key packet. A compositor-owned Super chord is not
delivered to Firefox, so no packet existed to trigger that synthesis. The one
earlier physical refocus success was therefore nondeterministic later-input
behavior, not a locked focus transition.

XLibre confirms that `SetInputFocus` calls `DoFocusEvents`, which emits core and
device focus events together. Yserver independently reduces each focus crossing
into mask-filtered core plus XI2 events at mutation time, including the
ancestor-derived detail and current pointer coordinates. Sophia now follows
that boundary for Engine-originated surface focus: the passive broker packet
carries one monotonic timestamp, each client writer snapshots protocol-local
pointer/modifier/button state, builds selected core then XI2 records before
taking the socket lock, and writes them without waiting for input. Keyboard
delivery no longer owns focus synthesis; its public transition mask is narrowed
to pointer crossings.

The two-client wire regression selects and deselects XI2 focus while exercising
repeated A-to-B-to-A transitions with no key injection. The Firefox page and
QEMU harness no longer contain the diagnostic `r` bypass, and the physical
verifier requires ordered focus-away, selected XI2 out/in, focus return, and
only then the DOM checkpoint. A fresh physical workflow remains the acceptance
boundary.

<!-- END IMPORTED BODY -->
