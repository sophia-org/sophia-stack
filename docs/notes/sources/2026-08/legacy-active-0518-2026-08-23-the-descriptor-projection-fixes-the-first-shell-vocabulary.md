---
id: legacy-active-0518
date: 2026-08-23
recorded_date: 2026-08-23
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "shell", "architecture"]
---
# 2026-08-23: the descriptor projection fixes the first shell vocabulary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15809–15843. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The offline reference slice now carries sanitized `ChromeDescriptorTable`
rows through one bounded title-only projection, generic renderer-neutral solid
and text commands, independent per-head lowering, and exact last-presented
opaque activation targets. Candidates contain at most sixteen unique slots,
surfaces, and issuer-scoped action tokens. Broker, revocation, recipient,
descriptor, and target generations must match. Ordering and selection remain
candidate inputs; Engine does not infer MRU policy or recover a `SurfaceId`
from activation.

The selected, trust, attention, and fallback-title states render without icon
disclosure. Icon tokens remain in the descriptor table for later policy work.
The renderer uses the bundled JetBrains Mono NL Regular 2.304 bytes already
retained for Tier 0, with a separate 128-entry, 16-MiB text cache. Cache keys
contain every pixel-affecting head-native text field, and `Arc` ownership keeps
an evicted raster alive through in-flight frames. A broker regression also
closes a metadata-loss defect: attention-only updates now retain the last
sanitized label instead of replacing it with no label. Lowering a disclosure
rule clears that retained label and immediately emits the cleared descriptor,
so a later attention change cannot republish data from the broader rule.

Deterministic tests cover stale, duplicate, excessive, and wrong-epoch
candidates; fallback labels and visual markers; stable-node selection damage;
unequal 1920x1080 and 1280x720 heads; cache hits, bounds, deterministic pixels,
and eviction lifetime; and exact device, target, presentation, and application-
owner capture behavior. The repeatable 16-entry, two-head offline probe ran 256
iterations at 110 us p95 against a 16,667 us budget on the current host.

This evidence advances the recent-window switcher and shell-facing window-list
ledger rows from Open to Partial. It does not add `sophia_shell_v1`, a live
shortcut, action dispatch, previews, MRU policy, or a Hagia shell. The next
critical-path boundary is the separately protected shell-role transport and
the first title-only `hagia-shell` client.

<!-- END IMPORTED BODY -->
