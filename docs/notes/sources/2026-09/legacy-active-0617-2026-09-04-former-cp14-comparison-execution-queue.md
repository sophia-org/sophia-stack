---
id: legacy-active-0617
date: 2026-09-04
recorded_date: 2026-09-04
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-09-04: former CP14 comparison execution queue

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 19614–19831. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The following is the superseded execution queue retained from `todo.md` before
Milestone 14 was retargeted to development-session readiness. Its checkbox and
status labels are historical, not instructions to resume the comparison. The
current decision follows this record. Failed attempts remain failures; this
archive records no new promotion.

<details>
<summary>Former CP-14.2 and CP-14.3 queue</summary>

### CP-14.2 — Same-hardware comparison (`NOW`)

- [ ] Run identical Kitty, Firefox, resize, and launch-burst workloads against
  Sophia, XLibre+xmonad, and a mature Wayland compositor on the same hardware.
  Run the separate Sophia two-hour soak only when overnight durability evidence
  is useful; it is optional and non-blocking.

Required exit:

- pin executable/configuration identities, topology, refresh, workload, sample
  windows, and raw evidence;
- report resource, frame-time, latency, allocation, and failure populations
  without converting reference results into Sophia correctness thresholds; and
- classify the comparison as diagnostic. Sophia's absolute correctness,
  authority, and refresh-relative latency gates remain authoritative.

The acquisition contract is implemented under
`cargo xtask conformance desktop-comparison`. A clean signed preparation
detects host identity and hashes the descriptors, isolated profiles, Firefox
fixture/profile, tracefs adapter, all stack-launch adapters, and the six stack,
policy, and shell executables. `gate` is the single TTY3 row owner: it
revalidates the clean prepared checkout and release
build before takeover, chooses the typed next stack, launches no operator
application, keeps the controller outside the measured supervisor tree,
resolves DP-1's active CRTC, and owns capture plus teardown. `attest`,
`preflight`, `qualify`, `capture`, `finalize`, `replay`, `verify`, and `report`
remain separately callable diagnostics.

Each attempt retains exact raw visibility, split resource, deduplicated
kernel-frame, workload, native-timing, and post-teardown attempt records plus a
derived schema-4 result and internal ledger. The first Sophia row also requires
an excluded four-target physical cursor qualification. Replay requires an empty
application baseline; a capture-owned, focused, visible DP-1 toplevel with zero
foreign application toplevels at settlement and every sample; uniform
60-second short windows; 120 resize observations; contiguous/monotonic
populations; zero crash/loss; and clean teardown. The optional soak lane
independently requires a full two-hour sample.
Correlation consumes PID/start identity only inside trusted conformance code
and persists no application identity. Partial attempts block progress only
within their own run. Regression coverage includes ready-but-hidden and
foreign-window rejection, legacy-run refusal, raw replay/archive integrity,
matrix/order mutation, kernel normalization, owner-only modes, tracefs probe
records, isolated Kitty configuration, and bounded runtime socket paths.
Reports retain resource, allocation, latency, and frame distributions with
`verdict=none`.

The first physical attempt failed closed before row 1 and is documented in
`docs/research-log.md`. A later run on signed candidate `00deb788` sealed the
first 15 rows, but the operator did not see Firefox during Sophia row 15.
Investigation found that the Sophia launcher keeps the Kitty used to invoke
capture inside the measured supervisor tree, while Hagia preserves focus on
that terminal and can leave the owned workload off-screen. Reference sessions
have no equivalent client. Readiness and DRM-vblank evidence do not prove a
visible workload, so Sophia rows 1, 6, 8, 10, and 15 are biased and the complete
prefix is non-promotable. Acquisition is paused before row 16.

Implementation and recovery hardening are complete; remaining critical-path
work:

- [x] replace Sophia's operator-terminal acquisition with a terminal-free
  session and a capture controller outside the measured supervisor tree;
- [x] fail closed unless trusted passive observation binds the capture-owned
  workload to focused, visible DP-1 placement without disclosing application
  identity to the blind WM, with hidden/foreign negative regressions;
- [x] commit and sign the corrected candidate, then prepare a fresh run using
  the already provisioned pinned XLibre prefix and isolated reference profiles;
