---
id: legacy-active-0501
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: first-heading-commit
date_commit: 04dbc609ad25163cdae4993d1cc1cbe8ea2f35a7
committed_at: 2026-08-22T10:00:13-04:00
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# A flag beside the field that decides it

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15335–15383. The heading has no date. Its first recorded addition is commit
`04dbc609ad25163cdae4993d1cc1cbe8ea2f35a7` (2026-08-22T10:00:13-04:00).
This dates the heading record, not every event or later edit in the entry.

<!-- BEGIN IMPORTED BODY -->

Reviewing the protection-domain work against the Pnut evaluation turned up the
same defect on both sides of the boundary, at different distances from
enforcement.

Pnut's was found by the audit and fixed upstream: `allowed_bind = []` and
`allowed_connect = []` were indistinguishable from the fields being absent, so
the natural spelling of deny-all left the right unhandled and produced
unrestricted network access. The fix makes the type carry the distinction and
keys `handled_access_net` on presence rather than non-emptiness, with the ABI
bumped to match so the handled bits are requested at a level that supports them.

Sophia's was the same class one step earlier. `ProtectionNetworkAccess` was
constructed, stored, exposed by a getter, and asserted in a test, and
`bubblewrap_arguments` emitted `--unshare-net` as a literal without ever reading
it. The configuration did not reach enforcement at all. It was harmless today
because the enum has one variant, which is exactly what makes it worth fixing
now: the agreement is a coincidence of the type having nothing to disagree about,
and the second variant is already named in the roadmap.

The fix is small enough to be worth describing precisely, because the risk in a
change like this is the change itself. The flag is emitted from a match on the
policy, in the bubblewrap backend rather than on the enum -- the policy is
backend-neutral by design and `--unshare-net` is one backend's spelling of it,
where a Landlock backend would satisfy the same `Denied` with a handled-access
mask. It is emitted in the slot the literal used to occupy, so the command line
is byte-identical, which was checked by extracting all fifty-one string literals
from the builder at HEAD, splicing the policy-sourced flag back at its original
index, and comparing.

The guard is a source assertion rather than a behavioural one, which is a
compromise worth naming. The mapping is private to the backend and should stay
there; making the builder public to observe it would widen the crate's API for a
test, and with one variant a behavioural assertion could only restate the mapping
it was meant to check. What needs guarding is that the mapping is consulted at
all, and that is a structural fact. Both negative controls fail correctly:
restoring the literal, and replacing the match with an unconditional return.

Also restored the source-layout baseline to 26. Two files had crossed the
thousand-line threshold: `writers.rs` in the protected-broker commit, split along
the seam between the writer threads and the pure record builders they emit, using
the `include!` arrangement the file already used for `writers/input.rs`; and
`gl.rs`, which I had extracted to 988 lines and which later work took to 1013.
The one-shot context probes came out of it into `gl/context_probe.rs` -- they are
a different lifetime from the persistent pipeline, building throwaway state on
somebody else's current context to find out whether a pipeline is possible at
all.

<!-- END IMPORTED BODY -->
