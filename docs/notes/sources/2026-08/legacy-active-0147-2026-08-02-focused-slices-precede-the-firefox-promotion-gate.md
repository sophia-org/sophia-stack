---
id: legacy-active-0147
date: 2026-08-02
recorded_date: 2026-08-02
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy", "validation"]
---
# 2026-08-02: focused slices precede the Firefox promotion gate

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4698–4721. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The combined physical Firefox workflow had become a poor debugging loop. A
late selection or lifecycle failure discarded several minutes of unrelated
manual interaction, and repeating the full sequence made operator timing part
of diagnosis. The long workflow remains the Milestone 10 promotion contract,
but it is no longer the first test for a localized change.

Two source-tree diagnostic slices now reuse the production session and X
authority while narrowing the manual surface. The selection slice launches one
Kitty and one Firefox, stops after four browser stages, and requires four
ordered cross-client owner-change/conversion intervals. Direction-specific
tokens plus a trusted full-field selection arm make stale CLIPBOARD or PRIMARY
state fail at the step that introduced it. The lifecycle slice launches two
Kitty and two Firefox processes, skips the content choreography, and proves a
normal close followed by a WM-forced close with both peers retained. Each slice
has its own completion record, fail-closed verifier, and negative fixture.

The validation ladder is therefore: offline reducer/coordinator/verifier
regressions, the affected focused physical slice, then one complete physical
promotion run. Repetition belongs to the unattended installed-session soak,
not repeated manual choreography. This separation enables future automation
and timing optimization without weakening the release contract.

<!-- END IMPORTED BODY -->