- [x] accept Sophia's connector-neutral RandR names and harden teardown so a
  greeter is activated only after both the origin and manager TTY input states
  are restored and verified, with a text-TTY fallback and persistent handoff
  record;
- [x] reproduce the schema-4 failure with the isolated real `xrandr` client,
  implement the advertised read-only `GetPanning`, `GetCrtcTransform`, and
  `GetCrtcGamma` requests, and make X protocol errors terminate topology
  admission immediately with preserved diagnostics;
- [x] make greetd recovery attributable and layered: verify exact captured state
  before restart, fall back to a verified safe text-console baseline if exact
  kernel round-tripping diverges, then require stable text display, a
  non-disabled keyboard mode, readable termios, and a live tuigreet on the
  configured VT before activation;
- [x] sign the protocol and recovery corrections as `d5a1f7da` and prepare
  `cp14-schema4-randr` against that exact candidate;
- [x] stop after its first Sophia-row attempt and inspect the zero-row result:
  topology and attestation passed, recovery safely established greetd on tty7,
  and capture aborted before creating an attempt because Firefox had upgraded
  from the pinned 154 to 155;
- [x] update the Firefox comparison pin and move exact Kitty, Firefox, and niri
  version admission into both preparation and the pre-takeover gate, retaining
  capture-time revalidation against upgrades during a run;
- [x] sign the version-admission correction, prepare `cp14-schema4-tools`, and
  stop after the first Sophia and XLibre rows for inspection;
- [x] diagnose the two-row discrepancy: the atomic cursor accepted pending
  positions without a guaranteed post-retirement commit, XMonad self-replaced
  through a missing isolated cache executable, duplicated DRM deliveries
  inflated X timing, and capture claimed clean teardown before teardown ran;
- [x] implement a topology-wide latest-wins atomic cursor owner with idle
  cursor-only progress, combined-commit retry, hard-rejection legacy fallback,
  bounded counters, and truthful queued-versus-visible reporting;
- [x] make comparison capture stage before teardown, finalize only after the
  exact supervisor exits, keep required component identities live throughout
  sampling, split stack/workload/aggregate resources, deduplicate kernel
  sequences, preserve nested gate diagnostics, and add the excluded cursor
  qualification;
- [x] land the cursor and evidence corrections in a clean signed candidate;
- [x] make the prepared comparison root and its identity/checksum records
  owner-only independent of umask, and reject later ownership or mode drift;
- [x] stop after the first owner-only Sophia attempt and diagnose its bounded
  runtime failure: candidate topology reconstruction retained the fixed
  connector, CRTC, and primary plane but discarded the discovered cursor
  plane, invalidating the already-selected atomic cursor path after commit;
- [x] preserve the cursor plane across output-policy candidate and rollback
  selections and cover that KMS-route invariant through the public topology
  planner;
- [x] stop after the next zero-row attempt and diagnose its bounded failure:
  cursor qualification proved two-head atomic motion, then withdrawing its
  final window left committed focus naming a surface omitted from the next
  complete public snapshot;
- [x] sanitize snapshot focus from the same live surface set and reject stale,
  cross-output, non-focusable, or minimized focus at both the protocol codec
  and Engine authority boundaries;
- [x] decouple primary content cadence from input turns with one
  refresh-relative latest-wins deadline and cover still-versus-moving input
  schedules deterministically;
- [x] replace the renderer-private cursor bitmap with one bounded immutable
  Engine asset, configurable standard Xcursor lookup, validated hotspot and
  static-frame handling, and the canonical X11 core `left_ptr` fallback;
- [x] pin the comparison's Sophia core profile and canonical cursor digest,
  materialize the same pixels as an owner-only Xcursor theme for niri, and
  select XLibre's matching core cursor without reading personal configuration;
- [x] diagnose the remaining pointer/cadence coupling: DMA-BUF Present used
  global request transaction IDs as its MSC, while cursor-only atomic commits
  could block ahead of ready Present feedback;
- [x] route physical KMS `(ust, msc)` through GPU and software Present
  completion, make transaction IDs correlation-only, give primary submission
  and feedback priority over cursor-only DRM service, preserve a superseding
  cursor cell, and keep hardware-cursor pixels out of native CPU repaints;
