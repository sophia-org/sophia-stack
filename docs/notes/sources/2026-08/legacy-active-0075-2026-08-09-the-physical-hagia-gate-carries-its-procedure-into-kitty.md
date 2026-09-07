---
id: legacy-active-0075
date: 2026-08-09
recorded_date: 2026-08-09
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy", "validation"]
---
# 2026-08-09: The physical Hagia gate carries its procedure into Kitty

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2439–2598. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The third authorized Hagia restart attempt ended cleanly but was not a policy
exercise. Its total physical ingress was exactly 34 events: press and release
for each of the 16 letters in `hagiapolicyproof`, plus Enter. It contained no
preceding chord events, no ordered policy actions, and no supervised restart.
That makes it a useful negative observation, not promotion evidence.

The proof launcher had printed the action sequence before transferring the
operator and device ownership to TTY4. The gate now starts Kitty with a bounded
POSIX-shell guide that watches the evidence file, displays one chord at a time,
and advances only after Sophia commits that action. It reveals the final phrase
only after the supervised restart and complete post-restart sequence. This
changes only the human proof harness; action admission, fault injection, and
evidence validation remain the same. A replacement authorized run is still
required.

The first guided attempt immediately exposed a separate verifier-ordering
error. Its initial Super press reached physical ingress and mapped to X keycode
133 before the bound `Y` press could be consumed as an Engine-owned action; the
exact application-text matcher treated that non-text modifier as the first
letter and aborted. Physical text verification now excludes Control, Shift,
Alt, and Super transitions from its text-producing sequence. Their application
delivery semantics are unchanged, and any unmatched non-modifier key still
produces an exact mismatch.

The next guided run crossed the intended failure boundary for the first time:
fullscreen and active-output actions committed before the injected fault;
epoch 2 loaded and reconciled the nonempty checkpoint and requested the
generation-2 refresh; and post-restart fullscreen plus both maximize actions
committed. The following minimize action correctly hid Kitty, but that also hid
the guide's later restore instruction. The run timed out without application
text rather than guessing past the missing prompt. The guide now presents
minimize and its immediately required restore chord together before waiting for
either outcome.

A follow-up again stopped at the committed minimize boundary. The twelve
client-routed modifier transitions account exactly for the six committed
chords, but terminal evidence could not distinguish an absent restore chord
from a recognized restore awaiting policy settlement. The owner now records
physical action admission immediately, before its ordered policy request
settles. The guide's minimize screen also leads with a three-line instruction
to release `Super+N` and then press and release `Super+R` while Kitty is hidden,
without waiting for another prompt.

The next attempt initially appeared to exhaust the 120-second global ceiling,
but timestamp review proved the operator's objection correct: only 15.5 seconds
elapsed. A separate hard-coded physical-sequence timeout had fired after the
second maximize action. That 15-second fail-fast default remains appropriate
for existing short physical proofs, so it is now an explicit bounded session
option rather than being weakened globally. The Hagia guide requests ten
minutes inside an eleven-minute global ceiling. This adds no soak requirement:
the run exits immediately after its exact proof and only bounds an abandoned
exclusive DRM/input session at a practical human timescale.

The next run reached `Super+N` and `Super+R` without a timeout. Both actions
were admitted and committed, proving the blank screen was not operator input
loss: the minimize projection contained zero render layers, and the following
restore projections also contained zero. Reduction found that Sophia built its
nominally complete public-policy snapshot from visible and planning layers
only. Once minimize removed the visible layer, Hagia's complete-snapshot
reconciliation correctly interpreted the absent surface as destroyed and
discarded its private minimized history. Sophia now retains authority-observed
facts independently from render visibility and includes every policy-owned
surface in the snapshot until an explicit withdrawal or removal. The follow-up
run showed why the frontend's `mapped` bit cannot define that lifetime: an
Engine-admitted X surface retains the client's pre-admission `mapped=false`
observation because admission is not a second client `MapWindow` request.
Sophia now uses the retained request/withdraw ownership record instead. A
regression covers the admitted, hidden, `mapped=false` record and its later
explicit withdrawal.
The physical verifier and in-Kitty guide now additionally require the first
post-restore checkpoint to remain nonempty, so a committed no-op restore cannot
be promoted. A replacement physical run is still required.

