---
id: legacy-active-0139
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-05: CPU admission releases the replacement before its patches

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4454–4471. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first QEMU run past the repaired semantic preflight admitted a third xterm
with CPU handle 172, then failed composition because the committed scene lacked
that handle. X authority had correctly emitted one replacement followed by
several same-handle patches while the surface was quarantined. Admission
released only the selected final patch; the renderer rejected its missing base
and the following Engine commit exposed the absent handle.

Backing-snapshot admission now releases the complete ordered group prefix
through the selected transaction and rebases each generation at the accepted
geometry. PresentedBuffer admission retains its stricter behavior: passive CPU
history cannot overtake or impersonate the selected Present. A deterministic
regression reproduces replacement transaction 380 and selected patch
transaction 381, requiring replacement-before-patch order and generations zero
then one. The source audit and complete all-features suite pass; the unattended
QEMU gate remains the acceptance boundary.

<!-- END IMPORTED BODY -->
