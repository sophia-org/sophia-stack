---
id: legacy-active-0587
date: 2026-09-02
recorded_date: 2026-09-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-09-02: a ready workload is not necessarily a visible workload

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18556–18604. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Physical run `~/.local/state/sophia/desktop-comparison/cp14` on signed Sophia
candidate `00deb7885a848434932677b011fe9b1649694a89` mechanically sealed and
replayed the first 15 schedule rows: all nine changing-Kitty samples and the
first two Firefox repetitions for Sophia, XLibre+xmonad, and niri. Every row
reported its required 60 resource samples, kernel timing population, zero crash
or sample loss, and clean teardown. Status then named row 16, niri Firefox
repetition 3, as next.

The operator did not see Firefox during Sophia row 15. The sealed evidence
nevertheless reported the loopback readiness beacon after 1.004 seconds, held
24 measured processes for 60.039 seconds, accumulated 29.080 CPU-seconds, and
shut down cleanly. That disagreement exposed an acquisition error rather than
a Firefox launch failure: the readiness beacon proves that the page executed,
not that its surface reached the visible projection.

The Sophia comparison path starts the ordinary Hagia session with
`sophia_append_session_terminal_base_args`, which registers Kitty and makes it
the startup application. The operator then invokes the capture owner from that
Kitty. `WorkloadOwner` launches a second Kitty or Firefox process, while the
resource census includes both the attested Sophia supervisor tree and the
workload roots. The launcher Kitty therefore remains in Sophia's measured
desktop population; neither reference launcher has an equivalent client.

The same path explains the missing window. Hagia's `addWindow` transition adds
a new opaque window but does not implicitly focus it. Snapshot reconciliation
then preserves Engine's existing focus on the launcher terminal, and the
single-view scroller keeps that focused column in the viewport. The workload
can remain off-screen while its client renders and the selected CRTC continues
to deliver DRM vblank events. The current capture contract has no passive fact
that binds the owned workload to a visible DP-1 placement, so replay cannot
distinguish that state from a valid comparison row.

This invalidates Sophia rows 1, 6, 8, 10, and 15 as apples-to-apples workload
samples. The complete 15-row prefix must not be promoted: its reference rows
remain useful local diagnostics, but the run ledger and immutable manifest bind
them to an acquisition whose Sophia population and visible load differ. The run
is paused in place as diagnostic evidence; no row 16 capture is admitted.

The replacement acquisition must start Sophia without a launcher client and
run its capture controller outside the measured supervisor tree, matching the
reference launchers' ownership shape. Before the measured clock begins, a
trusted evidence boundary must passively prove that the capture-owned workload
has a visible placement on DP-1. That observation must not disclose titles,
classes, PIDs, or other application metadata to the blind WM. A regression must
make a ready but hidden workload fail. Only a new clean signed candidate and a
fresh prepared 39-row run can close CP-14.2.

<!-- END IMPORTED BODY -->
