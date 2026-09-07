---
id: 37xvg0y7
date: 2026-09-07
kind: investigation
status: awaiting-physical-acceptance
tags: [investigation]
---
# Qt menus fail to open while pointer leases are rejected

## Driver and scope

The user opened a text file from Thunar in Okular and reported that menu
clicks open nothing and mouse interaction is spotty. This follows the short
successful t065 hide-and-resume check in the same installed session; that
acceptance did not establish general application usability. This user-selected
follow-up is t066. Inspect the live Qt popup and pointer-grab paths, reproduce
the failure in an isolated probe, repair the responsible boundary, and confirm
menu opening and selection in normal use. Preserve namespace and presented-input
constraints; do not weaken them to make a grab succeed.

## Evidence

Session `00000001788812367995-fe320326-8322-45b2-b23e-7c1781c12d10`
runs commit `2efff4cec91392d85f81d5c8a633bf7711eea3a4`. Its recorder remains
running without loss or storage errors. The root window tree identifies the
application as Okular, a Qt client; this is not evidence of a GTK failure.
Okular's main X window is `0xe00009`, with several separate menu windows.
The tree lists children regardless of mapping, so it does not establish that
any menu is visible.

The trace records repeated explicit pointer-grab rejections and releases.
The installed diagnostics do not name the rejection reason. Code inspection found
two candidate boundaries: preparing a grab requires a target in the retired
input projection, and an active lease rejects any change in its presentation
epoch. Neither is yet established as the cause of this user's menu failure.
Read-only attributes confirm all eight sampled Okular menus are currently
unmapped, override-redirect windows with WM_TRANSIENT_FOR naming the main
window. Absence of metadata-broker log records does not prove absence of a
map: those records describe metadata commits, not every mapping transition.
No live input, focus, configuration or session lifecycle was changed.

## Connections

- [Hidden-window input ownership](ohkzr8kg-unmapped-dialogs-retain-input-ownership-after-leaving-the-scene.md)
  records the prior repair and the scope of its short installed acceptance.
- [Explicit click-lease promotion](744uylx4-explicit-pointer-grabs-must-replace-their-own-click-lease.md)
  covers the previous own-grab replacement defect; do not assume it explains
  every subsequent rejection.

## Synthetic Qt ordering evidence

A temporary Qt6 Widgets probe uses a main window, menu bar and QMenu. Its
first version crashed with QtTest linked, including on the offscreen backend;
that startup fault is not evidence against Sophia. Removing QtTest and using
local QApplication event delivery let the synthetic client run. It injects no
physical events into the user's desktop.

Installed baseline trace `/tmp/sophia-qt-menu-zedm5ck6/session.log` records
MapWindow completing before XIGrabDevice is rejected on the first opening.
Later GetWindowAttributes reports that menu IsViewable. QMenu reports itself
visible despite the rejection. The second opening's grab succeeds in this run.
This reproduces a grab failure, not the user's whole Okular symptom.

A diagnostic build logged the Engine rejection branch without changing its
behavior. `/tmp/sophia-qt-menu-ezuzumos/session.log` shows the first Prepare
rejected because the session's route table did not yet contain the admitted
anchor. The second Prepare is rejected because the anchor has no presented
input entry. Both delayed attribute queries report IsViewable. The temporary
instrumentation was removed from production source after building; it was
never installed. The diagnostic binary remains `/tmp/t066-instrumented-sophia`.

These outcomes expose two ordering dependencies: the Engine must observe the
preceding authority mapping facts before adjudicating its grab request, and
X viewability cannot be equated with already-presented pixels. The existing
protocol separates Prepare from Activate, but both still require a reservation
whose target already exists in a presented projection. A correction must
preserve exact admission, namespace and lifetime checks while separating grab
reservation from physical input eligibility. The synthetic client remaining
visible means another factor may contribute to Okular's missing menus. No
production fix or installed acceptance is claimed.

## Wayland comparison

