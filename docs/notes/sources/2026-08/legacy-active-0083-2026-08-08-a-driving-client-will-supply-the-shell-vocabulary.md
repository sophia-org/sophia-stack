---
id: legacy-active-0083
date: 2026-08-08
recorded_date: 2026-08-08
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell", "architecture"]
---
# 2026-08-08: A driving client will supply the shell vocabulary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2830–2871. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

`sophia_shell_v1` waits on retained shell workflows to establish its smallest
useful display-list, hit-target, presentation-data, and action vocabulary.
Nothing supplies that today. xmobar is the only shell-like client with retained
evidence, and it is static text with no hit targets, popups, or animation.
Specifying against it would produce an interface too narrow to carry a desktop.

The decision is to derive the vocabulary from a complete external shell rather
than from first principles, and to select that shell by one criterion: it must
already keep a retained tree of typed drawing primitives, because that is the
artifact a display-list protocol standardizes. Noctalia qualifies. Its
`src/render/scene/` holds rectangle, text, glyph, image, effect, and
hit-area nodes, and its bindings enumerate 25 protocols a real shell needs.
A first comparison against `docs/compositor-graphics.md` shows the proposed
vocabulary already covers eight of its node kinds and omits nine, of which four
are per-widget visual novelty that should stay client-rasterized.

Xfce was considered and assigned elsewhere. It draws through GTK3 and Cairo, so
it can never emit a display list and cannot falsify any decision in this
interface. It is strong evidence for X11 compatibility completeness instead —
EWMH, work-area reservation, and tray/XEmbed — and belongs in the classical
compatibility profiles beside i3, dwm, and qtile.

Ordering is sequential: `sophia_wm_v1` freezes at 13.4 before shell modeling
starts. Hagia has already proved the specification pipeline end to end in a
third language, and repeating that machinery concurrently on a second interface
would reopen shared framing decisions while they need to settle.

One question this raised is sequencing-critical and remains open. A shell that
rasterizes its own novelty must upload textures, but a frame is capped at
64 KiB with one transfer in flight over a bytes-only wire. A 1920x40 ARGB bar
is roughly 307 KiB, and continuously animated content is not expressible at
all. Content-addressed cached textures may be sufficient; a shell-role
descriptor channel may not be avoidable. The envelope is role-neutral and each
role negotiates its own family, so this is implementation coupling in
`sophia-runtime` rather than wire-format lock-in — but the answer belongs
before the freeze, not after. Recorded as an analysis item in 13.2.

Full reasoning, capability tables, and the vocabulary delta are in
`docs/sophia-shell-v1-direction.md`.

<!-- END IMPORTED BODY -->
