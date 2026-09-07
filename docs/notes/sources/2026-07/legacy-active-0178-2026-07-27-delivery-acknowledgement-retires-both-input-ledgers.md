---
id: legacy-active-0178
date: 2026-07-27
recorded_date: 2026-07-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-27: Delivery Acknowledgement Retires Both Input Ledgers

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6117–6134. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The second emergency capture proved that both synthetic modifier releases
reached the frontend: all 547 expected input deliveries flushed, the pressed
key ledger reached zero, repeat was inactive, and Engine/native teardown was
clean. Completion still failed because the two release IDs remained in a
separate control-ordering barrier. Normal loops eventually pruned that set
during another control-service pass; emergency completion exits immediately
after the input-delivery reducer settles and exposed the split ownership.

Input-delivery acknowledgement now atomically retires an ID from both the
general pending set and the client-key release barrier. A focused reducer test
locks that invariant. The pre-emergency physical gates on the parent candidate
remain valid for this isolated bookkeeping correction. After the new commit
runs its unattended semantic gate, a closed-path adoption command reverifies
the parent native, hardware-smoke, and xmobar evidence and records provenance.
It cannot adopt emergency evidence or accept broader runtime changes.

<!-- END IMPORTED BODY -->
