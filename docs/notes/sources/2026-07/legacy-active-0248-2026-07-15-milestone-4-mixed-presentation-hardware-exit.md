---
id: legacy-active-0248
date: 2026-07-15
recorded_date: 2026-07-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "tooling"]
---
# 2026-07-15: Milestone 4 Mixed-Presentation Hardware Exit

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8265–8286. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The isolated mixed exporter passed while the persistent session aborted in the
Radeon command-submission thread. Lifecycle tracing and a GDB backtrace showed
that one CPU-plus-DMA-BUF frame drew, swapped, submitted, page-flipped, and
retired successfully; the asynchronous rejection surfaced in the following
ordinary CPU upload. Reusing the GL context across that imported-image boundary
was the distinguishing lifetime.

The native renderer now destroys the mixed frame's GL context after
`glFinish`, swap, and front-buffer lock. The returned GBM owner retains the
scanout surface independently through KMS submission and page-flip retirement;
the following CPU frame receives a fresh context. DRI3 `Open` also returns an
independently opened same-GPU render node instead of the compositor's primary
KMS node, preserving the protocol/backend ownership boundary.

The retained X13 schema-14 proof passed its strict verifier with 76 mixed Flip
completions, one controlled Skip, 77 matching Idle events and idle-fence
triggers, nine acquire-gate waits, zero submit/retire failures, and zero live
sources, fences, or transactions. The established software xterm/resize half
also passes, completing the Milestone 4 hardware exit.

<!-- END IMPORTED BODY -->
