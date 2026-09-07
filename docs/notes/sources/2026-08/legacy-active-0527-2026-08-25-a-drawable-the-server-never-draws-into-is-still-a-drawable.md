---
id: legacy-active-0527
date: 2026-08-25
recorded_date: 2026-08-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-25: a drawable the server never draws into is still a drawable

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16118–16180. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The run after the per-opcode tally landed named three causes at once instead of one,
which is what the tally was for: GLX `CreatePbuffer` and `DestroyPbuffer` refused six
times each, and six core `GetGeometry` refusals beside them at a non-zero resource id.
Every earlier fix held -- the guide completed, the proof phrase passed, and no XFixes
error appeared.

Those eighteen errors are one sequence, and the counts are the proof rather than the
inference. Mesa allocates the pbuffer id itself, asks the server to create it, then
builds its DRI3 drawable naming that same id; the drawable's first question is its
geometry, and a refusal there is what prints "failed to create drawable". The client
then destroys the pbuffer it could not use and falls back to software. Six attempts,
three requests each.

Sophia advertises GLX 1.4 and implemented five of the twelve requests GLX 1.3
introduced -- the framebuffer-config and window half, refusing the pbuffer half.
Neither way out existed. Claiming an older version is what broke Kitty before, and
the pbuffer capability bit was already withheld while the client asked anyway,
because it picks a configuration without requiring the bit and creates a pbuffer from
it. The compatibility matrix rules that an unsupported request stays a client-visible
error, and the probe exemptions beside it are narrow on purpose: they cover an error
that is the correct answer about something which does not exist, not an answer about
something unimplemented. Widening them is the broad whitelisting those fixtures were
written to prevent.

Bookkeeping is the honest implementation, not a shortcut. GLX owns no pixels in this
tree: a client allocates its own buffers and imports them through DRI3, so a GLX
window is already a record with no server storage behind it. A pbuffer differs only
in having no X window to borrow an extent from, and it is never presented, so its
path is a subset of the window path Kitty and glxgears already prove. Because it is
its own X drawable, core `GetGeometry` has to answer for it, which is the request the
client actually fails on; its depth comes from its configuration, since there is no
window to read one from. Core drawing still refuses it, because a surface with no
storage is not a drawing target.

Two facts that were derived twice now have one owner. The mapping from a
framebuffer configuration to its visual and depth lived in the catalog and again
inline in the drawable resolver, and a pbuffer's depth plus the size Sophia will
record would have made four copies of two facts. The maximum is one constant,
enforced as a refusal and published as the advertised maximum, so the two cannot
drift. `GLX_PIXMAP_BIT` is withdrawn in the same change: it promised four requests
none of which exist, which is the same advertise-then-refuse this entry is about, one
layer down. A test now ties the advertised drawable types to the implemented ones.

**What this does not establish.** The plan was to prove the Mesa and DRI3 half offline
before spending a physical run, using mesa-demos' `pbdemo`, which is a stronger probe
than a browser's initialization pbuffer. The smoke hangs -- and hangs identically with
`pbinfo`, which merely queries and exits, so the fault is in how the probe harness was
wired rather than in the pbuffer path. It was removed rather than shipped hanging, and
debugging that harness is its own task. So the physical run remains the first real
test of whether a GL client gets further, and three things stay unknown from source:
whether Mesa selects Present input on a pbuffer, whether the browser's GL layer gets
past initialization at all, and how a GPU process that survives initialization behaves
against a compositor with no server-side GL.

The risk of getting further is bounded rather than open. Every remaining refusal is an
X error rather than a wait, and the guide's browser step is already bounded, so a GPU
process that hangs fails legibly instead of stalling the rig. A second cause is
already identified and deliberately out of scope: ANGLE's fallback display type needs
`GLX_EXT_create_context_es_profile`, which is not advertised, and which is why that
path fails eighteen times in the same log.

<!-- END IMPORTED BODY -->
