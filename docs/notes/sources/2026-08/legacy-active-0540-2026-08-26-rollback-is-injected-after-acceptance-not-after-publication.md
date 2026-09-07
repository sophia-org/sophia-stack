---
id: legacy-active-0540
date: 2026-08-26
recorded_date: 2026-08-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-08-26: rollback is injected after acceptance, not after publication

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16627–16698. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The final frame-fed output proof now has one deliberately narrow fault control.
`--output-proof-rollback-after-apply` can target only the private startup output
transaction in a bounded native public-Hagia session, and requires an explicit
hardware arm. It fires on the backend's final `Applied` transition, which means
every ordered KMS card has accepted the candidate, but before Engine installs
candidate resources, queues first frames, or publishes anything to X or policy.
The ordinary reverse-card rollback and local noncommitted settlement then do all
recovery work. No protocol field or outcome was added for a proof-only concern.

The physical gate is a pair rather than one run with two branches. The first run
must prove installation, first presentation, frontend acknowledgement, committed
snapshot publication, physical input, and clean teardown before the second run
may start. The second proves the selected rollback boundary and must contain none
of those candidate-publication events. Both runs bind the same signed Sophia and
Hagia commits, binary hashes, checked-in core/profile blobs, and exact DP-1/DP-2
connector facts. The archive verifier rechecks commit signatures and blobs; the
recorder refuses a duplicate evidence pair. Synthetic fixtures remove every
required event and mutate identities, forbidden rollback events, signed profile
selection, and checksum closure. The harness is complete offline. The real TTY4
run remains explicitly authorized work and is not claimed by these fixtures.

The first authorized TTY4 attempt reached a passing schema-5 atomic scanout
preflight with two openable primary nodes, two atomic-capable nodes, and one
scanout target. Sophia then exited before the success phase completed. The gate
had already written the live stream into its private run directory, but its
top-level `ERR` trap deleted that directory, leaving only the preflight record;
there is therefore no honest basis for classifying the runtime exit yet. Failed
runs now remain in `/tmp/sophia-frame-fed-output/`, and the trap prints the exact
directory and exit status. The physical pair remains open until a clean signed
successor either passes or retains enough evidence to diagnose the exit.

The retained successor did diagnose it, and the display transaction itself was
successful: both candidate frames were prepared, the atomic card effect was
accepted, both page flips retired, the frontend acknowledged generation 2, and
the startup transaction committed. Failure came afterwards at Hagia
configuration. Hagia advertises all four session-operation slots; normal session
construction already admitted terminal, close, and logout, but the output-only
gate supplied no browser application and therefore omitted slot 2. Sophia's
all-slots admission check rejected the configuration, three supervised restarts
repeated the same rejection, and bounded cleanup then removed the X display
before Kitty finished connecting. The Kitty/GLFW error was a cleanup consequence,
not the trigger. The runner now maps the unused browser operation to its existing
terminal application, and a parser-level regression requires the resulting
Hagia-facing catalog to contain slots 1 through 4. The retained failed evidence
is `/tmp/sophia-frame-fed-output/249c429e328f-20260826T104207Z-10055/success.log`;
it is diagnostic evidence, not a promotion archive.

The next signed run closed the entire success phase, including physical text and
clean teardown, and the rollback phase crossed the intended boundary exactly:
candidate KMS acceptance, proof trigger before installation, reverse-card KMS
acceptance, local `RolledBack`, no candidate publication. DP-2 then retired both
the restored owner and the first post-rollback composed scene. Scene changes on
DP-1 kept producing the same blank logical checksum on DP-2, but retained-scene
suppression checked only the pending slot. Once each identical frame reached
displayed ownership, the next global scene queued it again. Two redundant DP-2
flips followed; the last delivered no callback and the 500 ms watchdog fired
roughly 400 ms after the physical-text matcher became ready, before a human
could complete `outputrollback`.

The correction is an ownership reducer rather than a longer watchdog or a proof
exception. It compares the requested retained checksum with the newest of
pending, rendering, submitted, and presented content. An identical newest frame
suppresses work at every stage; a different newer frame still queues even when
an older displayed frame matches, and Present-owned pixels remain
non-interchangeable because they expose no logical-scene checksum. The retained
failed evidence is
`/tmp/sophia-frame-fed-output/aa72ea2c183c-20260826T104521Z-17250/rollback.log`;
it proves rollback mechanics but is not a promotion archive. A clean signed
successor must still run both phases under one bound identity.

<!-- END IMPORTED BODY -->
