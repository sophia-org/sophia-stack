---
id: legacy-active-0432
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering"]
---
# 2026-08-15: XRGB alpha is not transparency on the sharp mirror path

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 13048–13071. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The physical run of `c2460938` proved that both heads submitted and retired the
  same CPU generations, DP-2 selected the sharp 0.75 reduction without fallback,
  and native teardown was clean. The operator nevertheless saw only the blue
  frame and hardware cursor on DP-2; its terminal interior was black.
- The sharp shader unconditionally constrained reconstructed RGB by sampled
  alpha. X11's XRGB buffers encode colors such as white as `0x00ffffff`: the
  fourth byte is padding, not transparency. Exact sampling happened to remain
  visible because its opaque draw disabled blending, while the sharp program
  converted every XRGB color to black. Frame checksums and projected damage
  describe logical identity and ownership, so they could not expose this raster
  error.
- Textured composition now carries an explicit opaque-versus-premultiplied alpha
  mode. Both normal and sharp programs force alpha to one for XRGB before layer
  opacity and retain `rgb <= alpha` reconstruction clamping only for ARGB. This
  also corrects fractional-opacity XRGB blending instead of limiting the repair
  to the observed full-opacity path.
- The physical gate now enables final-region readback and requires substantial
  gray/white terminal coverage on both native-size compositions, with a bounded
  DP-2-to-DP-1 coverage ratio. Sampling evidence records the alpha mode, and
  mutation tests reject a blue-frame-only secondary head. A new clean signed
  physical run remains required.

<!-- END IMPORTED BODY -->
