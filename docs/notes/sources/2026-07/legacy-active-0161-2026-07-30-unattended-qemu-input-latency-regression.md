---
id: legacy-active-0161
date: 2026-07-30
recorded_date: 2026-07-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-07-30: Unattended QEMU input-latency regression

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5247–5276. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- **Promotion coverage.** The commit-pinned Milestone 9 semantic gate now
  archives and verifies this isolated regression after its M7 and M8 scenarios.
  A candidate cannot pass gate zero with only application semantics while
  omitting the current libinput/page-flip correlation contract.
- **Local gate hygiene.** The promotion rerun exposed a stale generic-QEMU
  pass fixture and two unreviewed source-layout overages before virtualization
  began. The fixture now carries the latency and kernel-clock records, session
  configuration tests have their own domain file, and layout support is split
  from the persistent layout owner. The complete local gate passes again.
- **Host uinput is no longer required for development validation.** The QEMU
  session injects QMP keys through the guest virtio keyboard, evdev, and the
  normal threaded libinput poller. `tools/run_sophia_input_latency_qemu.sh`
  rebuilds the guest by default and retains commit-pinned evidence under the
  user's state directory.
- **No-WM admission deadlock fixed.** Presentation-intent quarantine is an
  external-WM ownership boundary. Proof sessions without a WM now commit
  policy-managed X11 pixels directly instead of waiting forever for an absent
  policy process to admit them.
- **Software scanout is correlated directly.** CPU-composed xterm frames do
  not create a GPU Present retirement record. The native head now retains the
  accepted kernel page-flip UST as well as its submission UST, allowing the
  input proof to select the changed post-ingress software frame exactly.
- **Retained result.** The isolated two-output guest reached startup readiness
  in 74 ms, routed and flushed all 14 keyboard events, matched `sophia`, proved
  the pointer path, and reported a 6 ms full chain with 8 kernel timestamps,
  zero fallbacks, and zero pending correlations. QEMU validates the clock and
  correlation plumbing; it does not replace the physical 20-sample p95 gate.

<!-- END IMPORTED BODY -->
