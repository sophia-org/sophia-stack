---
id: legacy-active-0613
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "validation"]
---
# 2026-09-04: Firefox canary passes after correcting two reader assumptions

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19460–19503. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next physical run used signed candidate `2823807e2ecd`. Frontend drain
completed in 8 ms and normal quiescence in 43 ms, with zero pending authority,
coordinator, CPU, or native work. Session/layout health and application/frontend
cleanup were clean. The strict reader nevertheless rejected the capture twice:
it counted schema-2 launch evidence and its schema-1 compatibility echo as two
launches, then required a standing-target successor for CPU backing admission.

The launch producer deliberately emits both schemas. The verifier now consumes
one matching schema-1 echo after a schema-2 launch, while accepting either
schema alone for compatibility. Modern surface observations must match the
launch transaction. Extra launches, repeated echoes, unsupported schemas, and
mismatched transactions remain failures; unrelated tracing between paired
records does not alter their meaning.

CPU backing admission clears its recovery extent directly in `commit_pending`
and emits `sophia_live_visual_admission` with `source=cpu_backing_snapshot`.
That path need not create a standing-target Present successor. The verifier
requires the subsequent committed CPU admission for the same browser surface;
Present-based recovery still requires its successor, and every observed
successor must commit. Repeated or unknown recovery, stale/wrong-surface CPU
admission, missing pixel proof, and failed teardown remain rejected. The
existing changing-nonblack-region/head-scene/native-retirement joins are intact.

Both new positive cases failed before the corrections and pass afterward.
Regression cases cover paired and individual launch schemas, interleaved
tracing, the combined CPU/paired-launch path, malformed/extra launches, and
missing, wrong, or out-of-order admission evidence. The unmodified physical
capture now passes the corrected verifier; no new compositor build or physical
run was needed. It is retained at
`.artifacts/diagnostics/firefox-2823807e-20260904T2110/session.log`, SHA-256
`21183cdfce97d239d0528ba0ab53475820d0ff7f38b4d89b90ab2b4f2393c1e4`.

This closes the short Firefox canary prerequisite, not CP-14.2. Preparing a
fresh pinned comparison run is next; previous partial matrices remain
diagnostic, and the overnight soak remains optional.

Verification: the expanded rendering-verifier corpus and the full offline
`cargo xtask check` pass, including 2,332 tests, all 20 retained archives, and
host buffer-age pixel equivalence. The earlier `48bf357f` timeout capture still
fails. Only verification scripts and documentation changed; runtime binaries,
personal configuration, and installed release contents are unchanged.

<!-- END IMPORTED BODY -->
