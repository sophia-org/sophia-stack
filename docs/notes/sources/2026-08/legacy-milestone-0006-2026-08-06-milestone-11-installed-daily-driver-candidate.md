---
id: legacy-milestone-0006
date: 2026-08-06
recorded_date: 2026-08-06
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: [historical, milestone, validation]
---
# 2026-08-06 Milestone 11 Installed Daily-Driver Candidate

Historical milestone source. Checked and unchecked items retain their original
meaning; they do not add work to the current roadmap.
<a href="../../../history/roadmap-history-2026-09-06.txt">Original snapshot</a>, lines 150–174.

<!-- BEGIN IMPORTED BODY -->

- [x] Promoted strict core/native-WM KDL configuration with safe discovery,
  validation, last-known-good reload, and explicit restart-only changes.
- [x] Established Engine-owned focus-ring and frame chrome behind blind-WM v6
  negotiation, generation-ordered policy updates, and atomic content clearance.
- [x] Installed one immutable, repository-independent release with normal,
  fallback, watchdog, emergency, Firefox, and native-chrome greetd entries.
- [x] Retained three consecutive normal archives plus independent fallback,
  watchdog, and emergency archives with exact binary/runtime identity and
  display-manager handoff.
- [x] Passed native chrome and live configuration proof, including the repaired
  tty7→tty3→tty7 renderer-image handoff and visible retained-frame retirement.

The historical three-cycle gate ends at normal archive `0005` on commit
`4cc84913`; fallback `0005`, watchdog `0003`, and emergency `0002` pass the
same installed-release contract. Native-chrome archive `0006` on commit
`d29e2f2c` captures and restores two renderer images, routes 28 physical keys,
retires retained content after seat reacquisition, and exits with clean
renderer, presentation, protocol, application, and VT ownership. Milestone 12
now owns repeated-cycle and soak stability; this archive does not claim those
long-duration gates.

---

<!-- END IMPORTED BODY -->