- [x] inspect the resulting signed physical Sophia attempt: the workload stayed
  focused and visible for 60/60 samples with 3,600 contiguous 60 Hz kernel
  frames, but late re-reading of volatile cursor qualification prevented
  `measurement.kdl` and retained the row as a partial diagnostic;
- [x] admit and snapshot live-session qualification before creating a partial
  or starting the timed workload, and preserve each nested conformance result
  in the durable TTY gate log;
- [x] reproduce the missing-qualification path without consuming a capture:
  the window mapped and routed pointer motion, but timed out at 0/4 targets;
  make the shell helper return explicitly after failed attestation or
  qualification even when its caller condition disables implicit `errexit`;
- [x] prepare a fresh interactive run and inspect its physical qualification
  and initial rows. Candidate `124ad6c1` sealed all nine Kitty rows with clean
  teardown; Sophia Firefox row 10 then exposed stale admission recovery and
  remained partial;
- [x] sign the admission-recovery correction and run one short physical Sophia
  Firefox canary: admission rebased to 1266x1408, Firefox reached page-ready,
  and several full-size frames retired before a distinct software-Present timing
  failure. Subsequent pixel inspection found black browser content; this was
  not a Firefox visual pass;
- [x] require a fresh native retirement for every software Present even when
  the retained scene checksum is unchanged, and retain failed staging work in
  the teardown-visible ownership queue;
- [x] preserve the selected DRI3 source when CPU backing also exists; resolve
  CPU/SHM/GPU child Presents through the same presentation root; validate core
  image formats, byte order, payloads, and GC operations before pixel mutation;
- [x] require changing nonblack browser regions joined through head scene and
  exact native frame retirement; keep pixel scans opt-in, preserve explicit
  trace modes, drain connected clients without cancelling accepted work, and
  express output focus through the layout label instead of a blue square;
- [x] inspect `48bf357f`'s visible Firefox canary without promoting its failed
  teardown; reproduce and fix EOF suppressing already-buffered authority work,
  preserve bounded native service, and require coordinator settlement before
  successful shutdown;
- [x] pass one short physical Sophia Firefox canary: `2823807e`, clean logout
  in 43 ms, zero pending authority/coordinator/CPU/native work, and zero
  remaining application groups or frontend workers; fix both reader assumptions
  against the retained capture without another physical run;
- [x] prepare `cp14-schema4-251d9acd` in `.artifacts/desktop-comparison/`
  from a detached checkout of signed `251d9acd`, with the canary's exact
  Hagia/Narthex binaries; nine Kitty rows sealed and checksum-verified;
- [ ] fix session-wide native evidence across scanout replacement and bounded
  shutdown while the seat is suspended. Sophia Firefox row 10 measured 60 seconds
  but failed final validation after VT resume reset native retirement counters;
  its partial capture blocks this run. Retain it as diagnostic evidence, prove
  the correction with a short Firefox/VT/deadline canary, then prepare a fresh
  pinned matrix if runtime code changes;
- [ ] run the unified one-row TTY3 gate for all 36 required rows on this
  machine; and
- [ ] retain and verify the complete interactive matrix. A separate one-row
  Sophia two-hour soak remains optional overnight evidence and does not block
  this gate or CP-14.3.

### CP-14.3 — Close Milestone 14 (`NEXT`)

- [ ] Verify the milestone exit, archive the concise result in
  `docs/roadmap-history.md`, and update every affected current/target statement.

Milestone 14 exits only with bounded warmed resource counts, no steady-state
allocation growth, refresh-relative latency evidence, clean normal teardown,
and no change to Sophia's native-X authority model.

The current-soak verifier remains available for optional overnight durability
evidence. It requires a nonsaturated five-second resource series, at least 120
contiguous samples, and flat settled peaks with zero tolerance for accounted
resources; the native sampler holds 1,560 samples, covering two hours plus ten
minutes without saturation. Historical installed archives explicitly use the
archive policy and remain reproducible. A fresh two-hour run is useful but does
not block Milestone 14 closure.

</details>

<!-- END IMPORTED BODY -->
