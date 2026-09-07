---
id: legacy-active-0475
date: 2026-08-20
recorded_date: 2026-08-20
date_basis: first-heading-commit
date_commit: 3ed54506f028b538c58bec0002e21f30658172dc
committed_at: 2026-08-20T19:11:42-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# Two sizes, both correct, and I picked the wrong one

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 14419–14451. The heading has no date. Its first recorded addition is commit
`3ed54506f028b538c58bec0002e21f30658172dc` (2026-08-20T19:11:42-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

The raster-size change did not fix the fatal: the next run reported
`planned: 1920x1080, held: 1280x1440` again. The record was still being filled
with the placement, because the live site called
`live_transaction_observed_size`, and that helper answers a different question:

```rust
if source == transaction.target_content_size() {
    logical            // the surface reached its configured extent
} else {
    source             // an old buffer cannot satisfy a new extent
}
```

It reports the *logical* size whenever a client produced the content it
declared, which is exactly right for resize and admission gates -- they ask
whether a surface has reached its configured size, not how many pixels it
holds. Reading it as a measurement put the geometry straight back into the
committed content, and the run failed identically.

`live_transaction_raster_size` now names the other question and reads the
buffer registry, falling back to the declared extent only where no buffer is
registered. The two helpers sit next to each other with their difference
written down, and a test asserts both answers for one transaction whose buffer
does satisfy its extent: raster 1266x1412, observed 1276x1422.

Worth naming the mistake plainly. The commit before this one described
measuring a raster rather than inferring it, and then inferred it from a helper
whose name contains "observed". A function that returns one of two different
quantities depending on a comparison is a hard thing to reuse safely, and the
guard against that is what its callers ask for, not what it is called.

<!-- END IMPORTED BODY -->
