---
id: legacy-active-0513
date: 2026-08-23
recorded_date: 2026-08-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-23: physical evidence matchers must mirror producer punctuation

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15694–15709. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first Hagia run on signed source
`c9248e73bd8085c1296fbe110c8c47087f5f9c17` committed move-to-output action 5,
moved Kitty to DP-2, and submitted a mixed frame there with 11,499 nonzero
pixels. The guide did not advance. Its new watcher expected
`nonzero_rgb_pixels=11499`, but the native producer embeds that field in a Rust
debug record as `nonzero_rgb_pixels: 11499`.

The guide and final verifier now match the producer's colon form, and the local
fixture carries the same record instead of an invented equals form. The strict
causal boundary remains: action 5 must commit first, the nonzero output-2
submission must follow, and action 6 must commit afterward. This is a proof-
fixture correction; it does not change Sophia's executable or invalidate the
current mirror and mixed archives.

<!-- END IMPORTED BODY -->
