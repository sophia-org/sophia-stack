---
id: legacy-active-0218
date: 2026-07-13
recorded_date: 2026-07-13
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "architecture"]
---
# 2026-07-13: Per-Connection X Admission Boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7326–7342. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The native X frontend now calls a protocol-neutral session policy after setup
authentication and before allocating X client or resource-range identity. The
policy receives only the bounded authentication method and kernel Unix peer
credentials; it never receives raw cookie bytes. A successful decision returns
an immutable `ClientAdmissionContext` retained in an admission lease. Native
X11 setup failure represents denial, and teardown or any early worker error
revokes the lease after route and resource cleanup.

The live classic session backs that policy with its session-owned
`NamespaceRegistry`: it requires a peer UID matching the effective session UID,
allocates a distinct admission per connection, and intentionally assigns those
admissions the same classic-shared namespace. This removes the listener-wide
identity shortcut without weakening classic X semantics. Confined launch and
targeted supervisor revocation are now implemented as described above.

<!-- END IMPORTED BODY -->
