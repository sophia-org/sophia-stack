---
id: legacy-active-0335
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-07-26: Private Legacy-WM Displays Are Leased One-Shot Endpoints

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10545–10580. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

A physical clipboard run exited during startup after the compatibility bridge
reported `no private X display available` ten times. The frontend display was
healthy; the failure was isolated to the bridge's synthetic X facade. Earlier
forced and aborted sessions had left owner-only socket nodes for every display
in the bridge's original `:90..:199` allocation range. Unix socket names remain
occupied after an ungraceful process exit, and the allocator treated each stale
name as a live policy endpoint.

The bridge now separates display-number ownership from the one-shot socket
name. A process-scoped file lease serializes each bounded display number, the
allocation range extends through `:4095`, and the socket name is unlinked as
soon as the configured legacy WM connects. The established Unix connection
continues to carry the synthetic X protocol, while a later abort has no socket
path left to leak. The lease remains held until the WM child and bridge worker
are stopped, and the kernel releases it even if the bridge is killed.

An integration regression launches two isolated fake legacy-WM processes at
once, requires distinct leased display numbers, and verifies that neither
accepted endpoint remains in `/tmp/.X11-unix`. This lifecycle belongs entirely
to the optional legacy-WM adapter; Engine, the real X frontend, renderer, and
blind WM protocol remain unchanged.

The immediate physical rerun confirmed the lifecycle fix. Sophia reached a
focused presented Kitty in 1,012 ms, entered workspace 3, launched two
independent peers, and completed 14 WM requests without a bridge restart or
degraded interval. The same run observed two selection-owner changes and one
selection conversion with content redacted. The operator confirmed that the
exact token was copied from the workspace-3 Kitty and pasted into the
independent workspace-1 Kitty after the workspace transition. The run flushed
all 1,397 expected input deliveries, retired native presentation without a
failure or live fence, logged out normally, restored the TTY exactly, and
revoked the frontend namespace. This closes the physical same-namespace
cross-workspace clipboard gate.

<!-- END IMPORTED BODY -->
