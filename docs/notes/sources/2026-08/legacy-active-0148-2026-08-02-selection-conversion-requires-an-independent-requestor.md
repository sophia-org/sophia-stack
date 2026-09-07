---
id: legacy-active-0148
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-02: selection conversion requires an independent requestor

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4722–4750. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first complete post-floating physical workflow reached all eight Firefox
stages, both normal and forced browser closes, and all six original Kitty
retention checkpoints. It nevertheless ended with one selection-owner change
and zero conversions. The browser page had advanced because its paste handler
prevented the default operation and wrote the expected token itself. Even a
real same-process Firefox copy and paste may reuse locally retained selection
content without sending core `ConvertSelection`, so neither signal proved the
cross-client X11 path required by Milestone 10.

XLibre's `ProcConvertSelection` and yserver's independent Rust implementation
agree that conversion begins only when a requestor sends core opcode 24. The
server then routes `SelectionRequest` to the current owner or returns
`SelectionNotify(property=None)` when no owner exists. The server must not
fabricate a conversion from an application-level paste event. Sophia's X
authority behavior is therefore unchanged; the defect was in the proof
workload and its completion contract.

The physical M10 page now opts into a peer-selection mode that leaves default
browser paste intact and advances only when an input contains the exact bounded
token. Kitty B validates Firefox-owned `CLIPBOARD` and `PRIMARY`, publishes two
new redacted title checkpoints, then returns ownership so Firefox consumes both
selections from an independent X client. The verifier requires an ordered
owner-change and conversion in each of the four directional intervals and at
least four of each operation overall. QEMU M8 retains its existing same-process
fixture path. Offline coordinator, reducer, and fail-closed verifier
regressions pass; a fresh physical workflow remains required.

<!-- END IMPORTED BODY -->
