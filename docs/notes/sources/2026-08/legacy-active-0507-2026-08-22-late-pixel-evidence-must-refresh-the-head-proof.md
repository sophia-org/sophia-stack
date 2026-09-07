---
id: legacy-active-0507
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-22: late pixel evidence must refresh the head proof

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15529–15560. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Signed source `8cfd831b9b2354e2253e4a470803cc30ff24a27f` produced and
independently re-verified mirror archive `0004`. The following mixed run,
`/tmp/sophia-mixed-output-centered-20260822-160302.log`, exercised the Present
pin: frame 55 waited behind frame 54, then a newer ordinary frame replaced the
deferred slot only after the Present had acquired KMS ownership. The public
policy committed three physical heads as two logical outputs and the 30-second
runtime drained cleanly.

The terminal head counts were exact. Heads 1, 2, and 3 reported respectively
`18/17/17`, `21/20/20`, and `12/11/11` submissions, retirements, and callbacks;
the one extra submission on each head is its logged synchronous modeset. The
completion check nevertheless rejected head 2 because its `nonzero_exports`
counter remained zero.

That zero contradicted the renderer's later evidence. Head 2 spent its three
full-frame readbacks on early blank compositions. After the output policy
settled, the gate's requested final-region readback measured 53,676 nonzero
pixels in the extended terminal, but region diagnostics did not update the
renderer context's cached light proof. Every later KMS submission therefore
carried the old zero even though the same finished composition had just been
read as nonblank.

Requested region readbacks now contribute to the persistent context proof.
The context retains the strongest nonzero count observed by either its bounded
full-frame probes or its requested final-region probes, and a later black frame
cannot erase that historical fact. The three-probe full-frame budget and its
cost are unchanged. This is local evidence; the mixed gate produced no archive
and the Hagia/broker gate was not reached, so the signed physical sequence must
run again.

<!-- END IMPORTED BODY -->
