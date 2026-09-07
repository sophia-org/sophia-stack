---
id: legacy-active-0188
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-26: One Snapshot Now Accounts For All Primary-Frame Pixels

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6368–6401. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Compositor damage could not safely become output scheduling authority while
client and software-cursor changes remained separate. Sophia now builds one
bounded immutable output-damage snapshot for every CPU and mixed CPU/DMA-BUF
frame. It contains only pixel-relevant Engine facts: output shape and scale,
ordered opaque surface IDs, committed generations, geometry, buffer identity,
the compositor display list, and optional software-cursor bounds. It contains
no XIDs, application metadata, WM facts, protocol objects, or renderer-native
resources.

The reducer damages old and new client extents for generation, geometry, or
buffer changes; damages all involved client extents for stacking, creation, or
removal; includes old/new compositor nodes; and includes old/new software
cursor bounds. Initial presentation and output shape/scale changes force full
output. Hardware cursors keep their independent plane lifecycle. Snapshots are
bounded to 1,024 client nodes and revalidated before reduction so public record
mutation cannot bypass ID, ordering, capacity, or output invariants.

CPU and mixed frames retain this snapshot through cloning, latest-frame-wins
queueing, native export, accepted KMS submission, and page-flip retirement.
QEMU established full initial plans on both outputs and emitted separate
compositor, combined-output, and repaint records for every retired primary
frame. During pointer focus, the matching combined plan correctly became full
when the same transaction also advanced client generations; requiring a
chrome-only partial plan would have hidden client damage. The verifier now
requires the compositor record, combined record, and safe partial-or-full
decision from the same retired frame before the following key.

The planner still does not authorize partial drawing. Native destination
buffers are not yet proven preserved or reconstructed for the reported region.
The next optimization step is explicit destination-buffer age/history and a
full fallback whenever that proof is absent.

<!-- END IMPORTED BODY -->
