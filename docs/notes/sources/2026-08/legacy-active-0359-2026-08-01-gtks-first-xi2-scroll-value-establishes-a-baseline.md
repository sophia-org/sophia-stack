---
id: legacy-active-0359
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-08-01: GTK's first XI2 scroll value establishes a baseline

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11069–11093. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first physical run after moving scroll valuators to axes 2 and 3 still
  stopped at Firefox step 4. Its finished session log contains exactly one
  `axis_observed`/`axis_routed` packet between PRIMARY and shutdown, proving
  that the corrected wire topology was exercised but could not yet produce a
  nonzero DOM scroll delta.
- GTK's XI2 device path intentionally records the first value received for a
  scroll valuator and returns a zero delta. Only a later absolute valuator
  value can be differenced against that baseline. The QEMU harness previously
  retried as many as ten individual wheel clicks until the page advanced, so a
  second retry silently satisfied this requirement without recording it in the
  evidence contract.
- XI2 2.1 also requires legacy button emulation when a device exposes scroll
  valuators. Sophia already emitted a core Button4-Button7 record, but did not
  emit the corresponding XI2 ButtonPress/Release event. Axis routes now resolve
  smooth-motion and button selections independently and write the button
  detail.
- Relative scroll valuators now advertise unknown bounds as zero/zero. The
  local page tells the operator to scroll through at least two notches and
  displays an explicit baseline message if it observes a zero-delta wheel
  event. QEMU sends exactly two clicks, and both automated and physical gates
  require two new routed-axis records before accepting DOM scroll completion.
  The Milestone 10 item remains open for a fresh physical proof.

<!-- END IMPORTED BODY -->
