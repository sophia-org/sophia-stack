---
id: legacy-active-0589
date: 2026-09-03
recorded_date: 2026-09-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-09-03: restore the backend default-feature boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 18647–18662. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The repository-wide default-feature test exposed stale native-presentation
exports in `sophia-backend-live`. The `presentation` module was correctly
limited to `libdrm-events` plus `gbm-probe`, but its public re-export was not;
the cursor transaction owner had the inverse mismatch, with a gated re-export
but an unconditional module declaration. Their native-only integration tests
also imported those APIs in default builds.

The module declarations, public exports, imports, and individual native
completion tests now use the same two-feature boundary. Generic production
session coverage remains active in default builds rather than gating the whole
test file. Both the default backend suite and its all-features suite pass, so
the next signed comparison candidate can be prepared without carrying a known
workspace compile failure.

<!-- END IMPORTED BODY -->
