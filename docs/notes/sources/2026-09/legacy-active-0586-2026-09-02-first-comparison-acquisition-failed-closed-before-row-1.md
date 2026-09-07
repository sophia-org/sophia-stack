---
id: legacy-active-0586
date: 2026-09-02
recorded_date: 2026-09-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-09-02: first comparison acquisition failed closed before row 1

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18526–18555. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical CP-14.2 acquisition attempt retained no admissible row. It
exposed three independent infrastructure defects before measurement: capture
directories inherited the operator's `umask 002` and reached the privileged
adapter as mode 0775; unprivileged preflight treated root-private tracefs as a
missing DRM tracepoint; and the generated Kitty control socket was 111 bytes,
beyond Linux's 107-byte pathname limit. Kitty could render the changing stream
but could never satisfy the remote-control readiness contract, so the owner
timed out after 30 seconds and killed its child.

The acquisition owner now tightens both incoming and attempt directories to
mode 0700 independent of umask. The tracefs adapter has a side-effect-free
privileged probe, allowing preflight to prove the exact
`drm_vblank_event_delivered` source before it creates an attempt even when the
tracefs catalog is root-private. Kitty ignores personal configuration and uses
a per-owner mode-0700 namespace below `$XDG_RUNTIME_DIR`; path length is checked
before launch, and failure cleanup removes the socket and namespace.

The same bring-up also found that the generic immutable Sophia artifact had
been packaged with `hagia_included=false`, while an older Hagia greetd entry
remained installed. The physical attempt therefore used clean, signed,
upstream-matching Hagia sources through the local-VT development launcher.
Installed-surface reconciliation is now shared by activation and packaged
rollback: the target release is validated before mutation, Sophia-managed Hagia
entries are removed when the target excludes Hagia, target Hagia entries are
restored when it includes them, and foreign path collisions are preserved. A
new Hagia-included artifact must still be built and installed before its greetd
entry is valid. None of this failed bring-up is comparison evidence.

<!-- END IMPORTED BODY -->
