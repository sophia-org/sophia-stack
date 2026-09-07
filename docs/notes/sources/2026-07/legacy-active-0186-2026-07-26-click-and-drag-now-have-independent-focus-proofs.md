---
id: legacy-active-0186
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "validation"]
---
# 2026-07-26: Click And Drag Now Have Independent Focus Proofs

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6312–6344. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first unattended pointer-focus proof used only a click-drag gesture. That
exercised the generic ordered handoff but left plain click-to-focus as an
inference. The M7 harness now creates two independent focus transitions against
an unfocused visible surface. The first sends primary press/release and a
following key. It then moves focus away through the WM and sends primary
press/motion/release plus a different following key.

Both real QEMU sequences passed through virtio input, Engine hit testing, the
blind-WM focus request, X-frontend acknowledgment, ordered deferred-input
release, focused-border composition, combined output-damage retirement, and
keyboard delivery to the selected opaque surface. The click released two
records and the drag released three. Completion retained two output baselines,
clean bridge recovery and logout, zero stale WM responses, zero protocol
errors, and no native cleanup debt.

The verifier treats each gesture as its own bounded state machine. It rejects a
missing or overlapping gesture, an incomplete handoff, insufficient click or
drag records, a missing key-probe boundary, a key routed to another surface,
missing target border/damage/repaint evidence, or a missing completion marker.
Physical libinput and visual confirmation remain required because QEMU cannot
prove the actual mouse, display, or TTY path.

The physical operator gate now has the same two-sequence shape through
`tools/start_sophia_xmonad_pointer_focus_tty3.sh`. The wrapper guides a plain
click and key, moves focus away, guides a click-drag and different key, then
automatically checks both ordered handoffs after normal logout. Its verifier
requires two independent requests and at least press/release for the click
plus press/motion/release for the drag. It does not infer visual success: the
operator still confirms pointer selection, border movement, and text delivery
on the physical outputs.

<!-- END IMPORTED BODY -->
