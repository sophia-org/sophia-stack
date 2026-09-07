---
id: legacy-roadmap-0007
date: 2026-08-30
recorded_date: 2026-08-30
date_basis: snapshot-date
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# Boundaries To Preserve

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original snapshot</a>, lines 496–515.
Date from the 2026-08-30 roadmap snapshot, not an event or completion date.

[Parent section](legacy-roadmap-0006-boundary-and-capability-ledger.md).

<!-- BEGIN IMPORTED BODY -->

- **Engine** owns physical input, outputs, work areas, scene geometry, focus,
  chrome, transactions, rendering, presentation, and scanout. It must not learn
  X11 resource identities or application metadata.
- **X Authority** owns visuals, colormaps, X resources, ICCCM/EWMH reduction,
  X11 events, client drawing, and protocol feedback. It lowers pixels and
  opaque policy facts into Engine; it does not own physical layout or scanout.
- **Blind WM policy** consumes opaque surfaces, workspaces or views, geometry,
  constraints, and permitted role facts. A native Sophia WM speaks this API
  directly. A classical X11 WM speaks to a private synthetic X server whose
  bounded profile translates its policy into the same API. Neither path may
  receive XIDs, titles, classes, PIDs, namespace IDs, or payloads.
- **Session shell and configuration** own trusted launch provenance, key-bound
  applications, status presentation, wallpaper, lock, screenshots, audio, and
  process supervision. These are not X Authority shortcuts.
- **Portals** own cross-namespace clipboard, drag-and-drop, file, URI, capture,
  and notification decisions. Only the small-text `CLIPBOARD` and `PRIMARY`
  execution path is complete.

<!-- END IMPORTED BODY -->
