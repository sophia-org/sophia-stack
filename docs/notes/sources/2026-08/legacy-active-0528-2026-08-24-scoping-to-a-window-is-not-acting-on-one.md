---
id: legacy-active-0528
date: 2026-08-24
recorded_date: 2026-08-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-24: scoping to a window is not acting on one

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16181–16230. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The next signed run passed the entire guide, proof phrase included, and failed at the
completion check on 24 X protocol errors. The first was `BadWindow` on XFixes
`SelectSelectionInput`, which no other site can produce: that handler overwrites the
minor opcode with its own, so the code identifies itself.

Watching a selection is scoped to a window rather than performed upon one, and every
toolkit scopes it to the root. The root here is synthetic -- never inserted into the
resource table, never given a window record, recognised only by scattered literal
comparisons -- so the shared window lookup cannot find it and refuses it. The drawable
half of that lookup already admits the root for exactly this reason; the window half
never did, which left each caller to remember its own guard. Three had not: selection
watching, Present event selection, and setting the root cursor. The guard they should
have used already existed, correct and misnamed as a grab-specific helper.

The exemption stays out of the shared lookup. Reparenting, destroying, and creating a
GLX drawable from the root are errors, and folding it in would quietly make those
wrong. Several further sites are left alone and recorded rather than fixed: presenting
to the root, validating a requestor inside an event body, which X11 never does at all,
and the runtime-layer window operations.

**This suppresses the error and delivers nothing.** `XFixesSelectionNotify` is not
implemented anywhere in this tree: no subscription table, no event variant, and the
event code appears only in the `QueryExtension` advertisement. Selection-owner changes
notify only the previous owner, through core `SelectionClear`. A client watching
CLIPBOARD still learns nothing. It is an improvement rather than a new silence -- that
client previously received the error and no events -- and the request is now
conformant, but real selection watching is an open feature, not something this closed.

The second defect is why the first cost a run to find. The session counted protocol
errors and retained only the first, so three physical runs each surfaced exactly one
cause. Errors are now tallied per request and reported one line per opcode, bounded,
with the count of what a reset dropped, and reported before the fatal return, which
otherwise precedes every schema line and leaves a failed run printing one string.

The tally carries opcodes and counts only. The resource id stays in the frontend:
`docs/architecture.md` rules that default diagnostics may not contain raw XIDs, and
this record is interpolated into an archived evidence file at default verbosity. The
batch fields that do carry XIDs are capability keys for descriptor adoption that are
never logged -- the opposite profile, not a precedent. The frontend traces the id at
debug level instead, where it already owns the value and the rule permits it.

The guide's per-step keypress check from the previous entry is reverted. Those counts
are cumulative and several actions are asked for twice, so a count above a step's
expectation is what a later legitimate press looks like from an earlier step. It broke
the physical matcher self-test -- precisely the case it could not tell apart. Catching
an extra keypress needs the run's final totals; that remains open, and the bounded
browser wait is kept.

<!-- END IMPORTED BODY -->
