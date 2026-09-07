---
id: todo-cutover-product-state
date: 2026-09-06
date_basis: cutover-snapshot
kind: source
status: historical
tags: [historical, milestone]
---
# Product state recorded at the todo.txt cutover

These are the old roadmap's claims at cutover, including known stale narrative.
They do not override current contracts, code, task state, or later evidence.


- Milestones 9 through 12 are archived compatibility evidence. Milestone 13's
  installed native policy path is complete.
- Hagia is the ordinary installed native session. The retained Triad behavior
  port is complete, `sophia_wm_v1` interface major 1 / wire revision 3 is
  frozen, and Engine-owned API v7 workspace policy is gone.
- Hagia and Narthex speak the Sophia WM and shell roles directly. Sophia ships
  no legacy-X11-WM bridge or compatibility policy profile; existing WMs must be
  ported to the language-neutral protocols.
- The common native protocol-family contract is ratified. The role-by-role
  lifecycle audit and one family-level conformance entry point are not complete.
- `sophia_shell_v1` revision 4 is experimental, with revision-1–3 clients still
  supported. Narthex implements switcher, bounded reservation, persistent tabs,
  reference sheets, and the application launcher; Sophia owns geometry,
  rendering, input, and presentation. Content capability remains unimplemented.
  The family audit, stabilization, and current signed physical tab acceptance
  remain open. Hagia's native trees and tab protocol pass offline verification.
- Milestone 14 has retained physical evidence for frame slots, buffer-age damage,
  input latency, shared rendering, direct scanout, cursor motion, and continuous
  software-content presentation. Candidate `2823807e` passed the short Firefox
  canary with changing nonblack pixels, exact retirement, and clean logout.
- Comparison run `cp14-schema4-251d9acd` sealed nine Kitty rows; Firefox row 10
  measured successfully but failed final validation after VT resume reset native
  counters. Suspension also delays the runtime deadline. These lifecycle gaps
  are the first task in CP-14.3; the comparison remains deferred at 9/36. See the
  [diagnosis](legacy-active-0616-2026-09-04-cp14-row-10-exposes-scanout-evidence-lifetime-and-suspended-deadline-gaps.md).
- The current target is a recoverable Hagia development session, evaluated by
  real workflows and targeted checks. Milestone 14 is not complete; neither a
  new comparison matrix nor a clean-day timer is a prerequisite for this work.

Latest retained Milestone 14 evidence:

| Slice | Retained result |
| --- | --- |
| Frame slots | signed native archive `0001` |
| Buffer-age damage | signed native archive `0002` |
| Input latency | physical run `20260828T231430Z`: p99 24 ms / 34 ms budget over 245 presses |
| Shared renderer | signed native archive `0003`: one worker, zero misroutes |
| Direct scanout | archives `0001`–`0003`: eligibility, effect fallback, and same-session cost |
| Cursor | archives `0004`–`0006` plus continuous shakedown: 57.97 fps, p95 16.687 ms |
| Stable X backing | physical terminal run: 63/64 patches, 2 COW splits, registry peak 1 buffer |
| CPU continuity | signed run `20260902T002500Z` on `b9f0735a`: 7,116 accepted updates accounted, 1,190 presented, 5,926 superseded, zero pending, 16.586 ms maximum source gap, 18.825 ms maximum display gap, and 31.737 ms maximum update-to-retirement latency |
| Comparison acquisition | `cp14-schema4-251d9acd` sealed 9/36 Kitty rows; Firefox row 10 measured 60 focused/visible samples and 3,600 kernel frames, then failed native completion after VT resume reset counters. The partial blocks its own run; CP-14.2 remains deferred and incomplete. |

Promotion does not imply default enablement. Damage-limited repaint is now the
default, with `SOPHIA_ENABLE_BUFFER_AGE_DAMAGE=0` as the opt-out; its
pixel-equivalence proof runs in `cargo xtask check`. Shared rendering and direct
scanout remain opt-in in ordinary sessions, each owing its own promotion
decision. The atomic cursor is preferred, with `--legacy-cursor` and the startup
probe preserving the ioctl fallback. Verify current code and packaged policy
before changing any product default.
