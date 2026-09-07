---
id: legacy-active-0138
date: 2026-08-05
recorded_date: 2026-08-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "policy"]
---
# 2026-08-05: presented admission has one positive-focus writer

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 4433–4453. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The first mixed-application QEMU run after the CPU admission repair reached
vkcube's exact presented candidate, then terminated on a duplicate X-authority
focus control. The layout state machine had correctly deferred positive focus
until that candidate retired. Independently, the WM workspace-projection
adapter queued the same transaction and surface immediately. Retirement could
therefore reproduce a control that was still awaiting its frontend
acknowledgement.

Positive focus now has one owner: `PersistentLiveLayout` queues it immediately
for committed backing snapshots or after the exact retirement for presented
admissions. The projection adapter only clears an old focus when policy leaves
no visible target. Focus evidence remains ordered after workspace projection
for immediate transitions and is emitted at retirement for deferred ones. A
pure projection regression rejects positive-focus synthesis, while the
presented-admission regression proves that no focus is available before the
matching retirement. The mutation-tested xmonad verifier, source audit, and
complete all-features suite pass; the commit-pinned QEMU gate remains the
acceptance boundary.

<!-- END IMPORTED BODY -->
