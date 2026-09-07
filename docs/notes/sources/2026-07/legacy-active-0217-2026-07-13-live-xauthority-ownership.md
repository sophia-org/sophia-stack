---
id: legacy-active-0217
date: 2026-07-13
recorded_date: 2026-07-13
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security", "architecture"]
---
# 2026-07-13: Live Xauthority Ownership

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7310–7325. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The live X session no longer relies on an unauthenticated owner-only socket. Its
supervisor obtains a fresh 128-bit cookie from the kernel for every run, writes
a standard `FamilyLocal` `MIT-MAGIC-COOKIE-1` record with mode `0600`, syncs the
complete record before exposing its path, passes `XAUTHORITY` to both launched
terminals, and removes the file through explicit and drop cleanup. A private,
owner-only `XDG_RUNTIME_DIR` is preferred; the random, create-new owner-only
file remains safe when the system temporary directory is the fallback.

The frontend validates that cookie before invoking session admission. Policy
sees only `MitMagicCookie1` provenance and kernel peer credentials, never the
secret. A regression proves bad cookies do not invoke policy, while the accepted
connection is admitted once and revoked once. Fresh per-session generation is
the rotation boundary; confined launch credentials remain future policy work.

<!-- END IMPORTED BODY -->
