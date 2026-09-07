---
id: legacy-active-0602
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-09-04: input wakeups cannot own content cadence or cursor styling

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19041–19081. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The zero-row `cp14-schema4-d0b10a2c` attempt remains preserved as a partial
launch diagnostic. During follow-up native observation, the visibly changing
Kitty stream appeared to advance differently while the mouse moved, and the
cursor did not match XLibre's default. The workload producer itself remained
independent: it flushed every 16 ms. The product defect was at the owner
boundary. Authority batches were eligible to compose immediately, while input,
authority, and native-service turns shared the same loop, so wake and batching
patterns could change which intermediate generations reached a primary frame.

Engine now owns one monotonic `PrimaryFramePacer`. It derives its interval from
the active primary-head refresh, rephases after a refresh change, admits the
first production state, coalesces later busy states latest-wins, and forces the
retained state at its bounded deadline. The timeout is folded into the owner
wait and can preempt continuous authority traffic. Physical input does not call
the reducer: atomic cursor updates remain immediately serviceable, so the fix
does not trade pointer latency for stable content cadence. Session evidence
separately reports cadence-deferred batches, deadline repaints, and the interval.
Deterministic tests feed identical content schedules with zero and many
simulated input wakeups and require the same deadline.

Cursor appearance is now a validated data path rather than renderer policy.
Core configuration selects a bounded theme, nominal size, and semantic shape.
The trusted session resolves standard Xcursor inheritance once, accepts no
asset over 128×128 or four MiB on disk, chooses the closest nominal size and
first static frame deterministically, and emits hotspot, digest,
ignored-frame, and fallback evidence. The immutable premultiplied asset is
shared by CPU composition and each KMS card group. Both software and hardware
placement subtract its hotspot. The fallback is reconstructed from the
public-domain Xorg `cursor-misc` `cursor.bdf` source/mask glyphs 68/69, yielding
the 10×16, hotspot-(1,1) cursor produced by XLibre's core `left_ptr`.

The desktop comparison no longer discovers Sophia's personal core config. Its
repository profile selects that fallback, its prepared manifest binds the
asset digest, and the gate verifies Sophia's startup attestation. The typed
conformance owner serializes the same Engine pixels into an owner-only standard
Xcursor theme for niri; XLibre explicitly selects its matching core cursor.
This closes the deterministic implementation gap. A fresh signed run and the
first Sophia/XLibre/niri rows remain the physical proof.

<!-- END IMPORTED BODY -->
