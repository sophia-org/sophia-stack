---
id: ce2b55uy
date: 2026-09-06
kind: investigation
status: investigating
tags: [investigation, x11, rendering, validation]
---
# Blank Thunar menus and frozen Brave need separate pixel and delivery evidence

## Question

Which boundary fails when Thunar menus have blank, black, or missing portions,
and when Brave Origin stops responding until the user switches windows?

## Evidence

The installed session is `00000001788751946481-31db6852-d07f-4f08-8ed9-87f63a561f59`,
built from `86ab21e4879cc5b3154ca1192775de73e6e6a030`. The binary SHA-256 is
`0d972718c734751aba7fe58eb5075eb9f2f776bb4da7fdea3d69e80f98ac4d06`.
The applications are Brave Origin and Thunar; the user corrected an earlier
reference to Firefox. Thunar's symptom is missing pixels, not a reported menu
position error. Brave accepts a few clicks and then stops responding entirely;
the user reports recovery after switching away and back.

The retained input-lease and explicit-grab records contained only their schema.
Chrome records retained their generation but lost frame and focus counts.
`diagnostics::reduced_record` omitted these fields and most input status values.
Moreover, successful pointer delivery emitted only a first-use marker. Zero
recorder loss therefore did not establish that later clicks reached Brave.

The CPU presentation code in `software.rs::present_window_damage` chooses AR24
only for a bounding shape and otherwise tags the buffer XR24. It does not use
the window's depth-32 visual. The compatibility matrix already names that alpha
loss. It can explain black transparent areas, but no captured Thunar popup
establishes that this is the whole menu failure. Startup smoke tests require no
mapped window or pixel proof, so their success does not accept menu rendering.

## Diagnostic correction

The recorder now keeps record-scoped delivery, grab, frame, and timing counts,
and the fixed status vocabulary needed to interpret them. It still excludes
application identities, coordinates, button/key codes, and payloads. Pointer
button batches retain observed, routed, and suppressed counts after first use.
They use the existing bounded, asynchronous recorder; no disk work enters input
routing. This changes evidence collection, not input policy or presentation.

## Validation and remaining work

Two regression tests pass for diagnostic retention, vocabulary scoping, numeric
bounds, and payload exclusion. `cargo xtask check` passed, including workspace
tests, Clippy, layout checks, fixture verifiers, and the host buffer-age proof.
The first sandbox run stopped at a denied Unix-socket bind; the complete run
passed with local sockets available.

The observed evidence directory is `/tmp/sophia-interaction-evidence-197b2af77a09`.
It holds the source patch, full check log, timing-probe script, and identity.
The source-patch SHA-256 is `197b2af77a093f8cf1c14a23fff4f007e00ff4a9af9fce5ec8b3e5ada2cab217`.
This temporary path is not a durable physical-acceptance archive.
The code change is based on `86ab21e4` and was committed as `e8573cf1`.

The replacement session is
`00000001788753082918-0a84405d-9e24-433d-950f-d3dce25a2607`, installed from
`e8573cf17afc022865009abe75c2e3fbc18db484`. Its binary SHA-256 is
`c3bb971a6925a9366480bb25eb6155d5fa1ebf8e177705d08f47d7f1787fa74f`.
Preflight, input guard, and graphics takeover completed. The recorder reports
no discarded records or storage errors. Chrome events now retain focused,
unfocused, and primitive counts, confirming that the diagnostic change is
installed. No pointer-button batch appeared in the initial sample. The next ordinary-use
recurrence retained routed clicks and repeated explicit-grab rejections, leading
to the [click-lease investigation](744uylx4-explicit-pointer-grabs-must-replace-their-own-click-lease.md).

For t061, reproduce a mapped GTK menu with retained pixels and trace its actual
rendering requests. Compare frontend pixels and alpha format with Engine's
composed result before choosing the repair. Regress the failed boundary, pass
required checks, then accept visible menu text, background, edges, and submenus
in one installed Thunar use. Keep menu placement and drag acceptance under t060.
The new diagnostic counters support the existing Brave t003 investigation;
they do not establish its root cause or accept its usability.

## 2026-09-07: GTK clip reset and a mapped damaged dialog

