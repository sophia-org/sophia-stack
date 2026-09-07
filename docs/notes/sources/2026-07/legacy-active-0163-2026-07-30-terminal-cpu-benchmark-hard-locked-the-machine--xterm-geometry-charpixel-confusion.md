---
id: legacy-active-0163
date: 2026-07-30
recorded_date: 2026-07-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering", "validation"]
---
# 2026-07-30: Terminal CPU benchmark hard-locked the machine — xterm geometry char/pixel confusion

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5312–5429. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first physical run of the 9.4 terminal CPU-path benchmark
(`tools/benchmark_sophia_terminal_tty3.sh`) hard-locked the machine and
required a power reset. Recovered evidence from
`~/.local/state/sophia/standalone-session/` isolated two distinct problems.

- **Functional root cause (fixed).** `tools/probes/run_bounded_xterm.sh` passed
  the intended *pixel* size (`SOPHIA_XTERM_WIDTH/HEIGHT=500`) straight into
  xterm's `-geometry`, but xterm reads `-geometry` in *character cells*. With
  the default font that requested a 4004x5004 px window. `apply_text_draw`
  backs each CPU window with one immutable software buffer bounded by
  `X_AUTHORITY_SOFTWARE_BUFFER_MAX_BYTES` (64 MiB, `sophia-x-authority`
  `software.rs`); 4004x5004x4 ≈ 80 MB overran it, so `draw_text` returned
  `None`, the `ImageText8` was rejected `BadWindow`, xterm aborted (exit 83),
  and the session ended with `"live session ended without a committed external
  WM layout"` (exit 1). The X authority failed *closed* here — it refused the
  buffer rather than allocating it.
- **Deterministic offline reproduction.** Driving the committed
  `x-authority-xterm-input-smoke` at `500x500` reproduced the crash
  bit-identically (opcode 76 `X_ImageText8`, resource `0x200014`, serial `362`)
  with no KMS involved. `40x8`/`100x50`/`200x150` pass. The authority runs the
  same dispatch path offline, so this class of bug is debuggable without a
  physical takeover. Note the `sophia_terminal_performance_pass.log` fixture was
  aspirational: the benchmark had never had a real green run.
- **Fix.** The probe now converts the pixel intent to a character geometry
  against a pinned fixed-metric core font (`-fn 6x13 -b 2`) and clamps the pixel
  intent well under the cap. Default 500 → `82x38` cells → 496x498 px → 988 KB;
  the worst-case clamp (2048 px) stays at 16.7 MB. `SOPHIA_XTERM_WIDTH/HEIGHT`
  remain the reported pixel intent on the `sophia_terminal_benchmark` line.
- **Fail-closed compose budget.** The terminal performance report rejects
  `cpu_max_compose_msec` above 25 ms, matching the established
  CPU-composition gate used by the retained two-xterm and QEMU evidence. It
  records `cpu_compose_budget_msec` beside the observed maximum; malformed or
  zero overrides fail before evidence is accepted.
- **Commit-pinned physical runner.**
  `tools/run_sophia_terminal_gate_tty3.sh` refuses a dirty worktree, requires
  both persistent logging services and a nonempty kernel log before takeover,
  and archives the source commit, benchmark/report results, session/guard/TTY
  recovery, launcher handback, and the exact appended kernel-log bytes. A
  rotated log or new AMDGPU rejection/reset/timeout fails closed.
- **First post-geometry-fix physical result: bounded transport overload.** The
  run on commit `d7fbcff` passed offline preflight, armed the input guard,
  acquired both outputs, completed the synchronous output baseline, and then
  stopped the X frontend at transaction 650:
  `X authority observed transaction channel is full`. The probe's tight loop
  wrote 200 lines per iteration with no interval; a few bursts filled the
  intentional 256-batch frontend-to-owner queue before ordered visual facts
  could drain. X authority correctly refused to allocate or drop facts, xterm
  exited 84, native suspend drained, and TTY3/greetd handback completed. This
  is neither the prior geometry failure nor evidence for enlarging an
  eventually finite queue.
- **Paced workload decision.** The terminal probe now defaults to eight lines
  every 16 ms and carries both values through schema-2 benchmark/client records
  into the schema-3 performance report. The reporter rejects mismatched cadence
  or inconsistent line totals. Zero and over-one-second intervals are invalid,
  so an override cannot silently restore the unbounded producer. The gate
  runner also leaves a structured `interrupted` result and copies available
  session artifacts from its exit trap.
