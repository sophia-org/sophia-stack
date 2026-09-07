---
id: legacy-active-0526
date: 2026-08-25
recorded_date: 2026-08-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-25: creating a drawable nothing may use moves the failure

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16062–16117. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The pbuffer change shipped without its DRI3 half, which the plan had sequenced last
and conditional on an offline probe. The probe then did not work, and it shipped
anyway. The condition was the whole point: once the probe could not speak, the choice
was to include the DRI3 half or hold the change, and neither was taken.

On the rig the browser got further than it ever had. `eglInitialize` failed eighteen
times in the previous run and not once in this one -- GL now initialised, reached
DRI3, named the pbuffer in its buffer import, took `BadWindow` seven times, lost its X
connection and died. Three GPU processes later no frame had landed and the surface
admission timed out. Strictly further along and visibly worse: a crash loop where
there had been a clean fallback to software.

Two readings of that log were wrong on the first pass and are worth recording, since
both are easy to repeat. The eight `exit_before_admission` lines were not the browser
giving up; they were eight further `Super+B` presses landing on an already-running
Helium, which says `Opening in existing browser session.` and exits zero. And the
regression was not that the browser failed earlier, it was that it failed later.

A pbuffer lives in the GLX drawable table and never enters the resource table, so
every validator that looks a resource up misses it. Seven sites reach a drawable a
client may have allocated for itself -- four DRI3 imports, the supported-modifier
query, and the two Present queries Mesa's loader makes for every drawable it
initialises -- and they used three different validators between them. They now share
one named accessor over the drawable resolver. Core drawing keeps the narrow one: a
surface with no storage is not a drawing target, and the guard pinning that boundary
passes unchanged beside the test asserting the opposite for imports.

**The probe harness was the real lesson.** It had hung, and the hang had nothing to do
with what it was probing. `accept` had no deadline, so the idle timeout covered a
client that connected and went quiet but not one that never connected at all; the
harness then joined that thread whenever the client exited on its own; and the demo
being driven took positional arguments it had not been given, so it printed its usage
and exited before opening the display. Three faults compounding into silence. With
`accept` bounded and the join made conditional on a connection, the harness reports in
seconds -- and the first thing it reported was the same DRI3 failure a rig session had
just spent an evening finding. Reproducing a physical failure offline in seconds is
worth more than the fix it was blocking.

What remains is one refusal the client tolerates: DRI3 minor 8, `BuffersFromPixmap`,
is not decoded while Sophia advertises DRI3 1.2, which includes it. Implementing it
means handing back plane descriptors the frontend deliberately does not retain, which
crosses the renderer import boundary and is its own change. It is recorded here rather
than folded in, and a physical run will now name it rather than costing a session to
find.

The GLX 1.3 surface is otherwise complete: `QueryContext` answers from the
configuration a context was made with, and `ChangeDrawableAttributes` validates its
drawable and records nothing, because the only attribute it sets selects clobber
events Sophia never sends. GLX 1.4's own delta over 1.3 is multisampling, and the
sample attributes were already advertised as zero, so finishing 1.3 is what makes the
version claim honest rather than any version change. The indirect-GLX requests stay
excluded by architecture rather than by omission, and a client id can no longer shadow
a live GLX drawable, which the creation path already refused in the other direction.

<!-- END IMPORTED BODY -->
