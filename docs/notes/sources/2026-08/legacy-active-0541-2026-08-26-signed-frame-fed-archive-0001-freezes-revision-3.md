---
id: legacy-active-0541
date: 2026-08-26
recorded_date: 2026-08-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-26: signed frame-fed archive 0001 freezes revision 3

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16699–16730. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed Sophia source `870ba46ae231081220b982ecc3a5a95517df7a90` passed the
complete two-phase gate with signed Hagia source
`a83c8fa022a4ceff5d8b96a01c46052bbd8ba64a`. The success phase applied both
heads, installed the candidate, first-presented it, published the frontend and
committed snapshot, accepted physical `outputapply` input, and tore down cleanly.
The rollback phase crossed the exact boundary after final KMS acceptance,
restored in reverse card order before installation or publication, accepted
physical `outputrollback` input, and also tore down cleanly.

The real stream corrected one verifier assumption. A committed snapshot is an
unsolicited transport publication with its own positive transport transaction;
it is not the private startup authority transaction. The verifier now binds that
publication to the committed local settlement by exact topology epoch, while
the rollback phase forbids any committed snapshot publication regardless of
transport transaction. Synthetic epoch-mismatch and rollback-publication
mutations prove both checks fire.

Permanent frame-fed archive `0001` contains success evidence
`7dbcc54326d48168df930edf88d81f5cf64fb64251f3b2a9b150e159a37431e5`,
rollback evidence
`267f8b11cc3de692708ee4c634efe6a09b6eb31da992483566e3ba520114f69d`, pair
hash `7311dfe675e7ef8ca7aa9ec22ef3d1aeecc0fa8e05f8a33976c106f3cc615f49`,
and archive-manifest hash
`2e67d12ea453b55dc436e5d4a6bb1c1f9f842f0a6f21920462028e5dcdab5457`.
Independent verification reports `status=passed boundary=after_apply phases=2`.
The Hagia ledger therefore closes at 21 Complete, 0 Partial, 0 Open, and 7
Excluded, and interface major 1, wire revision 3 becomes stable. API v7 removal
is now the next critical-path change; excluded post-freeze product work does not
reopen the wire.

<!-- END IMPORTED BODY -->
