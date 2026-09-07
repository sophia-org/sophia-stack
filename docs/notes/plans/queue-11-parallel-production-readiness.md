---
id: queue-11
date: 2026-09-06
kind: plan
tags: [plan, milestone]
---
# Parallel Production Readiness

This plan retains the scope, constraints, and task details from the roadmap
cutover. Task status and order live only in [todo.md](../../../todo.md)
and the [monthly completion history](../../../done.md). Follow the
[work-tracking contract](../../work-tracking.md).
Historical candidate identities in the details require revalidation before use.


These rows do not reorder the critical path.


Previously completed evidence: [Shell reference preparation: generic boundary documented, Quickshell fork and sophia branch established, Void baseline built, and panel/popout requirements and results retained.](../sources/2026-09/todo-cutover-completed.md#legacy-done-015).


Previously completed evidence: [Document descriptor and content shell models and the proposed content-shell behavioral contract: explicit operator admission, panel/popout lifecycle, input and visual trust boundaries, and…](../sources/2026-09/todo-cutover-completed.md#legacy-done-016).


## t024

Repair the evidence readers still pinned below their emitter. Ten accept
`sophia_live_session status=bounded_complete` at schema 15 or lower against an
emitter that writes 16, and nine accept `sophia_live_wm status=ready` at
schema 1 against an emitter that writes 4. These are retired-policy physical and
QEMU gates; they fail loudly rather than silently, so each needs a per-gate
decision about whether it still earns its keep. Add each repaired record to
`tools/check_live_record_schema_readers.sh` once its emitters are confirmed to
agree.


## t025

Decide whether `run_frame_fed_output_gate_tty4.sh` and
`run_current_critical_path_tty4.sh` keep requiring HEAD to equal the locally
known origin/master. The direct-scanout and Hagia native runners no longer do;
`package_live_session.sh` keeps it deliberately, because packaging is the
publishing question the rule was wrong about being.


## t026

Move remaining session-private test modules out of production `src` as
visibility boundaries permit, and split the oversized cohesive units named in
`docs/source-layout-debt.txt`. Do not weaken privacy or add test-only
production APIs.


## t027

Reduce `tools/start_sophia_tty3.sh` to the minimum TTY/display-manager
adapter around `sophia session run`. Typed parsing, verification, archive
handling, and gate orchestration stay in Rust.


## t028

Repair the load-sensitive `sophia-x-authority` `x11_wire` flake. Rewrite
the affected tests together with `read_x_reply`: it currently treats Present
event type 35 as a reply and interprets bytes 4..8 as a body length. Raising
the ten-second timeout is not a fix. Preserve the 178-test baseline while
making record-kind parsing explicit.



Previously completed evidence: [Implement RENDER.](../sources/2026-09/todo-cutover-completed.md#legacy-done-017).


## t029

Implement `SHAPE`. Quickshell asks for it in the same trace. Small: a
handful of requests for non-rectangular window regions, and Sophia already
carries region machinery for XFIXES.

**Result.** Implemented at version 1.1 and advertised. The "small" framing
above was wrong in one load-bearing way, and the correction shaped the work:
Sophia's `Region` was a bare rectangle list with no set algebra, and XFIXES
implemented five minors while answering version 6.0. Region algebra had to be
built before SHAPE could combine anything.

Landed in four commits, each gated and independently useful:

- `87c09c12` region set algebra plus the XFIXES region minors (Copy, Union,
  Intersect, Subtract, Invert, Translate, RegionExtents, FetchRegion).
- `6264db79` the shape store and all nine requests, deliberately unadvertised.
- `4e26bf78` bounding shapes clipping composition.
- `3bf967b2` input shapes honoured in hit testing, and the advertisement.

**Evidence.** `x-authority-quickshell-smoke` reaches opcode 145: 35 opcodes and
331 requests, up from 34 and 329 before, with the
`sophia_x11_authority_extension status=absent name="SHAPE"` line gone. Twelve
dispatch tests cover the tri-state, all five operations, masks read from
depth-1 pixmaps, change gating, and validation; two engine tests cover
click-through. The algebra is compared against a brute-force cell model over
every pair in a small grid, and both `subtract` and the vertical-coalescing
invariant were mutation-checked (an inverted condition fails four tests, a
disabled coalesce fails three). See the `SHAPE window regions` and
`XFIXES region minors` rows in `docs/x11-compatibility-matrix.md`.

**Why the advertisement waited.** A Qt panel's first use of SHAPE is an input
shape for click-through. Advertising with shapes merely stored would have been
the MIT-SHM over-promise again, so phases two and three shipped dark and the
advertisement flipped only once clicks genuinely fell through.

**Adapted from yserver** (`~/src/yserver`, MIT, Copyright (c) 2026 Jos Dehaes):
the half-open banded region design and its brute-force test approach, the
bitmap-to-region reader, the unset/empty/concrete tri-state, and change-gated
notifies. Their `ShapeInvert` aliases to Set, which is wrong; Sophia implements
source-minus-destination and tests it. They also validate no arguments, and
Sophia does.

**Remaining limits.** A descendant window's shapes are stored, answered and
notified but do not clip the parent presentation or affect intra-toplevel
routing. `ShapeClip` is stored and consumed by nothing, because Sophia composes
whole client buffers and draws no window borders. A shaped window falls back to
scaling its 1x raster on a non-1x display. Grabs capture regardless of shape,
which is what X defines. No physical acceptance: the evidence above is the
offline probe and deterministic tests.

**Debt found on the way.** XFIXES answers version 6.0 while implementing a
subset. The region minors now answer and the rest refuse by name with a
two-tier code rather than failing to parse, but the version claim still
overshoots its implementation and should be settled -- implemented on demand,
or clamped -- as its own decision.


Previously completed evidence: [Implement XC-MISC, before something needs it.](../sources/2026-09/todo-cutover-completed.md#legacy-done-018).


## t030

Decide, rather than implement, `Composite`, `DAMAGE`, `XTEST` and `DPMS`.
Each is a domain Sophia owns -- compositing, input, power -- and a client
reaching through one of them is asking to step around that authority. They
belong in the matrix as deliberate exclusions or as admitted surface, not as
gaps that stayed open because the list looked incomplete.

**Reference.** The `~/src/yserver` survey done alongside t029 supplies what each
would cost and what it buys, from a server that implements all four and runs
whole desktops. That is evidence rather than speculation, and it is why these
decisions can be made now instead of deferred again.

**Composite -- excluded, with a named admission price.** Sophia is the
compositor and Hagia is the only window manager; redirection is authority
Sophia does not delegate. yserver resolved the same tension by handing
compositing over: when a client claims the overlay window their scene emits
only root, overlay and cursor, which is Xorg's contract. If a measured client
ever needs this, that is the shape to adopt, and their capability flag
(record redirects and answer `NameWindowPixmap` before allocating real
backings) is the staged path. Panels and thumbnailers want `NameWindowPixmap`
specifically. Until the refusal log names one, absent by decision.

**DAMAGE -- excluded for now, and now measured.** The original rationale said
its consumers are external compositors and screen scrapers. That was wrong
about who *asks*: GTK4 zenity queries it, and the GTK3 probes added under t006
show mousepad and Thunar querying both `Composite` and `DAMAGE` at startup.
What the measurement does support is the decision: all three find neither,
continue, and complete startup with no error, which is the clean fallback the
exclusion assumed and had not previously seen a client perform. One lesson to
keep if it is ever admitted: yserver runs three separate region machineries
with written justification, because client-facing damage reports and internal
repaint damage answer different questions. Their presentation damage subtracts
by exact-match rather than geometrically, on purpose. Do not alias the two.

**XTEST -- excluded, and the reason is not cost.** It is four requests, and
yserver injects at the same entry point real libinput events use, which is
also what Sophia would do. What stops it is that theirs is entirely ungated,
and their own design notes record that as a known gap: any client can drive
the pointer and keyboard. Sophia's input is session-owned authority, so
synthetic input needs an explicit admission story before it exists at all.
Revisit when conformance tooling (xts5 drives the mouse through XTEST) makes
it worth designing that story.

**DPMS -- excluded; power is session authority.** yserver's is real, driving
DRM atomic commits that disable connectors, guarded by a scanout check after a
VT-switch left outputs half-disabled mid-modeset, and coupled to
MIT-SCREEN-SAVER in Xorg's order. If Sophia admits DPMS it must route through
session power ownership rather than authority-side connector writes. Until a
client asks, absent.

**XFree86-Bigfont -- leave absent, log-driven.** Surfaced by the xterm probe
during t029. yserver has nothing, not even a refusal note, and xterm works
there; Xlib falls back cleanly. No action unless a client fails rather than
falls back.

**A pattern worth borrowing if VidMode writes ever appear.** yserver keeps
XF86VidMode deliberately read-only, advertising read permission and failing
writes with the extension's own `ClientNotLocal` error -- a branch clients
already handle -- rather than `BadRequest`, because RandR owns display
configuration. That is a better shape than refusing outright for any legacy
extension whose read surface is useful and whose writes cross an authority
boundary.


## t059

Settle the `XFIXES` version claim. The server answers 6.0 while implementing a
subset of the minors that version defines. Found during t029, which added the
region minors (Copy, Union, Intersect, Subtract, Invert, Translate,
RegionExtents, FetchRegion) and converted the remaining unimplemented minors
from a parse failure to a refusal that names them with a two-tier code. What is
left is the version claim itself: cursor naming and images, pointer barriers,
save-set changes, and the client-disconnect modes are advertised by the version
and not implemented.

Either implement what a measured client asks for, or clamp the answered version
to what is behind it. The precedent is RENDER, whose advertised version moved
only as the requests behind it started answering; the counter-example is
MIT-SHM, which advertised 1.2 with two opcodes missing and sent Qt into its
error handler. Lowering an already-negotiated version is a behaviour change to
shipped clients, which is why it is its own decision rather than a fix folded
into t029.

## t060

`QueryPointer` answers zero for every coordinate: `root_x`, `root_y`, `win_x`
and `win_y` are all reported as the screen origin, `child` as none, and the
button mask as empty, whatever the pointer is actually doing.

Found while diagnosing a GTK menu that opened offset. The offset itself was
`TranslateCoordinates` echoing its input, fixed in `ae7b0929`; this is the
second defect in the same area and was left out of that commit because it
needs something that one did not.

A client uses this to place a menu at the pointer -- which is what a
right-click context menu is -- and to follow a drag. Answering the origin puts
those in the corner of the screen.

The obstacle is structural rather than arithmetic. Pointer position lives in
the socket routing layer, which encodes input events; `QueryPointer` is
answered in dispatch, which holds the runtime and never sees a pointer. Either
the routing layer records the last position somewhere dispatch can read it --
the runtime already holds `input_focus`, so there is precedent for input state
living there -- or the reply is assembled where the position is known. The
first is smaller and matches how focus already works.

Worth doing with the `child` field of `TranslateCoordinates`, which is
unanswered for the same reason: both need to know what is under a point.

Every row above came from measurement rather than a survey of what a server
usually has. `QueryExtension` now records what it refuses, so the next live
session extends this list by observation; the four decisions above should be
revisited against a week of real logs rather than against this paragraph.

Completed infrastructure baseline: `sophia-session` owns production lifecycle,
`sophia-conformance` owns development-only evidence logic, `cargo xtask` is the
canonical developer/CI surface, `just` is optional human shorthand, canonical
installed commands live under `sophia session`, and source-layout debt is an
exact identity ledger.
