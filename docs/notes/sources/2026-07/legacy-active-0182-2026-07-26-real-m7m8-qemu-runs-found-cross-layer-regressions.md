---
id: legacy-active-0182
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-26: Real M7/M8 QEMU Runs Found Cross-Layer Regressions

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6216–6253. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Running the retained scenarios as real guests, rather than relying only on
fixture verifiers, exposed five independent defects. The M7 click probe landed
on newly reserved compositor clearance at the output edge; the harness now
resets to the edge and moves 32 pixels into client content. The M8 Vulkan
fixture assumed its fixed startup size would survive xmonad relayout; it now
uses a deterministic software-rendered size on a separate workspace so the
test measures mixed presentation rather than toolkit resize timing.

Firefox then reached an unimplemented XSync counter request after Sophia
advertised XSync 3.1. The frontend now implements bounded generic counter
create, set, change, query, destroy, list, resource ownership, and namespace
cleanup. No browser policy entered Engine. Firefox also creates more than one
top-level surface and its software-rendered content does not retire through the
Present ledger. Launch admission therefore tracks a bounded fixed set of
observed surfaces and settles on any committed stable visual surface, whether
stability is proven by CPU visual detail or retired Present.

The final GTK launcher close exposed a teardown ordering defect. Shortcut
prefix modifiers had entered the per-client pressed-key ledger, but the
launcher did not select keyboard events. Sophia waited for synthetic release
acknowledgements before sending close, so the close timed out without being
dispatched. Closing surfaces now clear those keys through the state-only
authority path and dispatch close immediately. Focus handoff and VT suspension
still deliver ordered releases because those clients remain alive. The real
M8 run completed all eight browser stages, delivered all 48 controls with zero
timeouts, reaped the launcher, drained input and WM state, and revoked the
namespace and X authority cleanly.

The guest also established a precise compatibility boundary. Physical wheel
axis input is observed, hit-tested, and routed to Firefox, but the current
reduced core-button translation does not produce Firefox DOM `wheel` events.
The fixture advances its scroll stage with a focused Space key only after a new
axis route is observed. This proves generic Engine axis routing and continued
browser interaction; it does not claim native Firefox wheel compatibility.
That work remains in the X frontend compatibility milestone.

<!-- END IMPORTED BODY -->
