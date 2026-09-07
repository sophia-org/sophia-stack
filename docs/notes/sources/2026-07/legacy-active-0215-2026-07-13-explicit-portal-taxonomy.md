---
id: legacy-active-0215
date: 2026-07-13
recorded_date: 2026-07-13
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security"]
---
# 2026-07-13: Explicit Portal Taxonomy

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7246–7257. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The portal milestone began by removing two ambiguous protocol encodings.
`Screenshot` had represented both still capture and recording, while URI-open
requests were labeled as notifications and distinguished only by a type hint.
`PortalTransferKind` now has explicit clipboard, drag-and-drop, file-handoff,
screen-capture, screen-recording, URI-open, and notification values. Each maps
directly to its namespace capability. Reducer and codec regressions cover every
kind; established codec numbers for the five existing values remain stable,
with recording and URI-open using new tags. Request/grant lifecycle separation
is the next portal slice.

<!-- END IMPORTED BODY -->
