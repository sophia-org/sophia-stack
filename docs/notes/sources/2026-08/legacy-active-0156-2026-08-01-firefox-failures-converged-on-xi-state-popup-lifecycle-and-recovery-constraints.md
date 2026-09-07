---
id: legacy-active-0156
date: 2026-08-01
recorded_date: 2026-08-01
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "session", "x11"]
---
# 2026-08-01: Firefox failures converged on XI state, popup lifecycle, and recovery constraints

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4987–5110. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The latest physical observations separated three failures that the old
eight-stage fixture could accidentally conflate. Wheel packets could reach a
page without moving the document; Firefox popup/toplevel lifecycle changes did
not always publish a compositing snapshot; and a temporary exact-size recovery
extent could survive successful visual admission, pinning Firefox at
1280-by-1040 while xmonad resized only the two Kitty surfaces. The old resize
verifier compounded this by looking for focus action 1 even though
Super+Space is xmonad action 3.

The X frontend now reports current valuator values from `XIQueryDevice`, full
hierarchy-relative `XIQueryPointer` coordinates/child/button/modifier state,
and immediate-child plus button-mask state in XI2 device events. Root-child
`WM_TRANSIENT_FOR` is reduced to a protocol-neutral presentation-owner edge.
Attached client-positioned surfaces publish map/unmap snapshots, follow the
owner's workspace visibility, never enter blind-WM admission, and stay hidden
after owner removal until the client publishes a new ownership snapshot.

Successful CPU admission or exact DMA-BUF retirement clears the Engine-owned
temporary recovery extent and requests one coalesced relayout. Clean shutdown
now fails if any such extent or relayout obligation remains. The offline page
requires a real local navigation, a post-baseline DOM wheel event, and nonzero
document displacement while the verifier independently requires both physical
axis routes. The strict physical verifier requires a three-surface
resize epoch/layout, action 3, a three-visible-surface projection, and zero
recovery constraints. Focused Engine, X wire, transient lifecycle, query reply,
and verifier mutation tests pass locally. A fresh physical run remains the
Milestone 10 acceptance boundary; these changes do not promote historical
evidence.

The first strengthened QEMU run then proved the navigation click and both
axis routes, but timed out at scroll. The fixture incremented its wheel counter
only for nonzero DOM deltas, even though GTK can consume the first XI2 absolute
value as a zero-delta baseline. After correcting that counter, a second run
exposed the independent harness race: both notches were routed within 160 ms of
the click, before the replacement document had an observable ready point. The
navigated fixture now publishes an out-of-band title-length checkpoint, the
session reports a redacted `navigation_ready` marker, and QEMU waits for it
before injecting exactly two notches. Because XI2's first absolute value does
not produce a DOM wheel event, the page requires the second notch's DOM event
plus `scrollY > 0`, while the verifier independently requires both routed axis
packets between navigation readiness and the scroll checkpoint. Thus packet
delivery without real document displacement still cannot pass.

That corrected run passed scroll, resize, and refocus, then exposed an existing
fixture gap: Firefox rendered JavaScript `alert()` as a tab-modal overlay, so it
could not prove the X11 transient-toplevel lifecycle at all. The dialog step now
opens a real click-gated Firefox popup with an autofocus confirmation button.
The harness waits for its attached four-surface layout snapshot before sending
Return, then waits for the popup's X focus acknowledgement because layout
publication precedes focus application. The popup finalizes its blank document
before installing its confirmation handlers. The Enter handler publishes the
final redacted title-length checkpoint on the popup itself before closing,
avoiding cross-process messages and throttled opener timers during teardown.
Close is delayed by one second so Firefox can publish `_NET_WM_NAME`; the
harness then requires the return from four to three surfaces and publishes
dedicated redacted `dialog_open` and `dialog_closed` checkpoints. This directly
exercises the popup-lifecycle snapshot that the strengthened session path is
intended to guarantee.

The first real-popup QEMU run found one more ordering boundary: the attached
surface and X focus acknowledgement both preceded Firefox installing the
popup's DOM key handler, so an immediate Return was lost. The later retry then
opened a second popup and made an already completed stage look like a close
timeout. The popup now publishes a distinct redacted `dialog_ready` title only
after its confirmation handler is installed. QEMU waits for that readiness,
uses a pre-interaction stage baseline instead of accepting stale completion,
and fails the confirmation attempt instead of reopening an ambiguous popup.
The ready-popup run also proved that X toplevel focus does not guarantee
Firefox's internal keyboard focus proxy will deliver an immediate synthetic
Return. Because this stage is a pointer and popup-lifecycle proof (keyboard is
already proven separately), QEMU and the operator now click the popup's
full-window confirmation button. The title checkpoint wait is forty seconds:
under llvmpipe load the redacted metadata batch can trail the visible popup
close by more than twenty seconds.

The physical verifier now carries the same boundaries into the promotion
contract. It requires replacement-document readiness before counting its two
wheel routes, then orders popup document readiness, a four-surface layout,
dialog confirmation, and the return to three surfaces before Firefox's normal
exit. Mutation fixtures independently remove each readiness and layout record,
so the physical gate cannot regress to accepting the former overlay-only
dialog or pre-navigation wheel delivery.

The first run against that contract reached real document scroll but exposed
an admission-time timeout rather than an input failure. The initial diagnosis
widened manage-surface resize fences from two to eight seconds because Firefox
had published a 1280-by-1040 buffer but had not satisfied the three-window
1276-by-1422 epoch. That avoided an early rollback but did not make Firefox
honor the size. The physical verifier also counts xmobar's retained
non-workspace surface explicitly: four surfaces at the normal Firefox
baseline, five while the real popup is attached, then four again after close.

The next physical launch exposed two coupled owner-loop bounds. Application
admission still used the five-second proof-completion timeout even though a
manage-surface resize may now wait eight seconds (and the session accepts at
most ten). It declared Firefox timed out while that layout fence was valid.
At nearly the same point, a pointer focus handoff accumulated a full bounded
batch and propagated its capacity result as a fatal session error. Application
admission now has a distinct twelve-second bound, strictly beyond the maximum
WM transaction. Adjacent held pointer motions coalesce; an exceptional full
handoff is discarded atomically and reported without terminating the desktop.
Focused regressions cover both deadline ordering and the bounded input path.

The immediate rerun proved the longer fence was masking the actual authority
violation. Sophia admitted Firefox at the WM's 1276-by-1422 geometry, then
accepted Firefox's own mapped-toplevel `ConfigureWindow` and let it overwrite
that Engine-owned geometry with 1280-by-1040. The epoch could therefore never
match and the browser stayed hidden for the entire eight-second fence. Mapped
policy-managed toplevel geometry is now immutable from the client path;
children, override-redirect windows, and pre-admission windows retain their X11
geometry authority, while a denied toplevel request receives the current
Engine geometry in `ConfigureNotify`. The xmonad admission fence returns to two
seconds as a bounded fallback.

The same trace exposed a recovery ordering defect: selected admission pixels
were drained while the surface still remained in the unassigned WM set. The
later policy projection made Firefox visible only after that one-shot frame was
gone. Released admission groups now remain quarantined until policy assignment,
then enter production exactly once. Pointer focus handoff remains separately
bounded at four seconds so the two-second layout fallback cannot win a timeout
race with a click made during launch.

<!-- END IMPORTED BODY -->
