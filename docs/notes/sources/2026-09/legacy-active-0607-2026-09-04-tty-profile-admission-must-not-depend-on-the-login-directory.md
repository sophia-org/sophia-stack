---
id: legacy-active-0607
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "tooling"]
---
# 2026-09-04: TTY profile admission must not depend on the login directory

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19237–19256. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The zero-row run `cp14-schema4-94bf507e` never took DRM ownership and preserved
no measurement partial. Its outer absolute xtask correctly entered the Sophia
launch stage, but `start_sophia_tty3.sh` invoked the repository-only
`cargo xtask` alias from the operator's TTY login directory. Cargo could not
discover `.cargo/config.toml` there, so profile admission refused the launch
with `no such command: xtask`. The display manager was still running; this was
a safe pre-takeover failure rather than a recovery failure.

The desktop-comparison adapter now passes its already-running absolute xtask to
the TTY launcher for profile admission. This removes a redundant Cargo process,
keeps the check on the same tooling executable that admitted the comparison,
and makes its result independent of the operator's current directory. Direct
launcher use retains the check through an explicit offline `cargo run` with the
workspace manifest path, so it also no longer relies on alias discovery. The
launcher rejects a supplied checker unless it is an absolute executable regular
file. A fresh signed candidate and owner-only run are required; `94bf507e`
remains useful only as a launch-path diagnostic.

<!-- END IMPORTED BODY -->
