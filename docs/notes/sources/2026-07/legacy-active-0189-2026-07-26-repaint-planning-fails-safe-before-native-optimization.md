---
id: legacy-active-0189
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-26: Repaint Planning Fails Safe Before Native Optimization

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6402–6426. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The retirement-safe display-list region is not, by itself, authority to skip a
frame: client-buffer, software-cursor, output, and future animation damage must
also participate. Sophia therefore adds the next optimization boundary without
prematurely changing pixels. Engine now reduces compositor damage to an
output-local `skip`, `partial`, or `full` repaint plan. It clips every rectangle
to output bounds, coalesces only unions that remain exact rectangles, and
computes a bounded pixel count. More than 2,048 raw rectangles, more than 32
partial rectangles, or at least 60 percent output coverage fails safe to a
full-output plan. Invalid output dimensions and policies are rejected.

Deterministic tests cover clipping, exact versus L-shaped coalescing, pixel
accounting, coverage fallback, fragmentation fallback, raw-capacity fallback,
invalid inputs, and attachment to the in-flight display-list lifecycle. The
two-output QEMU xmonad run then observed empty `skip` baselines, four-rectangle
partial border creation, partial old/new focus damage, partial border removal,
and zero compositor repaint work for stable client-only frames. The M7 verifier
now rejects a focus proof without a nonempty retired partial plan.

This is planning evidence, not a claim that rendering is partial. The next
implementation stage must combine client, compositor, cursor, and output damage
into one frame plan, preserve or reconstruct the destination buffer correctly,
and use full rendering whenever buffer age or native capability is uncertain.

<!-- END IMPORTED BODY -->
