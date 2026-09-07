---
id: queue-02
date: 2026-09-06
kind: plan
tags: [plan, milestone]
---
# CP-14.3 — Development-session readiness and Milestone 14 closure

This plan retains the scope, constraints, and task details from the roadmap
cutover. Task status and order live only in [todo.md](../../../todo.md)
and the [monthly completion history](../../../done.md). Follow the
[work-tracking contract](../../work-tracking.md).
Historical candidate identities in the details require revalidation before use.

[Parent scope](queue-01-critical-path.md).


Milestone 14 now targets a reliable personal development session: terminal,
Firefox, clipboard, layouts/tabs, two monitors, dependable input, recovery, and
logout. Readiness is demonstrated through working tasks and resolved failures,
with no consecutive-clean-day requirement. CP-14.2 is explicitly deferred and
incomplete; neither its 36-row matrix nor the optional soak gates this milestone.
The [decision and former queue](../sources/2026-09/legacy-active-0618-2026-09-04-milestone-14-retargeted-to-development-session-readiness.md)
retain the change in exit criteria. Work through these stages in order.


## t001

Accept [desktop composition](../../desktop-composition.md) in one normal
session: select the desktop from `sophia/desktop.kdl`, retain the current
Hagia/Narthex/Quickshell arrangement, and confirm WM reload does not replay
login applications. Native shortcut help was reported working in the
installed session on 2026-09-06; see the research log.



Previously completed evidence: [Implement the native application launcher through the generic revision-4 shell protocol: session-owned catalog and execution policy, Engine-owned input/GPU presentation, and independent Narthex…](../sources/2026-09/todo-cutover-completed.md#legacy-done-001).


## t002

Accept the installed Sophia/Hagia/Narthex launcher changes with
Super+Space through normal use: search, keyboard/click activation, Escape,
terminal entries and opening a third window. The desktop-composition and
native-launcher configuration is installed. The first login failed argument
validation because the selected `brave-origin` browser was unregistered; its
explicit core registration now passes installed-session validation. The next
login exposed duplicate nominal DRM modes in the output projection; the
projection and full-modeline selection fixes are installed. That release
presented both heads, then rejected frontend publication because the authority
snapshot lacks backend timing metadata. Release `3fc0ab14` completed installed
login on 2026-09-06: both outputs committed, startup ready, zero recovery
attempts, and revision-4 shell connected. Super+Space dispatched Ghostty;
Ghostty then failed during its MIT-SHM mask upload. The upload repair passes
an isolated Ghostty startup probe; installed interaction acceptance remains
pending. No live reload was performed.


## t003

Accept the client-failure repairs in normal use: Ghostty opens from the
launcher, and a Brave typing test leaves the desktop usable. Packed MIT-SHM
uploads are repaired. Normal sessions contain rejected/failed input deliveries
while proof sessions remain strict. The original Brave `RouteRejected` reason
is unresolved; new diagnostics must guide any follow-up. See the
[diagnosis](../sources/2026-09/legacy-active-0633-2026-09-06--ghostty-mask-uploads-and-browser-input-failure-containment.md).

[Ghostty startup was physically accepted](../milestones/2mb23diq-ghostty-launcher-startup-accepted-in-ordinary-use.md)
through Super+Space on installed `8921174c`. The Brave typing check remains
required; this observation does not complete all of t002 or t003.


Previously completed evidence: [Accept Kitty startup and Super+Enter on a replacement release.](../sources/2026-09/todo-cutover-completed.md#legacy-done-002).


## t004

Accept Hagia's maximized/fullscreen stacking repair in normal use.
Super+F enlarged the window under its later neighbor; Super+M's column sizing
worked. Hagia now orders expanded windows above ordinary placements. The
replacement WM has not been loaded into the physical session.


Previously completed evidence: [Separate normal desktop lifetime from application startup proofs.](../sources/2026-09/todo-cutover-completed.md#legacy-done-003).


## t005

Accept panel-only login after reinstalling Sophia and its launcher. The active
desktop profile now starts only `quickshell-panel`; Super+Enter retains its
terminal mapping. Ordinary Hagia startup no longer arms the proof-only
focused-application deadline. Headless sessions survive the old eight-second
boundary, including failed/background startup applications. The installed
binary and launcher still need replacement; do not validate the edited
profile against the old release again. Record the result in the
[maintained startup investigation](../investigations/startup-panel-only-startup-physical-acceptance.md).


## t006

Unblock GTK application startup with RENDER 0.6 transforms and filters.
On installed `f323323d`, Super+Space starts both Ghostty and Thunar, but each
exits on `SetPictureFilter` (opcode 144, minor 30, `BadRequest`) before
admission. Implement and test the sampling semantics and query surface before
advancing the advertised version; rerun both real clients afterward. The
earlier MIT-SHM fix remains valid. Launcher dispatch is working.



## Related scopes

- [1. Recover reliably](queue-03-1-recover-reliably.md)
- [2. Establish the live session](queue-04-2-establish-the-live-session.md)
- [3. Make failures diagnosable](queue-05-3-make-failures-diagnosable.md)
- [4. Exercise real development workflows](queue-06-4-exercise-real-development-workflows.md)
- [5. Close Milestone 14](queue-07-5-close-milestone-14.md)

**Result.** RENDER 0.6 implemented and advertised: `SetPictureTransform`,
`QueryFilters` and `SetPictureFilter` all answer, and the version constant
moved in the same commit the requests started working. Commits `8811fcf5`
(sampler groundwork), `2f0fb615` (the 0.6 surface), `38a15da5` (the GTK
probes).

**Evidence.** Thunar and mousepad both complete GTK's startup against a
headless authority with `first_error=none` across 146 requests, where the
live session ended them on major 144 minor 30. `x-authority-thunar-smoke` and
`x-authority-gtk3-smoke` are retained as regression probes.
`x-authority-render-smoke` reports `version=0.6`. Twenty-six dispatch tests
cover transform sampling (identity, translation, constant-divisor scale, a
diverging projective point), bilinear blending, the filter table's exact
bytes, and the refusal terms.

**Why refusing politely could not have worked.** Both toolkits sent a 0.6
request against a 0.5 advertisement. Their client libraries do not gate
`SetPictureFilter` on the version they are told, so honest versioning did not
protect the minor; the request had to answer.

**A prediction the measurement did not support.** The plan expected trapezoids
(minors 10-13) to be the next refusal, on yserver's field evidence that GTK
CSD shadows are drawn with them. Sophia's own traces show neither GTK3 client
sending RENDER at all in this configuration -- both take the GLX path -- so
the trapezoid family stays unimplemented and undemanded. It remains a standing
over-promise against the advertised base version, answering `BadImplementation`
to anything that does send one; if a trace ever shows that, it becomes a
measured task. Building it now would have been a survey of what a server
usually has, which is the thing this project keeps declining to do.

**Limits.** Filters honour nearest and bilinear; convolution is deliberately
not advertised, so a client that wants kernel work disables it cleanly.
Transforms are full projective. Gradients and `CreateSolidFill` (0.10) stay
absent -- cairo renders gradients client-side below 0.10 and only sends them to
a server that claims it. The probes are headless and admit no window, so they
prove startup and the absence of refusals, not pixels. **Physical acceptance is
not claimed here**: this task is `@development`, and seeing Thunar and Ghostty
open from Super+Space rides the `@physical` session tasks t001-t005.
