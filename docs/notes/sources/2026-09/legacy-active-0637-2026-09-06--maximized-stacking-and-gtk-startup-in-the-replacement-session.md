---
id: legacy-active-0637
date: 2026-09-06
recorded_date: 2026-09-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11"]
---
# 2026-09-06 — Maximized stacking and GTK startup in the replacement session

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 20509–20568. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed release `f323323d` started at 22:50:41 UTC. Startup Kitty reached the
desktop, and two later Super+Enter launches were admitted at transactions 11
and 16. This accepts the preceding Kitty startup repair through normal use.

Super+F exposed a separate WM bug. The user's binding is `toggle-maximized`;
Super+M is `maximize-column`, and Super+Shift+F is `toggle-fullscreen`. Hagia
expanded the first window's geometry but retained its earlier place in the
bottom-to-top projection. The native composition log shows a 2526-pixel-wide
window followed by its 1258-pixel-wide neighbor, matching the reported overlap.
The WM protocol already defines ordered stacking. Hagia's pure logical
projection now orders ordinary, maximized, then fullscreen placements, with
focus last within each elevated layer. Toggling back restores the layout's
order. Engine continues to enforce and present the submitted order; no new
wire field or Engine policy is needed. The replacement WM has not been loaded
into the live session.

Ghostty and Thunar both launched from Super+Space and exited before admission.
Ghostty's failure at 22:52:03 UTC and Thunar's at 22:53:27 name major opcode 144,
minor 30, error code 1: RENDER `SetPictureFilter` returning `BadRequest`.
GTK printed different extension error names (`XSyncBadAlarm` and
`GLXBadContextState`), so those labels must not override the numeric request
evidence. The current dispatcher deliberately rejects this RENDER 0.6 request
while advertising 0.5. The earlier packed MIT-SHM repair did not cover the new
RENDER path. Session-bus warnings precede the fatal error; they are not evidence
that launcher execution failed.

The next GTK compatibility tranche is RENDER 0.6: picture transforms, filter
enumeration and selection, and the corresponding sampling behavior. Keep the
advertisement tied to implemented semantics, test malformed and unauthorized
requests, and rerun both real clients before claiming launch acceptance.
Silently acknowledging filter changes would conceal missing rendering
behavior. Further client requirements may emerge after this first refusal is
repaired.

The user also requested login without Kitty. Their selected profile is
`~/.config/sophia/desktop.kdl`, whose startup list named both `terminal` and
`quickshell-panel`; the legacy Hagia profile is not selected. Removed only the
terminal from the active startup list, retaining the terminal action mapping
and Super+Enter. Sophia's profile check accepts the result.

This exposed a launcher assumption: ordinary Hagia sessions unconditionally
received an eight-second focused-application startup deadline. A desktop with
only an unfocusable panel cannot satisfy that requirement. The launcher now
reserves that deadline for application proof profiles, including explicit
Firefox and TrueColor proofs. Ordinary Hagia sessions retain WM/profile
admission and supervision without requiring a login terminal. The source
launcher must be reinstalled before the next login with this profile; no
installed release or live process was modified.

Verification: Hagia's policy suite passes, including both window positions,
maximize/fullscreen bounds, focus and restoration. The full `nimble test` gate
passes with 187 Nim checks and the Sophia admission/restart corpus; it used an
empty `XDG_CONFIG_HOME`, as Sophia's workspace gate does. The first invocation
read the personal desktop profile and failed two fixture preparations on
`UnknownApplication("terminal")`; isolation resolves that harness issue.
Hagia formatting and module-layout checks pass. Sophia's 21 launcher-safety
tests, Rust formatting, shell syntax, and both repository diff checks pass.

<!-- END IMPORTED BODY -->
