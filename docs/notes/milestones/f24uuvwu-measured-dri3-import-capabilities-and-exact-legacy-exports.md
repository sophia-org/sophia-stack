---
id: f24uuvwu
date: 2026-09-08
kind: milestone
status: recorded
tags: [milestone, rendering, x11]
---
# Measured DRI3 import capabilities and exact legacy exports

## Result

The first implementation checkpoint of [t069](../plans/6tewvlbh-universal-device-negotiation-across-sophia-clients.md)
replaces invented DRI3 modifier lists with measured native EGL import
capabilities. The frontend latches the provider and bounded snapshot together;
ordinary requests read cached data. Unknown formats and unavailable evidence
answer empty lists. External-only and implicit modifier entries are excluded.
Multiple output devices admit only measured common linear layouts. Legacy
exports now refuse offsets and strides their wire format cannot represent.
There is no executable classification or application-specific Engine code.

This is an implemented slice, not completion of universal transfer or physical
client acceptance. [The X authority contract](../../sophia-x-authority.md)
describes the resulting protocol behavior.

## Evidence

The candidate is based on `e3c39890`. Exact changed-source hashes, patch, tested
binary hash, device identities and logs are retained under
`~/.local/state/sophia/development-evidence/t069-measured-capabilities-20260908/attempt-001/`.
The archive's commit receipt identifies the resulting signed checkpoint.

`cargo xtask check` passed: 2,763 tests, zero compiler or Clippy warnings,
20 archived proof verifications and buffer-age pixel equivalence. Formatting
and whitespace checks passed. The external fake-query tests cover bounded
counts, malformed responses, duplicate conflicts and cleanup. Socket tests
prove the first provider and snapshot remain paired across clone/rebind,
including actual returned FDs. Legacy export tests prove exact representable
layouts and refusal without mutation for unrepresentable ones.

Private offscreen tests on AMD Navi 31 (`renderD128`, PCI `03:00.0`) and Raphael
(`renderD129`, PCI `16:00.0`) prove exact XR24/AR24 pixels through measured linear
imports in both directions. Measured, allocatable tiled buffers also import
correctly on their allocating GPU. The tests neither launch a browser nor
change live input or scanout. No build was installed and no session restarted.

## Limits

The pixel probes use CPU-written buffers. They do not prove source-GPU-render
to destination-GPU-read synchronization or a renderer transfer fallback. The
capability snapshot covers the fixed native output-device groups; it is not
an inventory of every headless GPU on the seat. Import capability does not
promise that a device can allocate or scan out every advertised layout.

The [Brave investigation](../investigations/uqnx2t2b-brave-gpu-restarts-after-va-buffers-fail-gbm-import.md)
remains distinct: an import that fails inside a client before submission cannot
be intercepted by a Sophia copy path. No physical Brave result is claimed here.
The remaining device-lifetime, transfer, recovery and acceptance gates belong
to the existing t069 plan and task row.
