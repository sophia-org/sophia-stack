---
id: legacy-active-0135
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-05: installed operations have one packaged source of truth

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4349–4364. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The immutable release now carries its own operator runbook instead of relying
on a checkout after installation. The guide records the single retained AMD
two-output support boundary, required greetd, runtime-directory, libseat,
DRM/input, runtime-library, and application contracts, plus the exact status,
log, stop, emergency, fallback, evidence, and atomic rollback procedures. It
also labels native X11 scope, desktop-service isolation, physical coverage,
VRR, direct-scanout, cursor-plane, and rollback-retention limitations.

`sophia-status` reports the packaged guide and latest automatic cycle attempt.
The installed stop command discovers `/run/user/$UID` when a control-TTY shell
does not export `XDG_RUNTIME_DIR`, preserving the documented independent-stop
path. Installer fixtures verify that the runbook is checksummed, survives
install and rollback, and remains discoverable without the source tree.

<!-- END IMPORTED BODY -->
