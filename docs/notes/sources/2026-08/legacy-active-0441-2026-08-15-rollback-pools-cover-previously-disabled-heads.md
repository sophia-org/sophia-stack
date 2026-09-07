---
id: legacy-active-0441
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-15: rollback pools cover previously disabled heads

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13251–13265. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Published rollback projection now represents connected-but-disabled heads as
  disabled members. Their rollback side owns prepared detach properties instead
  of requiring an impossible framebuffer. Candidate enable, candidate disable,
  rollback enable, and rollback disable are all checked against the head's prior
  state before the cohort can become ready.
- The live head record now distinguishes connected-but-disabled heads from active
  logical-output members, while `outputs()` remains a separate logical-output
  table. Startup behavior is unchanged because every discovered startup head
  begins enabled. The production WM path still cancels before KMS: the next slice
  must retain rollback owners across physical apply, output-runtime
  reconstruction, and the first-presentation barrier rather than adding a
  premature finalize operation.

<!-- END IMPORTED BODY -->
