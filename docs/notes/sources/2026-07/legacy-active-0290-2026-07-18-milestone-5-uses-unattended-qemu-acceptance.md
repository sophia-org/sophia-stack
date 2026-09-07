---
id: legacy-active-0290
date: 2026-07-18
recorded_date: 2026-07-18
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation", "tooling"]
---
# 2026-07-18: Milestone 5 Uses Unattended QEMU Acceptance

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 9057–9071. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Machine-specific X13 capture is no longer an application-promotion gate. The
repeatable acceptance boundary is a diskless, networkless QEMU guest that owns
virtio DRM/KMS, guest console state, and libinput-backed virtio keyboard and
pointer devices. Direct hardware runners remain optional compatibility
diagnostics.

`tools/qemu_milestone5_acceptance.sh` rebuilds the guest and runs strict
two-xterm presentation/input, emergency Ctrl-Alt-Backspace recovery, and classic
plus confined GTK3 profiles without operator input. The first aggregate run
exposed a stale schema-1 poller assertion in emergency recovery; updating the
harness, verifier, and fixtures to the schema-2 tap-policy record closed it. The
rerun passed all four scenarios. The strict three-class baseline then passed

<!-- END IMPORTED BODY -->
