---
id: legacy-active-0554
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "security"]
---
# 2026-08-28: visual policy stays modular without moving compositor authority

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17173–17212. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The open architecture question was whether Sophia's Engine-centered composition
path would bake one desktop's chrome and effects into the core. Splitting the
compositor out of Engine would answer that concern by creating a second visual
authority, which breaks atomic validation, damage, frame scheduling, and
scanout ownership. Keeping every visual choice in Engine would preserve
authority at the cost of making Sophia's taste the limit of every WM and shell.

The decision separates policy and implementation from authority. WM and shell
authors own role-appropriate visual style and semantic effect intent. Ordinary
novel artwork remains shell-rasterized and reusable through content-addressed
textures. Recurring mathematical operations become small Engine capabilities.
Operations that need compositor pixels or specialized renderer code may be
lowered by a trusted visual provider after Engine validates the complete intent.
No public policy protocol carries shaders, native handles, or arbitrary renderer
state, and the blind WM still receives no pixels or metadata.

The first provider seam is a private Rust trait implemented by separately
maintained modules linked into the trusted renderer build. The installed desktop
profile selects and hashes that provider set as an immutable release input. It
is intentionally version-coupled: a dynamic-library ABI is premature, while a
sandboxed effect host would add buffer transfer, fencing, and failure semantics
before evidence requires them. Neither is ruled out after multiple providers
demonstrate what a stable boundary must contain. Runtime code supplied by a WM,
shell, or application is ruled out.

Engine owns the animation clock, effect lifetime, offscreen allocation, damage,
degradation, composition, and presentation. Clients declare bounded transitions
and committed fallbacks. Missing, stale, malformed, or failed optional effects
cannot remove mandatory controls or trust state; Engine renders the fallback or
preserves the prior coherent scene. An active overlay or effect requiring
composition also makes that exact frame ineligible for direct scanout.

This decision does not displace Milestone 14's active scanout work. The scanout
roadmap gains the generic composition-required guard now. The provider registry,
blur and focus-transition proofs, protocol records, and independently packaged
provider proof follow direct scanout and precede broader `sophia_shell_v1`
stabilization.

<!-- END IMPORTED BODY -->
