---
id: legacy-active-0487
date: 2026-08-21
recorded_date: 2026-08-21
date_basis: first-heading-commit
date_commit: 3b1379d88dcf86e5a603945a1809f88e66103e1d
committed_at: 2026-08-21T15:07:50-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# Filtering in light, and the filter that would have undone it

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14833–14897. The heading has no date. Its first recorded addition is commit
`3b1379d88dcf86e5a603945a1809f88e66103e1d` (2026-08-21T15:07:50-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The change itself is small: decode each tap before weighting it, re-encode the
sum once at the end. Gamma 2.0 rather than the sRGB curve, matching what
`software/raster_replay.rs` chose for the CPU path and for its stated reason --
a squared approximation is cheap enough to apply to all sixteen taps of a
bicubic and keeps one transfer function in the tree instead of two that would
eventually have to be reconciled.

Two details are the ones that would have been got wrong quietly.

The first is premultiplied alpha. A stored premultiplied value is `E(c)*a`, and
squaring it gives `E(c)^2 * a^2` -- the coverage squared along with the colour.
It has to be unpremultiplied across the decode. Under gamma 2.0 both directions
collapse to something cheap, `v*v/a` in and `sqrt(L*a)` out, which is a small
mercy given it happens sixteen times per fragment. Alpha itself is never
transformed: it is coverage, not light. Getting that wrong would have left every
partially transparent edge wrong by exactly the factor the colour was corrected
by, and the image would have gone on looking plausible.

The second is the sampler. The kernel gathers its own 4x4 footprint at texel
centres, so it needs the texels as stored; leaving `GL_LINEAR` on would have had
the hardware blend them in gamma-encoded space before the shader ever ran. That
failure is invisible in the evidence -- the draw still reports
`sharp_downscale status=active` and produces a partly uncorrected frame. The
filter was chosen at the call site from the requested sampling and the program
inside the draw from whether it had compiled, two derivations with nothing
forcing agreement, which is the defect this codebase keeps meeting. They are one
function now, and the fallback is the only place `LINEAR` survives.

Widening to both directions turned out to cost nothing. Catmull-Rom is an
interpolating kernel -- it passes through its samples -- so it is the textbook
bicubic upsample as well as a reduction filter. One program serves both, which
means one place where light is decoded and re-encoded, and the roadmap item
asking for a real upscale kernel stopped being a prerequisite: Lanczos-2 and
FSR 1 are now a question of whether bicubic is sharp enough, to be asked after
the colour space is right rather than before.

The naming had to move with it. Renaming `LinearUpscale` to `SharpUpscale` left
the degraded path without a name, and looking for one surfaced an existing
conflation: the fallback reported itself as an upscale, so `linear_upscale_draws`
was incremented both by a real enlargement and by a degraded reduction, while a
second counter tallied the fallbacks beside it. Two numbers, neither of which
meant one thing. There is a `LinearFallback` variant now and a single
`linear_fallback_draws`, and any value above zero says the reconstruction shader
is not running.

Three negative controls, because this session already paid for the lesson.
Reverting the tap to weight encoded bytes fails the test that pins the decode.
Setting the reconstruction filter back to `LINEAR` fails the test that counts
which arms keep hardware filtering. Removing the guard on the encode's input
fails the test that pins the clamp ahead of the square root. The arithmetic
test alone would have passed all three, because it mirrors the shader in Rust
rather than running it -- which is exactly the shape of test that proved nothing
four commits ago.

One thing the fixture taught me mid-write. An assertion that saturated pixels do
not move failed on a pixel at 0.00011, which lifts to 0.0105 under the square
root. That is not a saturated pixel drifting; it is a faint Catmull-Rom ring
being corrected, and correcting it is the point. The band was sloppy, not the
code. Worth recording because gamma 2.0 amplifies dim values hard -- a hundredfold
at the bottom of the range -- so ringing that was invisible in encoded space is
visible now. That is what filtering in light means, not a regression, but it is
the first place to look if halos are reported.

<!-- END IMPORTED BODY -->
