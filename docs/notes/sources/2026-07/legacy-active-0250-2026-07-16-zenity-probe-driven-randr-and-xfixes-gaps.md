---
id: legacy-active-0250
date: 2026-07-16
recorded_date: 2026-07-16
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11"]
---
# 2026-07-16: Zenity Probe-Driven RandR And XFixes Gaps

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 8300–8312. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The current GTK3 Zenity engine probe exposed two bounded requests after its
package became available locally: RandR `GetOutputProperty` for EDID and XFixes
`SelectSelectionInput`. Sophia now returns a valid empty output-property reply
when no EDID payload is retained and validates the selection window, atom, and
three-bit event mask. The same probe showed that advertising DRI3 without a
render-device provider creates an avoidable `BadImplementation`; socket
advertisement now withholds DRI3 in that configuration so GTK selects MIT-SHM.
The repeated probe commits one surface with 288,920 nonzero software bytes and
`first_error=none`; no broader RandR property store or XFixes event expansion
was inferred.

<!-- END IMPORTED BODY -->
