---
id: legacy-active-0094
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-07: VT handoff must settle detached renderer work

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3111–3138. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed Firefox attempt `firefox-runs/0001` proves that release
`0.1.0-7a6be56c6b29` selects the dedicated schema-4 Firefox ledger. The attempt
then failed four seconds after startup, before Firefox launched, when the
physical Ctrl+Alt+F2 path queued and prepared VT target 2. KMS startup and
retirement were healthy, the runtime tmpfs remained at one-percent use, and no
proof profile survived teardown. The terminal error was `WorkerPending`.

VT suspension drained native scanout and detached the skipped Present. It then
exported retained renderer images for switch-back recovery while the renderer
worker still held the detached frame as its in-flight result. The result was
already irrelevant to presentation, but image export rejected any in-flight
work rather than collecting it. The earlier maintenance correction covered
image clearing during final teardown, not retained-image export during VT
handoff.

Renderer-image maintenance now has one settlement path shared by export and
clear. It waits within the existing bounded maintenance deadline, discards an
exported lease only after the Present has been detached, clears the associated
worker-frame classification, and then reads the older promoted image set.
Worker failure or stall remains fatal. A deterministic worker regression
submits one frame and immediately enters image export; it requires the real
backend failure and rejects the former `WorkerPending` result. This refines one
backend worker step without changing handoff admission or cross-authority
ordering, so the existing transition reducers remain the applicable model and
no TLA+ state expansion is needed.

<!-- END IMPORTED BODY -->
