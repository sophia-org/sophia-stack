---
id: legacy-active-0118
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-06: Interactive QEMU is separate from acceptance choreography

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3902–3928. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The retained `xmonad-m8-soak` guest is an unattended acceptance workload. Its
thirty-minute clock, scheduled compatibility-bridge restarts, and host-driven
input make it a poor environment for investigating one live interaction.
`xmonad-interactive` now packages the same isolated terminal, Vulkan, Firefox,
launcher, xmonad, and two-output native-X stack without a runtime deadline,
fault injection, or automated action sequence. The guest powers off only after
the ordinary logout action.

The supported interactive display is an unnetworked Unix-domain VNC socket.
QEMU traces only the relevant VNC and input-core boundaries into a FIFO; a
stream reducer retains the first display, keyboard, pointer, motion, and button
boundary crossings plus bounded keyboard-count checkpoints without persisting
raw keycodes, coordinates, or button values. The guest's existing reduced
records then distinguish virtio-device discovery, Engine intake and routing,
focused-client targeting, output projection, and cleanup. A fail-closed
verifier covers that entire chain plus
manual terminal launch, later typed input, focus, close, and logout order. The
Q35 guest explicitly disables `vmport`; otherwise QEMU activates its legacy
absolute `vmmouse` ahead of the declared relative virtio mouse, and viewer
motion never reaches the guest. The RFB client honors QEMU's pointer-type
pseudo-encoding and keeps relative button events at zero delta.

The tooling regressions and a complete RFB-to-Engine QEMU capture pass. One
human-visible viewer capture remains before the supported backend gate closes.

<!-- END IMPORTED BODY -->
