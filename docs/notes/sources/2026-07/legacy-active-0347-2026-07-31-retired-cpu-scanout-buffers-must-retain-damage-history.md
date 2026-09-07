---
id: legacy-active-0347
date: 2026-07-31
recorded_date: 2026-07-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-07-31: retired CPU scanout buffers must retain damage history

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10809–10830. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The current-baseline correction removed unrelated pre-input frames, but the
  next physical 20-sample archive still failed at 29 ms p95 and 32 ms maximum.
  Its two slowest frames spent 14 and 22 ms from input dwell to KMS submission;
  both recorded a 20 ms native upload while queue and kernel page-flip clocks
  remained healthy.
- The CPU scanout worker allocated and wrote a complete 3840-by-960 linear GBM
  buffer for every changed frame even though Engine already proved bounded
  output damage. A terminal redraw therefore copied the whole output and
  exposed allocator/write-tail latency on the physical AMD path.
- Retired CPU worker leases now enter a bounded three-buffer free pool with
  their checksum and immutable output-damage snapshot. Reuse computes
  conservative damage against the exact pixels already stored in that buffer,
  maps the linear BO, and copies only those clipped rows. Missing, empty, or
  invalid damage proof falls back to a full repaint, and a failed reusable map
  falls back to the existing allocate-and-write path.
- The focused QEMU KMS regression exercised repeated damage-only reuse,
  reducing maximum native upload to 1 ms while preserving exact input pixels,
  kernel page-flip correlation, zero renderer failures, and clean lease
  teardown. Physical TTY3 p95 remains the authoritative acceptance gate.

<!-- END IMPORTED BODY -->
