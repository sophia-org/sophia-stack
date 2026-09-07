---
id: legacy-active-0376
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "architecture"]
---
# 2026-08-02: use yserver's layered regression model for the Firefox modal

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11484–11516. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- yserver's validation is layered rather than centered on one application
  script: ordinary Rust unit tests cover protocol encoding and state/event
  reducers; `proptest` covers parsers, properties, resource IDs, and state
  invariants; ignored Vulkan integration tests use pixel oracles, dma-buf
  export/re-import, and a 10,000-iteration FD-leak loop; XTS5 and rendercheck
  provide external protocol/graphics coverage; real Firefox, Chromium, GTK,
  and desktop sessions remain focused dogfood and x11trace comparisons.
- Its Firefox and GTK dialog investigations use the live application to expose
  a symptom, then retain the cause in smaller protocol tests—for example
  Present ConfigureNotify, descendant visibility, XI grab ownership, and
  button-release routing. Sophia already retains the admission-size and
  standing-target causes in Engine/session regressions, so the physical modal
  test should prove only the remaining cross-layer seam instead of replaying
  clipboard, PRIMARY, navigation, resize, and focus work.
- The new dialog canary has one Firefox surface and three monotonic redacted
  checkpoints: ready page, visible DOM modal, and confirmed modal. Both trusted
  clicks publish their own routed pointer batches. The verifier requires a
  complete 1276-by-1422 native retirement after the page checkpoint, after the
  modal checkpoint, and after confirmation. Because HTML `<dialog>` is not an
  X11 toplevel, any new frontend admission, stable-era restart/timeout, new
  recovery extent, incomplete clip, or GDK freeze is a hard failure. Existing
  unrelated surfaces and extra post-proof input do not invalidate the causal
  dialog result. Genuine transient X11 dialogs remain covered by the separate
  floating-policy wire tests.
- The 2026-08-02 physical gate passed: page-ready, modal-ready, and confirmed
  checkpoints each bracketed complete 1276-by-1422 Firefox retirements; routed
  pointer evidence covered both clicks; no surface was admitted after the
  stable page frame; and session, layout, and frontend cleanup were clean. The
  gate result closes the modal seam without requiring another replay of the
  operator sequence.

<!-- END IMPORTED BODY -->