The new installed session is
`00000001788785369819-028cfec4-8b74-46ef-93b3-6c529dea2ddc`, release
`4299e1cabb650cd481096f43ac8b5186d31ad4f1`, binary SHA-256
`9b152ebc7919635afb086b288bc4bc1cd747d752c55c077e318690d18c659386`.
The installed package is Thunar `4.20.9_1`, linked against GTK 3; this is not
a GTK 4 report. The user sees black and missing areas after interacting with
the sidebar or menu bar, and a black box with a pale border and white area
over Kitty after switching windows in the same workspace.

A read-only X11 probe captured Thunar's window attributes, hierarchy, and
drawable pixels. It sent no input and inspected no clipboard contents.
The main window's retained image has intact menu-bar and sidebar content.
The dropdowns were unmapped at capture time. A separate, still-mapped
`_NET_WM_WINDOW_TYPE_DIALOG` has `WM_TRANSIENT_FOR` pointing to Thunar and a
377-by-112 drawable that is mostly black with a white rectangle. Its only
child is 1-by-1, so missing child content does not explain this capture.
This is a candidate for the reported ghost, not proof of its identity in
the composed output. All observed Thunar windows have depth 24. The zero
fourth byte in their readback is padding, not evidence of lost alpha.

A bounded private session using the installed binary reproduced incomplete
GTK dialog content. A basic menu rendered correctly. Interposing Xlib calls
in this synthetic client showed GTK/Cairo repeatedly setting temporary
RENDER rectangle clips, then sending `ChangePicture(CPClipMask=None)`.
The frontend decoded the latter as an accepted no-op and retained the old
clip. The [RENDER contract](https://www.keithp.com/~keithp/render/protocol.html)
requires None to remove that restriction. The repair carries an explicit
clear request through decoding and clears the picture's rectangles before
later drawing. Omitted attributes preserve the clip; unsupported pixmap masks
remain refused. This stays within X11 protocol state and changes no WM or
namespace authority.

The pixel regression fails before the repair and passes after it. Tests also
cover omitted attributes, preservation after a refused mask, and both wire
byte orders. `cargo xtask check` passes. In the same synthetic GTK probe the
previously missing Close button becomes visible, but other dialog content
remains incomplete. The probe also encounters a separate core ChangeProperty
BadAccess on dialog remapping; GTK labels it `GLXBadPbuffer`, although the
reported request is core opcode 18 and error code 10. Session exit zero only
establishes bounded session cleanup: the synthetic GTK client exits with an
error. Neither result accepts Thunar's live behavior.

Extra readbacks between the synthetic client's drawing operations preserve
both label and button; removing that instrumentation again loses the label.
Keep this timing sensitivity separate from the deterministic clip-reset
regression. The final probe uses no interposition preload.

Private evidence is retained under
`~/.local/state/sophia/development-evidence/t061-77c1ab98b5a1` with validated
checksums. It includes two live Thunar captures, the synthetic client and
tracer, baseline and candidate images, the full check log, and the four changed
source/test files against `4299e1ca`. The source identity is
`77c1ab98b5a199bdd36285fd58f3a959db874bfa5f7fc7b8d39a4ed6f78106c2`;
`identity.json` records its per-file hashes and candidate binary hash.

t061 stays open for the remaining drawing defect, mapped-menu evidence,
comparison with composed output, and installed acceptance. Do not hide a
mapped dialog merely to remove the visual symptom, or infer popup ownership
from application identity in the blind WM.

## 2026-09-07: complete clipping, dialog reuse, and mapping before policy

The preceding evidence records the first, incomplete clip-reset candidate.
The next candidate also preserves core GC clip origins, implements
`ChangeGC(clip-mask=None)`, and distinguishes an empty clip from no clip in
both core drawing and RENDER. Empty regions remain extractable through XFIXES;
replay cannot mistake an empty clip for an unrestricted image upload. Invalid
and unsupported writes preserve the old attributes. Validating picture values
before mutation also removes the need to clone retained clip rectangles on
each `ChangePicture`.

With these changes, the original synthetic GTK dialog contains its label and
Close button without interposed readbacks. A second failure had killed GTK on
dialog remap: it rewrote `_NET_WM_STATE` after hiding, but the property table
still classified that initial hint as immutable Engine feedback. The exception
now requires an accessible, unmapped window with no pending policy admission.
It changes the X property, not Engine state. Both byte orders, foreign namespace
denial, pending admission, mapped feedback, and protected `WM_STATE` are tested.

The reusable [GTK probe](../../../tools/probes/README.md) checks actual client
exit and synthetic pixels in both dialog halves and every menu row. The first
run is `/tmp/sophia-gtk-redraw-iw35swgi`: all five captures pass and GTK exits
zero. Running the same probe against installed `4299e1ca` produces only three
captures and a GTK exit of one on remap (`/tmp/sophia-gtk-redraw-yy_bg22v`). Its
controlled CSS changes drawing requests: that baseline's initial pixel checks
pass, so this comparison proves the remap regression, not the original themed
dialog's missing pixels. The original probe and exact wire-pixel regressions
supply the separate clipping evidence. Batched, three-byte fragmented, and
paced socket writes produce identical final pixels and published buffer updates.

Parallel review with the adjacent Claude agent confirmed another defect:
managed scene visibility trusted cached WM projection after authority unmap.
Popups could also remain visible through an unmapped managed owner. Both paths
now require authority mapping before consulting policy. The two regressions
fail without that gate. A third test preserves existing popup remap, destroy,
and generation-reuse behavior. Destroy already purges the layer and mapping;
the repair does not hide a valid dialog merely because focus moves elsewhere.

The input audit found a second stale projection. Native pointer routing reads
the last retired frame; a scene fix alone leaves an unmapped target eligible
until another flip completes. Successful layout publication now prunes departed
surfaces immediately and advances the input epoch. Retirement also intersects
its frame with current eligibility, preventing an older pending frame from
restoring a dismissed target. Both guards have independent mutation checks.
Survivors keep their retired geometry. Membership uses a set, avoiding a
quadratic scan at the 1,024-surface bound. These are protocol-neutral lifecycle
checks, not X11 policy in Engine or application identity in the WM.

A fresh read-only capture at `/tmp/sophia-thunar-pixels-1788788253` still finds
the six menu windows unmapped and the damaged Error dialog mapped. Listing
root children is not evidence that those children are mapped. The running
session predates these repairs and the separately committed chrome-focus fix
`2d81faa3`. t061 remains open for installed Thunar acceptance; synthetic
frontend pixels alone do not establish the physical composed result.

### Combined candidate validation

The combined `cargo xtask check` passes, including workspace tests, Clippy,
fixture verification, and the host buffer-age pixel-equivalence proof. The
final rebuild passes all five GTK content checks and exits zero, with no
protocol errors (`/tmp/sophia-gtk-redraw-gh6ydc2y`). The remapped dialog and
menu PNGs were also inspected. Formatting, diff checks, task links, and all
64 task IDs pass validation. Earlier checks that overlapped the input-helper
signature edit were discarded; the final check used the completed code.

Private evidence is archived and checksum-verified at
`~/.local/state/sophia/development-evidence/t061-229aac9b83b7`. Its source
identity is `229aac9b83b7634108752e8f979a80b8c771202c2700ab1b9a1c18bdb87dd898`
against `2d81faa3`; the candidate binary SHA-256 is
`7362c6964d1699434b71381db7c874f5bd6c3ef07d9f5dc440066278331a1157`.
The archive contains source copies, the patch, full check log, the baseline
and final synthetic runs, and the latest private Thunar captures.

The audit separately found that live pointer projections discard SHAPE input
regions. [t064](../plans/queue-11-parallel-production-readiness.md#t064) records
that candidate follow-up, and the compatibility matrix now limits its claim
to the wire and direct-layer evidence. It is not folded into t061.

Install the combined candidate and accept one ordinary Thunar session: open
menus and submenus, interact with the sidebar, dismiss a popup, switch to Kitty,
and reuse a dialog. Text, backgrounds and edges must remain complete; dismissed
surfaces must neither linger nor take clicks. t061 closes only after that
installed observation, not after the headless probe.

## Connections

- [Brave watchdog investigation](h0vxis10-brave-gpu-watchdog-repeats-during-live-use.md)
  owns browser dumps, waiting-state samples, and frame-timing probes.
- [Pointer queries](knjco01f-pointer-queries-must-share-admitted-namespace-state.md)
  now return nonzero live state; pixel correctness remains separate.
- [Compatibility matrix](../../x11-compatibility-matrix.md) distinguishes startup,
  RENDER resource support, alpha limitations, and actual visual acceptance.