The [xdg-shell protocol source](https://chromium.googlesource.com/external/anongit.freedesktop.org/git/wayland/wayland-protocols/+/e8f7d4ebbd3d85b174e965ad601d806a49238696/stable/xdg-shell/xdg-shell.xml)
permits the explicit popup grab before mapping and requires a triggering user
input serial and seat. Parent and nested-popup rules constrain its lifetime;
dismissal ends the popup interaction. Mapping and presentation are separate
from authorizing that request. This is a useful design distinction, not a
proposal to impose Wayland's user-event requirement on existing X11 clients.
Sophia's frontend must keep X semantics; Engine must retain authority over
physical routing and namespace boundaries.

## Brave recurrence and active-grab hypothesis

The user subsequently opened Brave-Origin and reported mouse input dying.
The same session remained running without recorder loss or runtime-fatal
records. A read-only query identified Brave's mapped main window `0x3800003`;
focus was on Kitty by the time of inspection. That observation does not identify
Brave's focus at the time of failure. Recent input records show explicit-grab
preparation, activation and release as well as pointer routing. The existing
counts do not establish why an individual application appeared unresponsive.

A separate code audit confirms that ApplicationRouteLeaseState::authorize
rejects any presentation-epoch mismatch. The backend advances that per-output
epoch on interaction-projection changes, including adding or removing an
unrelated layer. This can revoke a held grab after an unrelated popup changes
the scene. It is a demonstrated mechanism, not yet a live attribution of the
Brave failure; installed diagnostics do not distinguish StalePresentation from
other authorization failures.

Any replacement must revalidate the scope at the pointer using the actual
presented scene, while preserving the original lease's admission, authority,
control epoch, surface lifetime, device and output restrictions. It must also
respect the existing [grab-scope contract](../../target-resolved-input.md):
confined grabs may cross application regions in their namespace, and
classic-shared grabs may cross application surfaces in that profile. Another
application above the original target does not automatically revoke a grab if
it lies within the granted scope. Shell, secure and foreign-scope regions do.
The pointer need not remain inside the original target's geometry; requiring
that would break the intended retained grab.

The proposed regression matrix therefore distinguishes unrelated scene
updates, another permitted application under the pointer, and forbidden
shell/foreign-scope occlusion. Testing authorize alone cannot demonstrate
occlusion: that function receives already-derived scalar scope and epoch
values. The regression must exercise presented-scope resolution as well as
lease validation. This is a proposed contract refinement, not permission to
remove the epoch guard without replacement or to widen an existing grant.

## Local Rust references and contract amendment

The user identified `~/src/wayland-rs`. The inspected checkout is
Smithay/wayland-rs at `0813584ea50379bd22e95dc1e0b50f02a4b36ca2`, with an MIT
license in LICENSE.txt. It supplies protocol bindings, transport and object
lifetime machinery; it is not the Smithay compositor-framework repository.
Its ObjectId contract prevents equality across different clients or recycled
protocol identities, making it relevant to stale-handle tests. Protocol XML
submodules are not initialized in this checkout. Niri's local Cargo.lock pins
Smithay/smithay at `4cf0b62028039661477d482ec4758b687d8f4392`, which is the
separate implementation reference to inspect for compositor popup/grab logic.

The existing normative input contract explicitly requires an exact presentation
epoch. Preserving a grab across a scene update therefore requires a documented
contract amendment, not merely a code correction. Pin current behavior first,
then change the expected scope-preserving case alongside the specification and
an ADR. Keep the scope-exit and target-evidence-loss regressions unchanged.


## Implementation and deterministic acceptance

The accepted [grab ownership ADR](../decisions/mbvdvhk5-separate-grab-ownership-from-presentation-evidence.md)
separates authoritative mapping, lease ownership, and retired presentation.
The implementation now carries the last actually published observation receipt
into Prepare, accounts it after the owner applies lifecycle effects, and releases
frontend state locks during arbitration. Mapped client-positioned windows
publish their exact creator route even before drawing; their eligibility no
longer depends on a CPU buffer or WM layout node.

Engine keeps phase and presentation binding separate. An unpresented reservation
can succeed before drawing; it cannot receive physical input until binding and
current evidence checks pass. Promotion preserves its output and earliest
pending deadline. Retained delivery revalidates original target identity and
the presented scope, including compositor occlusion, without treating unrelated
scene revisions as revocation. All existing namespace, control epoch, admission,
authority-session, device and output restrictions remain. Held input and pending
requests have bounded, exact-identity cancellation. The WM interface is unchanged.

The unchanged Qt/raw probe rejects the installed baseline in its mapped,
owned, draw-after-grab case: grab status 3 prevents the client from drawing its
marker. Candidate tests receive grab success before drawing, verify the exact
own-window marker, open and select Qt menus, and report a nonempty composed
scene. These are separate observations: own-window pixels and nonempty frame
counts do not prove the exact composed menu image or physical input recovery.

Review also caught two integration faults before acceptance. Binding an already
bound lease returned InvalidPhase; the physical routing path now compares its
pinned output and only pins an unbound lease. A regression through the actual
input loop verifies motion and button release across a scene change, and fails
when the old call is restored. The GTK probe then exposed an Engine release
racing the client's ungrab. Repeating release for the same exact identity now
joins the existing release without extending its deadline. The rejected release
had also stranded a frontend transaction ticket; later disconnect cleanup waited
behind that gap. Frontend error paths must account for every allocated ticket
and preserve completed dispatch effects before cleanup.

The model checks cover bounded admission ordering, cancellation, readiness and
scope-preserving scene changes, with negative controls that violate prerequisite
accounting, cancellation, exact scene equality and target eligibility. They are
contract models, not a Rust trace-refinement proof. Their coverage and limits are
listed in the [model record](../../../validation/tla/pointer-grab/brief-coverage.md).


The repaired ticket path let GTK complete all five captures, then exposed a
second teardown race: a queued FocusSurface command arrived after its target
was destroyed. The exact correlated UnknownSurface completion now retires the
obsolete focus command without counting it as applied. Engine clears only that
surface's focus, handoff and lease claims, including staged claims that could
otherwise restore it. Admission and configuration rejections remain failures.
This uses the existing lifecycle cleanup and leaves replacement focus to the WM.


## Validated candidate and remaining acceptance

Base commit: `2efff4cec91392d85f81d5c8a633bf7711eea3a4`, plus the retained
working-tree source snapshot. The candidate was built with
`cargo build --offline -q --bin sophia --features native-session`.
Binary SHA-256:
`c527f55d86b0acc51ea0baca8366522e268a25f980cdfd065e496c3ce52b971a`.

Both unchanged private headless probes pass against this same binary:

- Qt/raw: five successful grabs, four Qt captures, exact own-window marker
  drawn after grab success, menu selection and dismissal, and 84 nonempty
  frames. Client, session and verifier exit 0.
- GTK: five captures spanning main window, dialog, menu, remap and redraw;
  focus after dialog unmap passes; 44 nonempty frames; client, session and
  verifier exit 0. Health and cleanup are clean. This run did not log the
  stale FocusSurface branch, so it does not independently reproduce that race;
  the control tests pin its changed classification.

The session regressions cover actual retained motion and release, same-scope
application overlap, foreign and compositor occlusion, mapped popups before
pixels, observation prerequisites, control-epoch cancellation, held-input
bounds, active Abort and simultaneous release. Engine tests cover all twelve
readiness combinations, stale identity/evidence, promotion pins and deadlines,
and output loss. Frontend tests cover transaction gaps, lock progress during
arbitration, cancellation and capacity, early and late dispatch failure, and
preservation of peer-owned mapping and CPU effects. The actual physical-routing
regression fails when its old bound-output call is restored.

Final `cargo xtask check` exits 0: all-feature workspace tests and Clippy,
formatting, source-layout and diagnostics checks, installed verifier fixtures,
and the retained physical archive checks pass. The relevant bounded TLA+
models and their named negative controls also pass their expected outcomes.
The Qt verifier has eight passing tests. Task IDs and note links were checked;
`zk index` and `git diff --check` are clean.

Evidence is retained at
`~/.local/state/sophia/development-evidence/t066-pointer-grabs-20260907`.
It includes the installed comparisons, rejected candidates, final probe
identities and logs, model runs and negative controls, source manifest and
patch, and the compressed final binary. The failed GTK runs remain part of the
record; they are not replaced by the passing run.

No live session was installed, restarted or driven with synthetic physical
input. t066 remains open for normal installed use: open and close Okular menus,
select entries, and exercise Brave clicking and dragging through ordinary window
changes. Confirm that applications retain responsive input and release it on
exit. A green synthetic client does not attribute or close every Brave stall.
