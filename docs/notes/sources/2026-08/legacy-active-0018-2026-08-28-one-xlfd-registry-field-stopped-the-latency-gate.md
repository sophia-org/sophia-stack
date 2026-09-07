---
id: legacy-active-0018
date: 2026-08-28
recorded_date: 2026-08-28
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-28: one XLFD registry field stopped the latency gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 605–635. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first p99 latency run produced one sample and stopped. The session
  reported that its startup budget elapsed before a focused terminal frame was
  ready for the input proof, and refused two X requests:
  `by_opcode=[45/0/15x1 47/0/7x1]` -- `OpenFont` answered `BadName`, then
  `QueryFont` answered `BadFont` for the font that never opened. The injector
  was healthy and reported `stopped before injection`; the timeout the operator
  saw was the consequence, not the cause.
- `XFontFace::from_name` accepted the 6x13 face as `fixed`, `6x13`, `nil2`,
  `cursor`, and its canonical `iso8859-1` XLFD. xterm in UTF-8 mode asks for
  the same face spelled `iso10646-1`, which is what
  `/usr/share/X11/app-defaults/XTerm` sets as `*VT100.utf8Fonts.font`, and a
  UTF-8 locale makes that the default. The trailing XLFD fields are a charset
  registry and encoding, not a typeface, so the refusal rejected the terminal
  rather than an unsupported font.
- It bit only this harness because the mirror gate passes `-fn 6x13`
  explicitly while the latency harness takes the CLI's default xterm arguments,
  which carry no `-fn`. Every other physical gate now runs Kitty, so nothing
  else exercised the path. The harness last succeeded on 2026-07-31, and the
  installed xterm is 410; the exact date the default flipped is not established
  and does not change the repair.
- Accepting the Unicode spelling is not a claim to cover the Unicode
  repertoire. Sophia rasterizes one fixed face under either name, and a glyph
  outside it falls back exactly as it already did. A regression opens every
  accepted spelling through the real dispatch and requires `QueryFont` to
  answer for each -- a name that opened but could not be queried would fail the
  client one request later, which is how this presented -- and a companion
  refuses `10x20`, so a second registry does not become any name at all.
  Removing the fix fails the regression on the exact XLFD xterm sent.

<!-- END IMPORTED BODY -->
