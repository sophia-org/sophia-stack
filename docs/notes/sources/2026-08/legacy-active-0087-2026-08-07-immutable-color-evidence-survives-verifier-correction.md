---
id: legacy-active-0087
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-07: Immutable color evidence survives verifier correction

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2917–2961. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Automatic TrueColor attempt `0001` exited normally and proved the real Kitty
DMA-BUF path and the X Authority's exact core color round trip, but correctly
failed promotion. The proof client had placed its palette at global x=2800 on
output 2. The present classical-WM compatibility path builds its retained
client scene for output 1; output 2 owns an independent startup baseline but
does not yet receive active client projection. The renderer therefore rejected
the global palette rectangle as outside its output-local composition target,
and no later output-2 frame could retire.

The corrected gate keeps both real clients inside output 1, where their final
regions can be correlated with actual native submissions and retirements, and
continues to require output 2's nonzero startup baseline. This proves TrueColor
through the implemented boundary without pretending to close active
cross-output projection. The attempt also exposed a diagnostic-label defect:
the session banner still prints its legacy `terminal=xterm` constant even when
Kitty is the configured application. The verifier now identifies Kitty through
its selected PresentedBuffer/DMA-BUF evidence and immutable runtime identity;
changing that banner belongs to a separate schema correction.

Corrected attempt `0002` from commit `c62eabd6` then recorded the exact palette
populations, chromatic Kitty DMA-BUF region, causally next output-1 submissions
and retirements, both-output startup, normal logout, clean ownership drain, and
exact TTY restoration. Its automatic verifier nevertheless rejected the run
because one regular expression assumed `outputs_ready` preceded `presented` in
a structured startup record. Both fields were present with the required values
in the opposite order.

The verifier now parses those fields by name. The run-set gate may also
re-adjudicate an immutable exit-zero `reason=session_verification` record under
the current verifier. It does not rewrite the archive and does not admit a
session-exit failure, another failure reason, a checksum change, or evidence
that fails any current semantic check. Attempt `0002` consequently closes the
physical TrueColor gate without asking the operator to replay an already valid
physical sequence.

The operator subsequently ran the corrected installed gate once more. Attempt
`0003` from commit `883666a2` passed at capture time and under the run-set gate
with `reverified=0`. It independently reproduces the exact palette, Kitty
DMA-BUF, two-output startup, native retirement, logout, ownership drain, and
TTY-recovery evidence. Attempt `0003` is therefore the canonical promotion
record; attempts `0001` and `0002` remain immutable diagnostics of the
cross-output placement and verifier-ordering defects.

<!-- END IMPORTED BODY -->
