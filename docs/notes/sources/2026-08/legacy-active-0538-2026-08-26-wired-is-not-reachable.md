---
id: legacy-active-0538
date: 2026-08-26
recorded_date: 2026-08-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-26: wired is not reachable

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16557–16599. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The reservation coordinator was complete, proven offline against the real Nim
shell, and unreachable. `SOPHIA_SHELL_BAR_THICKNESS` selected the strip, and the
only writer of that variable in the tree was
`shell_descriptor_conformance_host.rs` -- the offline harness that proves the
path. `LiveMetadataShell::start` passed the socket and nothing else, so a live
shell always read an unset thickness and claimed nothing. Every gate passed
because the harness set the variable it was testing, and `todo.md` recorded the
variable as though a session could set it too.

A physical run against that tree would have proved the switcher again, raised no
strip, and cost a rig session to discover it. The failure mode is worth naming
because nothing was broken: the substrate was right, its tests were honest about
what they exercised, and the gap sat entirely in the space between the harness's
environment and the session's.

The depth is now `shell { panel N; }`. `panel` was already an allowlisted shell
key with no validation, no accessor, and no reader, so a profile could ask for a
panel and be silently ignored; it now means one thing and is refused where it
cannot be honoured. The ceiling has two owners -- `sophia-config` depends on
nothing else in the stack and cannot read the wire's maximum -- so it carries its
own copy and a test in `sophia-cli`, which sees both crates, fails if they drift.

Two claims from the previous entry were also wrong. The strip is not a bar. One
shell connection carries one candidate stream and one visible state, and the
reservation rides on the candidate, so the claim lives exactly as long as the
switcher does. A persistent panel needs a second shell role, which is separate
work; what exists is a switcher that claims work area while visible, and the
guide now says so. And the shell's death does not restore the work area: the
presented claim is retained, because the model forbids growing the area while
nothing can present into the strip. The guide had been drafted to prove a
restore that would have been a defect, and reading the implementation's actual
emissions before writing the waits is what caught it.

The verifier gained the reservation counts it had never had -- it could
previously accept a run that raised a claim and lost it -- anchored on the
shell's own restart line rather than the WM policy restart much earlier, which
is a distinction the first draft got wrong and the matcher fixture caught in
seconds. Five negative cases now drop or corrupt each new line and require the
verifier to refuse the run, because a check that cannot fail is worse than no
check: it reads as coverage.

<!-- END IMPORTED BODY -->
