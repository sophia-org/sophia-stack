---
id: legacy-active-0451
date: 2026-08-16
recorded_date: 2026-08-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-08-16: merging must be bounded in transactions, not batches

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13558–13578. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Attempt `0023` on `6bdb42b4` failed at `KmsSubmit` with `session runtime
  observation batch exceeds 64 events`. Merging worked, and then overran a
  bound downstream of it.
- Cause: every committed transaction contributes one
  `AuthorityTransactionObserved` to the tick's runtime observation batch, and
  `MAX_SESSION_RUNTIME_OBSERVATION_BATCH` is 64. The merge run was bounded at
  64 *batches*, which is the wrong unit twice over — a batch may carry several
  transactions, and the fixed per-tick observations share the same budget.
- Fix: the run is now bounded by committed transactions, at the runtime
  maximum less a reserve for the fixed per-tick observations, and the constant
  is derived from `MAX_SESSION_RUNTIME_OBSERVATION_BATCH` rather than repeated
  so the two cannot drift. The head batch is exempt: it commits regardless of
  size, exactly as it did before merging existed, so the change can never make
  a previously working single-batch cycle fail. The batch cap remains as a
  second bound on owner-turn duration.
- Retained as a regression: a run assembled from many queued batches must stay
  under the runtime maximum, and one oversized batch must end the run rather
  than drag it past the budget.

<!-- END IMPORTED BODY -->
