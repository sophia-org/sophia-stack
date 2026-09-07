---
id: legacy-active-0537
date: 2026-08-26
recorded_date: 2026-08-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-08-26: the reservation model was already written

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16503–16556. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The work-area reservation coordinator needed a lifecycle model before its
records could freeze. `validation/tla/ShellWorkAreaCoordination.tla` already
held one, written against the target architecture and carrying the exact
invariants the coordinator owes: a presented bundle is coherent, was ready,
matches generation and epoch exactly, and a rejected attempt preserves the
presented triple. It checks 12,278 distinct states and has never had an
implementation. So the model cost nothing and the implementation became its
refinement rather than its sequel.

Three of its properties decided the shape directly. A claim is prepared at
admission and presented only at commit, so `active_bands` is empty between the
two and a prepared claim reduces nothing -- the offline proof asserts that gap,
because a coordinator that reduced at admission would look correct in every
screenshot and be wrong in exactly the window the model forbids. Commit takes
the exact prepared identity and preserves the presented claim otherwise, which
is the rejected-attempt invariant in code. And disconnect burns only the
in-flight claim: a presented reservation is retained with the inert pixels,
since growing the work area while no shell can re-present is the half-new
desktop the model rules out. Withdrawal is therefore not a message. It is a
later candidate carrying no reservation, committed through the same path, so
there is no release that can be lost on its own.

The claim rides on the candidate rather than a separate request stream for the
same reason: the candidate already owns both visuals and reservation, and a
second stream would need its own ordering against the first.

The wire change reuses reserved bytes rather than adding a message. Candidate
byte 33 was a reserved zero and is now the reservation edge; bytes 38..40 were
reserved and now carry thickness. Zero in both still means no reservation, so
every frame that was valid before decodes to the same record. That reuse cost
one test: `shell_v1_rejects_reserved_and_unknown_envelope_fields` poked byte 33
and expected `ReservedNonZero`. The frame is still refused, now as an edge with
no thickness, which is the sharper reason. The assertion moved to the per-entry
reserved field so the check keeps a live subject instead of quietly becoming a
test of something else -- a reserved-field test whose field is no longer
reserved passes for the wrong reason, and that is worse than failing.

Engine converts the edge claim into the same root-relative band the reducer
already consumes for X-side struts, so one reduction subtracts both authorities
and a shell bar composes with an xmobar strut without either knowing about the
other. Depth is measured from the root's edge through the output, which matters
only when an output does not sit flush against the root -- a shorter head
claiming 28px at the bottom of a taller root needs a 388px band, and a
coordinator that wrote 28 there would reserve empty space beside the display
instead of the strip under the bar.

Not yet wired into the live owner loop: both `reduce_output_work_areas` callers
pass an empty band slice today. The substrate, its reduction, and an offline
proof driving the real Nim shell through the real coordinator are complete; a
live session claiming a bar is the step that makes it production, and the
physical archive remains its gate.

<!-- END IMPORTED BODY -->
