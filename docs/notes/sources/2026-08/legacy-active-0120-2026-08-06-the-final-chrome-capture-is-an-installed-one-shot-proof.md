---
id: legacy-active-0120
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell", "validation", "tooling"]
---
# 2026-08-06: The final chrome capture is an installed one-shot proof

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3980–4009. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The schema-2 native chrome verifier was strict, but its physical runner still
depended on a checkout, an on-login release build, and a separately retained
sequence file. The installed release now packages the native WM and guarded
driver as `Sophia Native Chrome Proof`. One menu selection reserves an attempt,
advances ring-only, frame-only, and combined modes, and finalizes a checksummed
archive after normal logout.

The shared installed-attempt ledger accepts explicit bounded extra evidence and
verifier inputs, keeping reservation, launch identity, lifecycle, and checksum
semantics common rather than cloning them for chrome. The archive verifier
binds the sequence commit to the release and fails closed on incomplete
transitions, lost physical input, output/native debt, emergency recovery,
modified evidence, or identity drift.

Installed commit `e07afa0f` passed native-chrome archive `0002` on two physical
outputs. The checksummed evidence contains all six ordered ring/frame phases,
48 routed physical keys, normal logout, clean native drain, an untriggered
guard, and exact VT restoration. This closes the remaining physical schema-2
chrome capture.

The preceding archive `0001` remains a useful failed attempt. An operator VT
switch quiesced native work before releasing tty7 and reacquired the seat on
return, but the resumed renderer repeatedly returned `InvalidTarget` instead
of rebuilding a usable target. Emergency recovery then retained a failed
status-130 archive. The chrome proof is complete, but the installed candidate
must restore and re-prove VT-resume target recreation before stability work can
advance.

<!-- END IMPORTED BODY -->