The first run with request/withdraw ownership failed closed during Kitty
startup, before physical interaction. Its diagnostic snapshot contained three
unassigned surfaces, and Hagia's resulting proposal reached a surface for
which Sophia had no X11 client route. Authority presentation observations can
describe transient hierarchy members without carrying a frontend client
identity, so retained presentation facts alone are broader than the public
policy capability set. Complete snapshots now admit only the intersection of
retained policy ownership and a live authority client route. That route is
created by the client request, survives minimize/restore, and is removed with
the actual surface. The regression includes an unrouted authority observation
and proves it cannot enter the snapshot. The real-Hagia nonexclusive restart
smoke then completed cleanly: its startup snapshot admitted exactly one routed
surface, epoch 2 loaded and reconciled the nonempty checkpoint, the refresh
cycle committed, and all session/layout/cleanup health checks passed.

The replacement physical run then crossed the architecture boundary that had
blocked every earlier attempt. The minimize request retained one snapshot
surface; the next request reported that same surface as minimized; restore
returned it to non-minimized state; and every checkpoint remained nonempty.
Both output actions, fullscreen, two maximize transitions, and the exact final
text were physically delivered. A final close race prevented promotion: after
the guide accepted the phrase and exited, Hagia returned a projection based on
the just-retired one-surface snapshot. The canonical reducer still held that
base generation, so Sophia attempted to materialize the dead placement before
advancing to the already-queued surface-removal cycle. Public response handling
now snapshots current Engine facts before reading any response placement. If
the scene advanced, it retires the outstanding request as `RejectedStale` and
only then permits a fresh complete cycle. Thus close-during-response cannot
name a missing planning surface or terminate the session.
The guide also remains alive after accepting the final phrase until Sophia's
exact proof ends the session, removing the harness-created close race while
retaining the Engine-side stale-response defense for real client exits.

The next replacement attempt showed both requested `Super+Right` actions as
admitted and committed, but the guide remained at its restart wait because no
restart occurred. The injected fault was still armed on the sixth global
checkpoint. Extra valid startup and stale-response settlement cycles changed
that count, so checkpoint ordinals were not causally tied to the physical
procedure. The gate now arms an evidence watcher for the ordered fullscreen
and first active-output commits, then kills Hagia only after the next nonempty
checkpoint. The wrapper starts the watcher before `exec`-replacing itself with
Hagia, preserving the exact supervised PID authorized by the private policy
endpoint. A deterministic process test proved that an earlier checkpoint does
not trigger the kill, the first following checkpoint does, and an epoch-2
invocation with the marker present runs without a second injection. Another
physical run remains required.

That replacement run proved the complete policy path: the restart occurred
immediately after the checkpoint following the first active-output action;
epoch 2 loaded and reconciled its nonempty candidate and issued the
generation-2 refresh; fullscreen, both maximize transitions, minimize/restore,
and both output actions committed; and the restored checkpoint remained
nonempty. The operator then entered the exact phrase, all 52 action-plus-text
key transitions flushed to the X frontend, and native presentation remained
active. The run did not promote because Sophia timed out waiting for the
semantic result file. Command construction had appended the stock proof-result
writer after Kitty's custom guide program, making it unused guide arguments
rather than an executed command. Sophia now passes the owner-only result path
explicitly in the guide's environment, and the guide writes the exact bounded
line only after reading it. An isolated replay over the completed evidence
proves that witness path. One final physical run is required for the combined
semantic, pixel, presentation, health, and cleanup verdict.

The final authorized run passed that combined verdict. The causal injector
restarted Hagia immediately after the nonempty checkpoint following the first
active-output commit. Epoch 2 loaded and reconciled the candidate, requested
its generation-2 two-output refresh, and preserved fullscreen state. The
post-restart sequence committed fullscreen, maximize twice, minimize, restore,
left output, and right output, with a nonempty checkpoint after restore. The
guide recorded the exact 16-byte line it read; Sophia matched all 34 text
press/release events, flushed all 52 action-plus-text X11 transitions, observed
changed terminal pixels, and correlated the final libinput ingress to a kernel
page flip in 24 ms. The bounded session ended after 24.994 seconds with one WM
restart, 13 committed WM cycles, no degraded state, no pending WM/action/input
work, no unexpected protocol errors, drained native ownership, clean topology,
and clean process/namespace/Xauthority teardown. This is the first passing
installed Hagia physical policy/restart evidence and closes that promotion
boundary; it is not a universal cadence or latency guarantee.

The foundation-era revalidation at Sophia commit `9b3750e4` and Hagia commit
`cbb629a` passed the same installed gate as immutable promotion run `0002`.
That record independently verifies the unified compiled profile path,
Chronicles checkpoint evidence, epoch-two restoration, all required physical
actions, exact semantic input, checksums, and clean bounded teardown.

<!-- END IMPORTED BODY -->
