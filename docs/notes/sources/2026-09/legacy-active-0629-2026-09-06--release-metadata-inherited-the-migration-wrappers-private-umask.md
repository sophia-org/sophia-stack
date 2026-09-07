---
id: legacy-active-0629
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-09-06 — Release metadata inherited the migration wrapper's private umask

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20273–20299. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The native-launcher wrapper used `umask 077` for personal configuration. The
packager inherited it for generated release metadata, and installation preserved
mode 0600 while changing ownership to root. The selected release was installed,
but the unprivileged post-install manifest read failed before policy-client copy
and profile publication.

Packaging now sets umask 022 for public release files. Installation also repairs
manifest, checksum-list and session-entry modes when copying older artifacts.
The installation fixture now builds an artifact under umask 077 and verifies
mode 0644 after installation; install, activation and rollback checks pass.
The private profile wrapper retains umask 077 and narrows umask 022 to its
packaging subprocess. An exact-release repair/resume script is staged at
`/tmp/finish-sophia-native-launcher.sh`; it needs the operator's sudo password.
It verifies release hashes and configuration preimages before completing the
interrupted installation, without restarting the running session.

The permission repair completed on the host. Both staged files passed the
installed parser as uid 1000; `UnsafeOwner` was consistent with running the
whole wrapper under sudo against those user-owned files. The wrapper now
requires the session user and requests sudo only for the system step. The
remaining installation completed as niltempus: exact-release verification,
Hagia copy, core configuration backup and publication of both mode-0600
user-owned profiles. No live restart occurred; Super+Space awaits the next
login and physical acceptance.

<!-- END IMPORTED BODY -->
