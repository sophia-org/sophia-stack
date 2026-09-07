---
id: legacy-active-0091
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-08-07: Terminal roles do not imply Kitty command-line syntax

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3031–3054. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Installed xterm attempt `xterm-runs/0001` proved the new profile-specific
ledger and exact executable identity, then failed before X11 admission. The
session registry launched `/usr/sbin/xterm` with Kitty's `--config NONE` and
`--override` arguments. Xterm rejected the first option, and Sophia correctly
failed startup when no primary frame arrived. Renderer, X Authority, xmonad,
and work-area code were never reached.

The shell launcher had treated the protocol-neutral `terminal` application
role as if it also selected Kitty's command-line grammar. Terminal adaptation
is now explicit. A small passive helper resolves only the supported `kitty`
and `xterm` kinds, appends their disjoint base and title arguments, and rejects
an unknown kind before takeover. The installed xterm wrapper pins its kind;
Firefox proof profiles continue to require Kitty because their checkpoint
scripts use Kitty's executable-tail convention.

The regression requires xterm's `-cm`, `-dc`, and `-title` spelling, rejects
every Kitty-only option in the xterm vector, and asks a real xterm binary to
parse that vector when available. Packaging carries the adapter beside the
session launcher. This is installed-launch policy, not Engine or X Authority
state, so it does not change the transition model. A new immutable install and
physical attempt remain required.

<!-- END IMPORTED BODY -->
