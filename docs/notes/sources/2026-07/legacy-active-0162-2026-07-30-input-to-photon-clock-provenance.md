---
id: legacy-active-0162
date: 2026-07-30
recorded_date: 2026-07-30
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-30: Input-to-photon clock provenance

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 5277–5311. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- **Kernel presentation timestamp preserved.** The native DRM event adapter now
  retains `PageFlipEvent::duration` as microseconds on its private callback,
  carries it beside the public callback through bounded polling, and correlates
  it by output/frame serial before presentation retirement. Production
  retirement therefore uses the DRM kernel's monotonic page-flip UST instead of
  `presentation_started.elapsed()`.
- **Fallback is observable, not silent.** Synthetic presentation timestamps
  remain available for fake and non-kernel callback sources, but production
  completion now emits `sophia_live_page_flip_clock` with kernel timestamp,
  fallback, and pending counts. The physical latency gate must require positive
  kernel timestamps with zero fallbacks and zero pending correlations.
- **Raw ingress is now separately injectable.**
  `tools/probes/uinput_text_injector.py` creates a bounded Linux virtual
  keyboard and publishes its event node so the session opens it through the
  same threaded libinput path as hardware. The poller retains a private
  per-event timing sidecar (serial, kernel event time, queue dwell); protocol
  packets remain passive and unchanged. `--inject-text` remains synthetic and
  is never counted as this proof.
- **Exact-frame correlation and gate.** The physical proof anchors on the last
  routed key press, waits for X delivery, requires a changed output frame
  submitted after that raw ingress, and computes its retirement from the
  matching kernel page-flip UST. GPU Present surfaces retain the stable-surface
  proof, while software-composed frames correlate on the native output
  submission/page-flip pair. Completion reports queue dwell,
  dwell-to-submit, submit-to-page-flip, and full-chain latency.
  `tools/run_sophia_input_latency_tty3.sh` collects 20 independent
  commit-pinned uinput/libinput samples, rejects any fallback/pending page-flip
  timestamps, and requires full-chain p95 below the configured refresh period.
  `tools/setup_sophia_uinput.sh` installs the persistent Void/udev module,
  device-node, and `input`-group policy required by that unprivileged runner.
  The code path and injector ABI self-test pass offline; physical p95 evidence
  is still required before closing the todo item.

<!-- END IMPORTED BODY -->
