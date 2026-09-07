---
id: legacy-active-0635
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "tooling"]
---
# 2026-09-06 — Shared build artifacts hid the current source-layout failure

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20443–20467. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`cargo xtask check` invoked from the main checkout used an artifact embedding
`/tmp/sophia-client-fixes-3fc` as its manifest directory. The earlier isolated
client-fix verification had shared the main target directory. The reused xtask
therefore ran checks in the temporary tree and failed archive verification when
it looked for Hagia under `/tmp`. Removed the temporary target symlink, cleared
workspace package artifacts while retaining the registry dependency cache, and
rebuilt from the main checkout. Separate worktree target directories are now
documented in the tooling contract.

The fresh check passed 2,496 test executions and Clippy, then exposed the actual
source-layout violation on the combined tree: the RENDER probe had grown
`basic_smokes.rs` to 1,225 lines. Moved that complete probe, unchanged, into
`render_smoke.rs` behind the existing X-authority command facade. The basic
probes now occupy 960 lines and the RENDER probe 264; the debt ledger is
unchanged. The layout gate passes. The real-client RENDER smoke reports version
0.5, the expected composited and glyph pixels, and zero protocol errors.

The complete `cargo xtask check` now passes from the main checkout without path
or configuration overrides: 2,496 test executions, Clippy, profiles, source
layout, verifier fixtures, all retained archive families, and host buffer-age
pixel equivalence. The gate ran outside the tool sandbox because its Unix-socket
fixtures require host socket access. No live session was restarted or installed.

<!-- END IMPORTED BODY -->
