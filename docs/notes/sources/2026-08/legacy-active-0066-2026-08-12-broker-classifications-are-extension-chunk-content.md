---
id: legacy-active-0066
date: 2026-08-12
recorded_date: 2026-08-12
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security"]
---
# 2026-08-12: Broker classifications are extension-chunk content

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1982–2024. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The open question was how broker-issued classifications reach Hagia, and it was
  the last wire decision that could force a `*Begin` layout change. Two shapes were
  on the table: a small closed set of policy classes riding
  `SnapshotSurface.kind` or spare `capability_bits`, or an expiring per-surface
  grant that fits neither and forces a layout change. The answer is neither.
- **The closed set does not fit the rules.** Triad's `WindowRule` at baseline
  `fb8fb27e` (`src/types/runtime_values.nim:211-283`) carries about thirty-five
  outcomes. Stripping what never crosses (match expressions), what already crosses
  (`min_size`/`max_size`, `request_state_bits`, reduced parent role), and what
  belongs to Engine chrome or the session and security authorities
  (`border`, `focusRing`, `clipToGeometry`, `keyboardShortcutsInhibit`,
  `idleInhibitMode`, `presentationMode`, `openOverlay`, `openUnmanagedGlobal`)
  still leaves around nine WM-bound booleans and eleven WM-bound *parameters*:
  default workspace, output, column width, scroller proportions, default window
  size, named scratchpad, floating position, maximize policy, forced layout.
  Eleven free `capability_bits` would not survive the booleans, and a bitfield
  cannot carry a workspace number or a scratchpad name at all. The phrase "a small
  closed set of policy classes" described a classification vocabulary accurately and
  these rules inaccurately.
- **Decision: a capability-gated extension chunk**, carrying
  `(surface, classification)` records under a reserved `0xFF00`–`0xFFFF` kind. It is
  uncounted, so it costs no layout change; chunk data is self-delimiting, so it
  carries parameters as easily as flags; and it reaches only a client that
  negotiated the capability.
- **This removed a pre-freeze obligation instead of satisfying one.** The decision
  was believed to require reserving vocabulary in `SnapshotSurface` before the
  freeze. It requires reserving nothing there, and the classification vocabulary is
  no longer frozen with the revision — a rule family recognized later is an added
  chunk record rather than an impossibility.
- The option existed only because outbound capability gating landed earlier this
  session. The original analysis was written while clause 2 was unsound, when every
  server-to-client addition really was now-or-never. Its framing survived the change
  that invalidated it, which is the general hazard: a decision inherits the
  constraints of the day it was framed, and those constraints are exactly what the
  intervening work is meant to remove. Re-derive a pending decision after landing
  anything that changes its premises.
- One review-time rule is now checked. The generator rejected nothing in the
  reserved range because keeping it clear was documented rather than enforced; it
  now refuses any ordinary record declaring a kind at `0xFF00` or above. Verified by
  temporarily moving `SnapshotAction` to `0xFF01` and confirming the refusal.

<!-- END IMPORTED BODY -->
