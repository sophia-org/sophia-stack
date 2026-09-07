---
id: legacy-active-0535
date: 2026-08-25
recorded_date: 2026-08-25
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-25: a probe's feature gate can read as a product defect

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16441–16469. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The offline pbuffer smoke refused its own pbuffer. `BuffersFromPixmap` answered
`BadPixmap`, and the recovery trace named the reason exactly:
`reason=never_imported kind=Some(GlxPbuffer)`. Read alone, that is a coherent
defect report -- the server allocated the drawable, so no import exists to
recover, and the half of DRI3 where the server owns the storage looks missing.

It is not missing. `external_probe_pixmap_allocator` is `#[cfg(not(feature =
"atomic-scanout-live"))] -> Ok(None)`. Run without the feature, the allocator is
`None`, the backing block's `&&` chain fails at its last link, and the request
falls through to a recovery that correctly refuses a drawable nothing backed.
With the feature the same commit exits zero, `first_error=none`, one buffer
originated at the pbuffer's own extent.

Every guard in that chain traces its refusal except the allocator, which is
absent rather than refusing, so the one link that broke was the one link that
said nothing. A temporary probe over the whole condition settled it in one run:
`backing_request=true allocator=false`. The runtime side had been correct the
entire time, and three commits' worth of suspicion pointed at it.

Two things follow. A capability resolved by `cfg` should say so where it
resolves to nothing, on the same switch the rest of the path already honors;
absence is a decision and deserves the same line a refusal gets. And a command
published in the compatibility matrix must carry the features its evidence
requires -- the neighbouring glxgears row names
`--features atomic-scanout-live` and the pbuffer row did not, which is the whole
distance between "proven against real Mesa" and a defect that was never there.

<!-- END IMPORTED BODY -->
