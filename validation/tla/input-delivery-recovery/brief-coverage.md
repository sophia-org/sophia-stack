# Coverage audit

All four scenarios run in `InputDeliveryRecovery.cfg`; scenarios 1–3 share the
same state space because their interesting case is their interleaving.
`InputDeliveryRecoveryNoDeadline.cfg` targets scenario 4's lost progress;
`InputDeliveryRecoveryEarlyBarrier.cfg` targets scenario 4's unsafe shortcut.
Both retain the positive model's safety checks. The positive config explicitly
enables every safety and liveness property listed in the brief.

Observed with pinned TLA+ Tools 1.7.4 (jar SHA-256
936a262061c914694dfd669a543be24573c45d5aa0ff20a8b96b23d01e050e88):
positive: 935,978 generated / 349,360 distinct states, depth 30, no errors.
NoDeadline: temporal counterexample, unfinished bound delivery remains pending
when time has saturated; this maps to the original owner queue's indefinite
barrier and is a deliberate implementation regression model.
EarlyBarrier: `BarrierSound` violation after admitting a release and removing
its barrier without any terminal receipt. This is the forbidden timeout-only
settlement design, not a defect in the submitted implementation.

No emitted implementation trace has been checked against this model. The source
mapping and deterministic code tests are separate evidence, not trace validation.
Runtime and physical acceptance are recorded separately from model checking.
