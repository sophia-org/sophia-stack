---
id: legacy-active-0604
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-09-04: comparison qualification must be admitted before timed capture

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19127–19173. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first Sophia row of owner-only run `cp14-schema4-07effa0a`, bound to signed
candidate `07effa0a`, completed its physical 60-second workload but remained a
partial diagnostic. It retained 60 resource samples, an empty visibility
baseline, one focused and visible DP-1 workload at settlement and all 60
samples, one complete workload record, and 3,600 contiguous single-delivery
kernel frames. It did not create `measurement.kdl`, and the live-session cursor
qualification file was absent afterwards. The filesystem boundary therefore
places the failure after workload and trace normalization but before the final
measurement record; the intact manifest and writable attempt directory isolate
the absent qualification as the immediate cause. The old gate copied only its
outer `return 1` to the durable log, so the reason that qualification was absent
was initially lost. No row was sealed or promoted.

Capture had validated the excluded cursor qualification only after the timed
workload. That evidence belongs to the live runtime directory, while the row
belongs to the durable owner-only run. Capture now validates and snapshots the
qualification before it creates a partial directory or starts the timer, then
passes the immutable fields into measurement assembly. Missing or mismatched
qualification consequently fails immediately without consuming a minute or
leaving a partial. The TTY adapter also tees every typed attestation,
qualification, capture, finalization, and final-status result into
`gate-last.log` under `pipefail`, preserving the actual conformance-owner error
without weakening its exit status. The diagnostic partial remains immutable;
the correction requires a new signed candidate and prepared run.

The next zero-row run, `cp14-schema4-af87c8f0`, made the underlying control-flow
bug explicit without creating a partial or starting a workload. The
qualification window mapped, became focused, and received routed pointer
motion, but no target click was accepted before its 20-second timeout. The new
log retained both that `0/4` result and an immediately following capture
rejection for missing qualification. `attest_qualify_capture` was itself called
as the left side of `|| result=$?`; Bash therefore disabled implicit `errexit`
inside the whole function, and its sequential commands continued after
qualification failed. The earlier run's lost output and late missing-file
failure are consistent with the same path.

The adapter no longer relies on contextual `set -e` behavior. Attestation and
qualification now return their exact nonzero status explicitly before the next
operation can run. A failed qualification cannot invoke capture, regardless of
how the helper's caller handles its status. The early Rust admission remains a
second owner-boundary defense rather than a substitute for correct shell
sequencing. A fresh signed candidate is required; on its first physical run the
operator must move the pointer and click each of the four green targets before
the timed Kitty workload starts.

<!-- END IMPORTED BODY -->
