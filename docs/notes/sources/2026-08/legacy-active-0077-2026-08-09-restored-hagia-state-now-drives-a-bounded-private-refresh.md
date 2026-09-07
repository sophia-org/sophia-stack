---
id: legacy-active-0077
date: 2026-08-09
recorded_date: 2026-08-09
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-09: Restored Hagia state now drives a bounded private refresh

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2647–2661. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Hagia now exercises the retained policy-to-session refresh path instead of
merely decoding it. After a private checkpoint is restored, reconciled against
a complete snapshot, and committed once, the client advances the last admitted
private generation and emits one geometry-free `PolicyDirty` scoped to the
complete live output set. Ordinary actions do not create redundant refreshes,
and a pending session operation settles before the refresh is sent.

The independent socket test observes projection transaction 1, dirty
transaction 2, and the generation-2 projection at transaction 3. Hagia's full
verification task and Sophia's Rust/C/Nim/X11 client matrix pass. The installed
smoke and two-output physical gates now require the post-restart refresh
diagnostic; they remain evidence definitions until an authorized live run.

<!-- END IMPORTED BODY -->
