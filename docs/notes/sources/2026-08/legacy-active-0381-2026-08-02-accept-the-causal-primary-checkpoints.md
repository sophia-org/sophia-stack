---
id: legacy-active-0381
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-02: accept the causal PRIMARY checkpoints

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11600–11620. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The instrumented physical run flushed Firefox's selection request and
  synthetic property-bearing notification to Kitty, which read the exact
  22-byte source token and published its receipt checkpoint. After Kitty took
  PRIMARY ownership, Firefox negotiated targets and received a flushed
  synthetic UTF-8 notification; the page immediately published its exact-token
  confirmation. This closes both real-client directions without another
  operator replay.
- The session reducer nevertheless reported zero of four checkpoints because
  it waited first for a 250-byte initialization title. Firefox coalesced that
  property update before the global metadata observer saw it, while the
  trusted-selection 251-byte title, Kitty's 253-byte receipt, and Firefox's
  252-byte confirmation were all observed in causal order. Page initialization
  is redundant once a trusted full-field selection has armed the source.
- The focused proof now begins at `source_armed` and retains only the three
  causal checkpoints. Its verifier brackets each transfer with the matching
  owner change, conversion, and socket-flushed synthetic notification, so
  removing the unobservable initialization marker does not weaken selection
  authority coverage and avoids coupling the gate to property-update timing.

<!-- END IMPORTED BODY -->
