---
id: legacy-active-0427
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "rendering"]
---
# 2026-08-15: core fixed text and CopyArea now carry xterm pixels losslessly

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12913–12953. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The prior text path synthesized a partial uppercase 5x7 alphabet inside an
  8x12 cell and discarded PolyText item structure. That could not prove mixed-
  case terminal output, font shifts, or scrolling correctness. Sophia now
  vendors all 256 glyphs from X.Org's `6x13-ISO8859-1.pcf` data. The upstream
  `font-misc-misc` license identifies this font as public domain: “Public domain
  font. Share and enjoy.” Metrics are consistently 6-pixel advance, 11-pixel
  ascent, and 2-pixel descent in rasterization and QueryFont/ListFontsWithInfo.
- OpenFont accepts `fixed`, `6x13`, the canonical XLFD, and the core `cursor`
  compatibility name; unknown names fail with BadName. The real xterm proof also
  exposed its standard startup open of `nil2` for tiny-font/icon slots, so that
  compatibility name retains the fixed face without importing a host-font
  dependency. GCs resolve and retain a face at creation/change time, so closing
  the font XID cannot invalidate later draws. QueryFont accepts FONTABLE GCs.
  PolyText8 retains signed deltas and request-scoped, MSB-first font shifts;
  malformed items fail length validation and missing shift fonts report BadFont.
- ImageText8 now forces GXcopy/Solid image-string semantics while preserving GC
  plane mask and clipping, fills the full cell background, and draws exact 6x13
  foreground glyphs into either window or pixmap backing. CopyArea validates
  both drawables and the GC, checks all depths, snapshots overlapping sources,
  clips source/destination together, and publishes only the actual destination
  damage. Pixmap PutImage and text no longer succeed without backing pixels.
- The real-xterm render proof pins `-fn 6x13`, white foreground, black
  background, disabled cursor blink, and an eight-row terminal workload that
  prints twelve mixed-case markers. Passing requires four exact white-on-black
  adjacent final rows in a currently live per-surface buffer plus accepted
  same-surface CPU updates in ImageText8→CopyArea→ImageText8 order. Historical or
  superseded buffers, auxiliary surfaces, no-op copies, scattered rows, and
  unordered opcode presence cannot satisfy the proof. Core
  GraphicsExpose/NoExpose event generation is deliberately
  deferred; xterm's pixel scroll path and bounded authority ownership do not
  depend on that optional event breadth.
- Final core-resource review found that fonts, pixmaps, and GCs could overwrite a
  different live resource kind with the same XID even though CreateWindow
  already rejected such collisions. Core creation paths now consult one XID
  namespace, and the shared resource table rejects replacement as a second
  defense. Request-specific failures report BadPixmap, BadGC,
  BadDrawable, or BadValue instead of collapsing into BadWindow. Regressions
  prove collisions preserve the original window, GC, or font.

<!-- END IMPORTED BODY -->
