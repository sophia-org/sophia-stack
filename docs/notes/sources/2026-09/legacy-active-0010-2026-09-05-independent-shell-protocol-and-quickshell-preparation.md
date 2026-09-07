---
id: legacy-active-0010
date: 2026-09-05
recorded_date: 2026-09-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell", "architecture"]
---
# 2026-09-05: independent shell protocol and Quickshell preparation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 307–354. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The operator chose Quickshell as a downstream reference consumer for richer
`sophia_shell_v1` capabilities while retaining X11 as the application priority.
The shell role remains independent of X11, Wayland, Qt and Quickshell. Future
application frontends translate into Engine's protocol-neutral boundary; they
do not redefine shell IPC. Engine retains physical input, authoritative scene
state, pacing and composition; the WM retains blind spatial policy. Toolkit
integration and local widget behavior stay downstream.

The [reference-client audit](../../../shell-reference-client-audit.md) captures one panel,
an anchored popout and a local control, maps each need to its owner/current gap,
and requires an independent C content client for the later prototype. Narthex
remains the independent descriptor implementation; the Noctalia survey remains
research provenance. Existing revisions 1/2 provide descriptors and reservations,
not arbitrary content. Cached textures remain the initial content direction;
FD/DMA-BUF admission requires transport, lifetime and performance evidence.

Created `sophia-org/quickshell` from the official GitHub mirror, retained the
canonical Forgejo remote as `upstream`, and published `sophia` at upstream
`2d3b3e9c70ef380dff751b61d334dc88df016c29`. The fork's issue tracker is enabled;
local downstream edits add `SOPHIA.md`, link it from the README, and change the
crash-report default to the fork. The downstream changes and Sophia
documentation form separate repository commits. No upstream contribution was
submitted.

The pinned Nix attempt stopped before configuration because the daemon socket
was absent. The operator installed Void dependencies using the temporary
installer; its final form skips installed packages and uses Void's case-sensitive
`SPIRV-Tools` name. Native CMake 4.2.2 configuration and the Debug build passed
with GCC 14.2.1, Qt Core/Quick/ShaderTools 6.11.1, tests enabled and default
features. The final crash-default edit also passed an incremental build.
Eight of nine offscreen/software CTest suites passed: the upstream
`TestPopupWindow::moveWithParent` assertion at line 115 reported x = 12 versus
expected 20; Qt's minimal platform reproduced the failure with x = 10. No test
was changed or skipped. Its cause and behavior under isolated X11 remain open
downstream baseline debt, not a Sophia content-protocol failure.

Preparation now has a build, retained test results and bounded follow-up work.
Content implementation still needs separate admission after the CP-15 coherence
gates. CP-14.3 stays `NOW`; no wire records, protocol artifacts, application
frontend, live profile or installed shell changed in this tranche.

Validation: both checkouts pass `git diff --check`; all 85 relative links in
the touched Sophia documents resolve. Protocol schemas, generated artifacts
and bindings have no diff against HEAD. Sophia changes are documentation-only,
so no Sophia build or physical acceptance campaign was run for this tranche.

<!-- END IMPORTED BODY -->
