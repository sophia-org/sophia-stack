---
id: legacy-active-0525
date: 2026-08-23
recorded_date: 2026-08-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-23: one surface may need two retained source realizations

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16039–16061. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The transaction-identity rerun admitted Helium and rendered its CPU authority
raster through several page flips. A later retained repaint planned generation
5, selected CPU handle 4 as the exact variant, and ended with
`MissingCpuSource(4)`. The handle had already rendered successfully; it had not
failed intake, but that evidence did not distinguish final residency from
source-set construction.

The retained source builder had made two valid realizations mutually exclusive.
When a Present was in flight, it supplied the compositor-owned renderer image
for the canonical DMA-BUF and skipped every resident CPU variant for that same
surface. Engine still received the complete content set and correctly selected
the exact authority raster for the head. Lowering then failed because its source
set contained only the other realization.

An in-flight renderer image now joins, rather than replaces, the surface's CPU
variant sources. Each keeps its protocol-neutral `BufferSource` identity, so the
per-head plan still decides which exact realization is drawn. A focused
regression supplies one DMA-BUF renderer image and one CPU authority raster for
the same surface and requires both to survive source-set construction. The
signed installed switcher rerun remains the promotion gate.

<!-- END IMPORTED BODY -->
