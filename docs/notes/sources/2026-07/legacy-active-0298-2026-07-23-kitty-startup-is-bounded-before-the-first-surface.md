---
id: legacy-active-0298
date: 2026-07-23
recorded_date: 2026-07-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11"]
---
# 2026-07-23: Kitty Startup Is Bounded Before the First Surface

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9237–9293. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first Kitty-only `seat0` run acquired both outputs and discovered fourteen
libinput devices, but remained blank until the independent guard restored the
TTY. The session log contained no focused or committed application surface.
Kitty's desktop-settings request failed after 10.389 seconds because a bare TTY
had no usable portal service; emergency recovery was requested at the same
boundary. The separate non-modesetting Kitty trace still passed 207 X requests,
one DRI3/Present transaction, one runtime surface, and `first_error=none`, so
the direct-Mesa GLX path was not the failing boundary.

The first private-bus attempt activated the host notification and XFCE settings
services without a usable desktop display, adding another nondeterministic
startup path. The Kitty gate now matches the passing standalone trace instead:
Wayland variables are removed, desktop-service bus activation is disabled, and
the no-WM profile forces opaque X11 rendering. The live session accepts a
generic bounded startup deadline and succeeds only after a focused CPU-detail
or DRI3/Present surface crosses actual native presentation. A missing surface,
uncommitted surface, missing visual content, or unpresented frame reports a
distinct reduced stage and returns through the normal TTY cleanup path after
eight seconds.

Native normal sessions initialize an empty output runtime immediately. Physical
pointer motion is polled before a client surface exists, the compositor-owned
classic hardware cursor begins at the primary-output center, and unfocused
keyboard and pointer-button events remain unrouted. This removes the prior
first-surface dependency from cursor feedback without introducing an
application-specific branch in Engine.

The first centered-cursor repair was insufficient. A physical rerun still
showed a frozen inherited pointer beside Sophia's moving pointer because the
atomic display owner attempted to clear cursor state through deprecated legacy
cursor ioctls and discarded every error. Backend-live now discovers cursor
planes compatible with every selected CRTC, atomically detaches them before
first use, retains an ARGB cursor framebuffer, and performs coalesced atomic
attach/move commits. Hardware and software paths share one canonical classic
X11 pointer raster.

The same rerun proved that input discovery was not the keyboard failure:
libinput observed a key and routed pointer motion plus a button. Kitty created
and mapped a surface and submitted one DRI3/Present frame, but no later
authority batch arrived after asynchronous KMS retirement. Initial focus and
startup-content reconciliation lived only in the authority-batch branch, so
the retired surface never gained focus and keyboard delivery remained gated.
Present retirement now carries its transaction and surface back to the session
loop; the shared reconciliation path runs after both authority work and KMS
service, sends X11 focus control, and recognizes the retired frame without
requiring a second Present. Startup diagnostics now report actual committed
surfaces, focus/control state, Present retirement, native submission failures,
callbacks, and per-output in-flight state.

Finally, the physical gate had not actually matched the passing Kitty smoke:
it still loaded the normal user configuration. The guarded profile now uses
`--config NONE` with only the forced X11, opaque-background, and diagnostic
title arguments. Normal Kitty configuration compatibility remains outside the
minimal input gate.

<!-- END IMPORTED BODY -->
