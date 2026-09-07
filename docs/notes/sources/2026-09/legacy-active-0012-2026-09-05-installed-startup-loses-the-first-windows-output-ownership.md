---
id: legacy-active-0012
date: 2026-09-05
recorded_date: 2026-09-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "rendering", "tooling", "architecture"]
---
# 2026-09-05: installed startup loses the first window's output ownership

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 397–439. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The installed Hagia launch `20260905T205938148091817Z`, Sophia release
  `0.1.0-86b5fe1d20bc` (full commit
  `86b5fe1d20bca31b463f1f587a83017020f24cdd`), settled DP-1 and DP-2, then
  exited after the first Kitty layout committed. The fatal owner-loop error was
  `visible Present surface is missing from the presentation order`. Existing
  installed-session and Hagia session logs retain the launch/profile identity,
  transaction 284, surface 2097166, and bounded cleanup with zero cleanup errors.
- The per-output composition change assigned output ownership only when
  constructing a new layer. Startup had already cached a raster with no owner;
  the WM proposal cloned it without assigning the projection's output. Every
  output display list then excluded it. The same branch retained the former
  owner when moving a cached layer between outputs. Accepted placements now
  always supply their output while preserving the committed layout until commit.
- A second deterministic reproduction found that GPU Present source discovery
  used the primary display list even when lowering a secondary output. Source
  discovery now covers the union of the actual applicable output lists and
  lowering consumes those same lists. Surfaces are resolved once; ownership
  filtering remains in force. A scrolled surface overlapping only a neighbour
  follows invisible-Present rejection before gathering sources.
- Both focused regressions failed before their respective fixes: first
  placement retained `None`, and secondary Present produced the physical
  failure's exact missing-order error. The source regression also checks the
  secondary-only case, omission of the owning output, and distinct CPU/GPU
  layers after lowering two heads. `cargo xtask check` passes, including 2,410
  Rust test executions, Clippy, source-layout checks, retained archive
  verification, buffer-age GPU pixel equivalence, and verifier fixtures.
- This repairs Engine/session plumbing for WM-selected ownership; WM/shell
  protocol authority and the opt-in scripting endpoint are unchanged. The next
  physical check is an installed startup, first terminal, and use of both
  outputs. Earlier recovery evidence remains retained; no comparison matrix
  is reopened.
- Physical retry: the operator reports being back in the live session. Launch
  `20260905T211245949274074Z` runs installed fix
  `84c109c68c79fba228833c1fef2e3330c365248f` with the same profile digest as
  the failed launch. Both outputs settled; the inspected log contains 421
  retired Presents across two application surfaces and head composition queues
  on both outputs, with no fatal owner-loop error. Four supersession warnings
  remain in the log; this is startup recovery evidence, not a claim that every
  workflow is clean. Visible use of both displays and normal logout remain
  pending operator acceptance.

<!-- END IMPORTED BODY -->
