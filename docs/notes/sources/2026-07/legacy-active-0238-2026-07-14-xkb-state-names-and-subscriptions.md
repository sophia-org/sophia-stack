---
id: legacy-active-0238
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-14: XKB State, Names, And Subscriptions

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7978–8029. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The X authority now implements the generic XKEYBOARD 1.0 state/name path rather
than a toolkit-specific startup exception. `GetState` reports the last
authority-translated effective modifier state, `GetNames` publishes interned
component atoms derived from the configured session RMLVO, and bounded
`SelectEvents` parsing persists each client's StateNotify detail mask. Modifier
transitions emit the standard 32-byte StateNotify record only when the selected
state detail changed. Focus/hierarchy policy and retained classic/confined
session evidence remain separate open gates.

Window input routing no longer treats event-mask update order as focus. The
connection records CreateWindow parent links, mapped state, and ConfigureWindow
sibling/stack modes. Engine-selected target surfaces begin core propagation at
their owning window, ancestor selection is bounded against malformed cycles,
and root focus resolves through the current mapped stacking order. Scene-level
restack acknowledgement remains an Engine integration/evidence gate.

Retained live-session completion is now schema 12. It binds each completion to
its `classic_shared` or `confined` namespace profile and records whether the
deterministic Engine topology update was applied. The paired Milestone 3 runner
executes the same guarded two-xterm proof once per profile; its verifier requires
the confined startup record to have zero request and publish capabilities, both
runs to include an applied output update, and both to satisfy the existing
startup, composition, input-flush, presentation, resize, and cleanup checks.
The output-update acknowledgement now also carries the number of RandR records
queued to live subscribers. Schema 12 retains that count, and promotion rejects
an accepted topology update that reached no X11 client.

The paired runner now also requests a deterministic one-shot X11 surface
resize after both terminal surfaces have published. The live layout sends the
client-targeted ConfigureSurface command, validates the matching control
acknowledgement, and keeps the new geometry quarantined until a transaction
with matching resized pixels arrives. Schema 12 reports `surface_resize` only
after that commit; the promotion verifier requires the configure acknowledgement
and pixels marker in both namespace profiles.

The topology path now opens a dedicated authenticated RandR witness before the
Engine update, uses a reply-producing core request as a subscription barrier,
and reads back the resized ScreenChangeNotify record. This replaced the earlier
timing-dependent assumption that xterm itself would subscribe. The witness is
closed before frontend drain; a two-xterm headless live smoke then completed
with four queued RandR records, a matching wire event, committed resized
pixels, and clean process teardown.

Milestone 3 promotion no longer accepts the synthetic-input default. The paired
runner requires readable physical keyboard and pointer event nodes, exact
physical `sophia` plus Return input, flushed delivery, presented text pixels,
and a pointer-driven pixel change in both profiles. Schema 13 separates
automated terminal-content readiness from total operator interaction time, so
the two-second startup budget measures startup rather than typing speed.

<!-- END IMPORTED BODY -->
