---
id: legacy-active-0612
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-09-04: frontend EOF must not strand accepted owner work

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19412–19459. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The physical Firefox canary on signed candidate `48bf357f2dd2` rendered visibly.
The operator confirmed that the apparent flashing was the page's deliberate
light/dark alternation, not a report of the window disappearing. This is a
visual rendering result, not a full gate pass: logout timed out after 2,000 ms
with `authority_pending=1`, `cpu_pending=0`, and `native_pending=false`.
Frontend egress had already drained in 18 ms without producer cancellation.

The owner selected an idle service turn whenever frontend EOF was observed,
before checking its initial or buffered batches. Opportunistic draining could
receive the final removal batch and then EOF in the same turn; the next turn
would never consume that batch. The shutdown state correctly refused success,
but the work selector made progress impossible. Existing tests checked queue
retention and the completion predicate independently, missing their connection.

The production selector now keeps topology/native service priority, then takes
initial, FIFO-buffered, and coordinator work before deciding whether to receive.
EOF disables only receives. No accepted batch is cleared or reclassified as
completed, and merge restrictions still preserve removal/resource-release
boundaries. Confirmed EOF also suppresses opportunistic receiver probes; idle
service yields within the frame and existing shutdown deadlines. Quiescence
now counts pending layouts, issued WM requests, unapplied WM commits, and
released surface content as coordinator work. Timeout diagnostics add bounded
initial/queued/coordinator counts and the oldest pending authority transaction.

The extracted production selector failed the final-removal regression with
the original EOF-first ordering and passed after correction. Added tests cover
FIFO/capacity boundaries, service preemption, initial and coordinator work,
resource-release preservation, closed-receiver suppression, and independent
authority/coordinator/CPU/native completion barriers. Unexpected runtime EOF
remains fatal, and graceful disconnect remains distinct from cancellation.
The canary now explicitly describes its intentional light/dark animation.

Physical acceptance still requires one newly signed canary with pixel proof
and clean normal teardown before a fresh CP-14.2 comparison run. Previous
partial comparison evidence remains diagnostic; the overnight soak is optional.

Verification: all 11 focused shutdown regressions pass, including the
failing-before/passing-after EOF selector test. The offline `cargo xtask check`
passed 2,332 tests, Clippy, source-layout and profile checks, all 20 retained
native/mirror/direct-scanout archives, the verifier corpus, and host buffer-age
pixel equivalence. Unix-socket tests required execution outside the sandbox;
the initial sandbox run was denied local socket connections, not a product
failure. The original physical log still fails the strict rendering verifier
and is retained at `.artifacts/diagnostics/firefox-48bf357f-20260904T2054/session.log`
with SHA-256 `bdedf4b0722d6d9a35e7350d30b5ff8469b38d647356de7f5dc5c6bde6a7be6d`.

<!-- END IMPORTED BODY -->