- **First paced physical results: rendering passed, controller completion
  fixed.** On commit `839d21a`, the first run was manually logged out at 14.3
  seconds after visually confirming the scrolling xterm and responsive pointer;
  it drained native scanout and restored TTY3/greetd cleanly but correctly had
  no 20-second client record. The next run was left for the full window:
  authority batches dropped `0`, unexpected protocol errors were `0`,
  `cpu_max_compose_msec=6`, native failures were `0`, the kernel delta was
  complete and clean, and handback was clean. It still failed the report
  because xterm backpressure held the producer inside `seq(1)` past its
  wall-clock loop test; the outer 25-second safety timeout ended xterm before
  the producer's final-only count write. The probe controller now runs the
  producer under an independent bounded timer and records each completed burst
  incrementally. A stalled-pty offline regression exercises that path. A real
  software-only Sophia session then emitted the client completion and cleaned
  up normally even though xterm lingered until its process safety timeout.
  The standalone launcher now explicitly tells bounded-xterm operators to let
  it exit automatically instead of showing the generic logout hint.
- **Native path is not the lock cause.** Audit: CPU-layer GL textures are
  reallocated to the incoming layer size (`sophia-renderer-native-egl` `gl.rs`),
  but that layer *is* the ≤64 MiB software buffer (≤ ~4096², inside RDNA3's
  16384 GL limit); DMA-BUF layers are client-bounded; scanout framebuffers are
  output-sized. In the crashed run the oversized CPU buffer was refused, so
  nothing oversized ever reached the GPU — only one output-sized blank frame
  presented.
- **The hard lock is downstream of the abnormal early exit at KMS handback.**
  There is no DRM/VT code in Rust; the launcher stops greetd/lightdm, Sophia
  becomes DRM master implicitly, and on exit drops master by closing the fd
  while the launcher restarts the display manager. Teardown
  (`detach_native_scanout`, `production_visual_runtime/native.rs`) drains
  in-flight scanouts, rejects pending presents, and rebuilds the output set
  without scanout — but it does **not** disable the CRTC or restore the prior
  mode. Sophia leaves its last framebuffer active on the CRTC and drops master;
  greetd/Xorg then re-modesets the RX 7900 GRE from that state. This teardown
  path is identical for normal exits, which hand back cleanly, so the trigger
  was the *early* abnormal exit (session aborted right after the first blank
  frame, before the steady-state page-flip loop) hitting the RDNA3 re-take in a
  fragile transient state.
- **Decision.** The probe fix removes the trigger; the benchmark should now
  reach steady state and hand back like every other clean run. Optional
  hardening for the handback (deferred to a validated physical change): issue an
  explicit CRTC-disable / mode-restore atomic commit before dropping master.
- **Environment (not in-repo).** Host is Void Linux (runit, no journald), so
  the prior-boot kernel dmesg was lost on reset. Persistent logging is now
  enabled via `socklog-void` (`/var/log/socklog/kernel/current`); setup helper
  is `~/sophia-amdgpu-logging-setup.sh`. GPU: Radeon RX 7900 GRE (Navi 31,
  RDNA3, `03:00.0`) plus a Raphael iGPU (`16:00.0`); `amdgpu` `gpu_recovery`,
  `lockup_timeout`, `reset_method`, `runpm` are all auto (`-1`).
- **Controller-fixed physical gate passed.** Two commit-pinned runs on
  `4cb4f5f` completed the 20-second producer and emitted passing schema-3
  reports. Both recorded 6,648 lines / 831 completed iterations, positive
  immutable CPU patch traffic, damage-driven partial repaint, zero authority
  drops, zero unexpected protocol/native failures, clean kernel deltas, and
  clean TTY3/greetd handback. Maximum CPU composition was 7 ms against the
  25 ms budget. The runner archives retained `visual-confirmed=false` because
  the local prompt did not record `yes`; the operator had separately observed
  the expected scrolling-number surface and responsive pointer in the paced
  session. That prompt metadata is not rewritten. The named automated
  acceptance criteria are complete; no Xserver parity claim is made.

<!-- END IMPORTED BODY -->
