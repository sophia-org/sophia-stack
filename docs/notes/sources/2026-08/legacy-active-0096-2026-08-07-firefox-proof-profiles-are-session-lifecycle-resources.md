---
id: legacy-active-0096
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11", "validation", "tooling"]
---
# 2026-08-07: Firefox proof profiles are session-lifecycle resources

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3167–3191. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed release `0.1.0-fb1c38046d37` cleared the core `GetImage` failure.
Physical run `0041` completed with `protocol_errors=0`, `unexpected=0`, clean
layout health, normal logout, and clean frontend/resource teardown. Its
installed-run verifier still failed because the startup Kitty, selection, and
held-repeat proof stages did not complete. Firefox reported
`NS_ERROR_FILE_NO_DEVICE_SPACE` while writing its isolated profile.

The user runtime tmpfs was full: 44 prior `firefox-m10.*` proof profiles used
6.2 GiB under the Sophia session directory. The launcher created one isolated
profile for every proof run but never removed it on normal, failed, or
emergency teardown. The profiles are mutable test inputs, not retained
evidence; the reduced session log and installed-run archive already carry the
proof result.

Profile ownership now follows the session process lifecycle. After proving
that no prior wrapper or graphical session is active, the launcher removes
only stale `firefox-m10.*` directories beneath its exact private runtime
directory. It creates the current profile only after installing the cleanup
trap, and removes that exact directory after terminating supervised children.
A launcher regression locks in trap ordering, stale-profile reclamation, and
current-profile cleanup. The next installed Firefox run will reclaim the
existing backlog automatically; no manual cleanup sequence is required.

<!-- END IMPORTED BODY -->
