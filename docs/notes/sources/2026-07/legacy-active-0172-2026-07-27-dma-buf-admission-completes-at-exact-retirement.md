---
id: legacy-active-0172
date: 2026-07-27
recorded_date: 2026-07-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-27: DMA-BUF Admission Completes at Exact Retirement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5915–5940. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The follow-up physical trace removed the earlier mixed-transaction rejection
but exposed the remaining visual-lifecycle defect: vkcube surface 6291456 was
admitted, laid out, focused, and framed without any retired Present for that
surface. All 51 retired Presents belonged to the existing Kitty surface. The
admission commit had treated a released DMA-BUF transaction with no causally
paired Present as a synchronous visual commit, while the X mapped snapshot
could independently disable quarantine.

DMA-BUF admission now enters `AwaitingRetirement` with the exact selected
visual transaction. It becomes managed and eligible for deferred focus only
when that surface and transaction retire from KMS. Admission quarantine no
longer consults the mutable X mapped bit. Both quarantine and production intake
require one-to-one surface/buffer pairing between DMA-BUF transactions and
Present submissions. Resource release is held while a quarantined group
references its DMA-BUF or fences, and backend intake registers and begins
presentation ownership before applying release. A Present-bearing GPU cycle
also cannot queue a retained CPU frame ahead of its candidate.

Offline Engine, CLI, and backend regressions cover exact retirement matching,
mapped-bit independence, malformed-group rejection, deferred resource release,
and deferred focus. The retained session log predates its source commit and is
diagnostic rather than proof; the new physical verifier requires matching
`armed`, `presented`, and native `retired` records plus clean teardown.

<!-- END IMPORTED BODY -->
