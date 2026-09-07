---
id: legacy-active-0207
date: 2026-07-19
recorded_date: 2026-07-19
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-19: X-Centric Product Direction And Wayland Retirement

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7060–7080. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sophia is an X-centric product built on a protocol-neutral architecture. X11
is the sole supported application protocol and the native X Server Frontend is
the product vehicle. Engine transactions, routed input, namespaces, portals,
rendering, and presentation remain independent of X11 object identity so a
future translator can be evaluated without moving authority.

The Smithay-backed Wayland frontend is retired from the workspace, CLI,
launcher, dependencies, documentation contracts, and validation gates. Its
source, tools, fixtures, last Kitty SHM evidence, and controlled linear
DMA-BUF evidence are frozen under `research/wayland`. Those results proved that
the Engine boundary was not X-shaped; they do not create an ongoing Wayland
compatibility promise.

Future application protocols are not deferred backlog. A translator or native
Sophia interface requires named product evidence, an explicit specification
amendment, existing authority boundaries, and bounded maintenance cost. Sophia
will not import another protocol ecosystem's shell, workspace, input,
presentation, or compositor-extension architecture.

<!-- END IMPORTED BODY -->
