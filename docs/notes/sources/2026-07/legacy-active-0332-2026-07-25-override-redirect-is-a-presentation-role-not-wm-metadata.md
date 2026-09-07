---
id: legacy-active-0332
date: 2026-07-25
recorded_date: 2026-07-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-07-25: Override-Redirect Is A Presentation Role, Not WM Metadata

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10402–10476. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Unmodified xmobar creates its bar with `CWOverrideRedirect`, requests top
geometry, publishes dock/strut properties, and maps the window. Sophia
previously discarded that attribute, reported it as false in replies and
events, and admitted every rendered surface to the blind xmonad policy stream.
That would tile a status bar as an ordinary application even if all of
xmobar's drawing requests succeeded.

The X frontend now owns the exact override-redirect bit in its passive window
record and returns it through core X11 semantics. Across the authority
boundary it reduces the bit to `SurfacePresentationRole::ClientPositioned`.
The live Engine layout retains client geometry, composition, hit testing, and
top presentation for that role, while withholding the surface from WM
management. No XID, class, title, atom, dock type, or application identity is
sent to xmonad.

The xmonad TTY launcher can now discover an installed xmobar or an executable
built from the operator-selected source checkout, then supervise it as a
secondary X client with a deterministic local config. Xmobar and xmonad remain
unmodified compatibility clients. This first slice intentionally overlays the
bar: interpreting `_NET_WM_STRUT_PARTIAL` as protocol-neutral output
reservations is the next work-area step and requires retained physical
evidence before promotion.

The first real-client trace exposed two generic drawing gaps rather than an
xmobar-specific condition. Xmobar's Cairo backend uses MIT-SHM `GetImage` and
`PutImage` against an offscreen pixmap, followed by core `CopyArea` into its
window. Sophia initially omitted `GetImage` and then acknowledged pixmap
`PutImage` while discarding its bytes, so the final window transaction was
blank. Pixmap dimensions and software pixels are now passive authority data;
upload, readback, and copy use the same drawable buffer path as other software
clients. The unmodified xmobar 0.51.1 smoke subsequently completed 163 requests
across 28 opcodes with two committed transactions, 8,967 nonzero pixel bytes,
and `first_error=none`. The source checkouts for xmobar and xmonad were not
modified.

The first physical session then proved that xmobar stayed supervised, retained
its client-positioned role, and continued publishing nonzero CPU buffers, but
the bar was not visible above Kitty. The defect was in the generic mixed
renderer boundary: all CPU surfaces were flattened into one output-sized
background before the current and retained DMA-BUF layers were appended.
Flattening discarded the per-surface ordering needed for a CPU overlay, so the
later Kitty layer covered valid bar pixels.

Mixed presentation now snapshots CPU surfaces as passive
surface/geometry/buffer records and reduces the Engine presentation order into
one interleaved CPU/DMA-BUF layer sequence. The scheduler owns that immutable
snapshot with each queued Present; the renderer remains unaware of xmobar,
Kitty, X atoms, or window roles. Reducer regressions cover both a CPU overlay
above GPU clients and an ordinary CPU client below the current GPU surface.
The xmobar smoke was also strengthened to require six committed transactions
and nonzero pixels in the newest redraw; the retained run completed 215
requests across 28 opcodes with 25,929 nonzero bytes and no protocol error.
Corrected physical visibility remains pending, and neither client source tree
was modified.

The next physical run rendered the status bar, confirming the corrected
surface order, but Kitty never became visible. Kitty did start, registered
three DMA-BUFs and fences, submitted 19 Presents, and Super-Enter committed a
`LaunchTerminal` action. All 15 attempted mixed native submissions failed
before KMS with `ScanoutExportFailed`, after which the startup focus-control
gate timed out.

The remaining defect was a stale renderer invariant. Its persistent CPU
texture was allocated at output size and `draw_cpu_layer` rejected every other
extent because the previous mixed seam supplied only a flattened full-output
background. A correctly represented status bar is instead a narrow CPU layer.
The native pipeline now tracks the texture's allocated extent, reallocates it
only when the next CPU layer differs, and uses the existing sub-image fast path
for same-sized redraws. This supports arbitrary application-agnostic CPU
layers without allocating a new texture for each bar update. A passive reducer
test locks the reallocate-versus-update decision; corrected physical
bar-plus-Kitty presentation remains pending.

<!-- END IMPORTED BODY -->
