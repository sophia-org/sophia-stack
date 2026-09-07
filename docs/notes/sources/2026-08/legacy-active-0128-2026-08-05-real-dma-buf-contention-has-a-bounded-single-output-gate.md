---
id: legacy-active-0128
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-05: Real DMA-BUF contention has a bounded single-output gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4169–4206. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Lavapipe could exercise the Vulkan client path but could not populate Sophia's
native import cache. The new rendering image instead connects QEMU's virgl GPU
to an explicit host render node and runs three unmodified `glxgears` clients.
The image includes Mesa's dynamically loaded GLX vendor library and Xlib's
locale database; the latter is required for unmodified xmobar to create its
font set and publish the 14-pixel work-area reservation.

Starting all producers simultaneously made the setup measure admission races
rather than steady-state rendering. The final profile uses Sophia's existing
bounded application-admission FIFO: the first producer and xmobar establish a
stable desktop, then two session actions introduce one producer at a time. No
sleep declares admission complete. Each action must reach exact presented
pixels, settled layout, and the matching application-admission record before
the next action is sent. Initial client dimensions match the deterministic
work-area allocations, so the retained run committed all six WM transactions
without timeout, recovery, or restart.

Two consecutive production runs passed the contract. The latest bounded window
retired 97 frames: 32, 33, and 32 from the three distinct DMA-BUF surfaces.
Completion reported 816 imports, 1,069 cache hits, 816 final evictions, zero
live cache entries, 818 balanced renderer requests/completions, no worker
failure or stall, and a 47 ms maximum request. Xmobar produced 56 CPU patches
beside those clients. All nine frontend controls were delivered with a 1 ms
maximum acknowledgement, Present cadence advanced for every retained sample,
and native, layout, protocol, application, and frontend ownership drained
cleanly.

The verifier derives per-surface counts only between causal window markers and
checks the marker totals against the raw retirements. Mutations starve one
producer, falsify those totals, remove cache reuse, leave worker debt, exceed
the 100 ms request budget, inject layout recovery, or silence the CPU bar; each
must fail. The two-output topology is retained, but output 2 carried only its
startup baseline. This closes the single-active-output cell, not the roadmap's
inter-output fairness requirement. That remaining proof needs active work on
both outputs in one render-device group after the shared-worker prerequisite.

<!-- END IMPORTED BODY -->
