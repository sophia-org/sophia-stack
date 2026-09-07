---
id: legacy-active-0544
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-28: the text proof refused a NumLock the user asked for

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16842–16866. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical latency run after the readiness fix armed, injected, and
then failed its very first event: `expected keycode=39 pressed=true state=0
observed keycode=39 pressed=true state=16`. Sixteen is Mod2 -- NumLock. The
user's desktop configuration says `numlock #true`, the session honors it by
seeding the core keyboard mapper with the latch, and every routed key
truthfully carries Mod2. `PhysicalTextProof` hardcoded state zero for every
press and demanded zero on every release, so a correctly routed keystroke in a
correctly configured session was refused as a routing error.

No earlier gate could see this. The Hagia promotion gates run the tracked
generic profile, which sets no NumLock; the QEMU guest has no user
configuration at all. The latency harness is the first physical text proof to
run under the operator's own desktop configuration.

The proof now tolerates exactly Mod2, on presses and releases both, because
NumLock changes the interpretation of none of the keys the proof can expect --
lowercase letters and Return read the same either way. CapsLock stays a
mismatch: a latched Lock would deliver uppercase and falsify the text the
proof claims to have typed. Held modifiers stay mismatches with or without
NumLock beside them; transient state is exactly what a routing proof exists to
refuse. Three mutations pin the boundary: removing the tolerance, widening it
to everything, and dropping the state comparison each fail the regression.

<!-- END IMPORTED BODY -->
