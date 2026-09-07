---
id: legacy-active-0385
date: 2026-08-03
recorded_date: 2026-08-03
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-03: separate transition rationale from the implementation audit

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 11686–11725. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Review of `state-and-transition-discipline.md` found that its transition
  systems, I/O-automata, single-writer, and CALM rationale remains useful, but
  its dated conformance section was becoming a second roadmap. The rationale
  is now evergreen; current gaps remain dated evidence here and become planned
  work only when admitted and ordered in `todo.md`. Milestone 11 remains the
  active roadmap priority.
- The audit confirmed that `AuthorityTransactionIntake::commit` and
  `ProductionSessionCoordinator::{commit_authority_batches,
  replace_committed_surfaces}` can advance or replace state outside the
  prepared-presentation retirement path. `PreparedSurfaceCommit` application
  is protected by coordinator call order rather than a type that binds the
  exact prepared scene and submission to the required output retirements.
- Per-output and backend assembly replacement APIs still expose mutable copies
  shaped like alternate committed-state writers. A future admitted design may
  replace these with immutable, generation-tagged scene projections and a
  retirement capability owned by the Engine coordinator. Any such capability
  must bind the exact candidate, submission, and required output-retirement
  set. A failed retirement is terminal settlement, never authority to commit.
- `PortalRequestGrantLifecycle` enforces central capacity, duplicate, and
  generation rules, while `ClipboardPortal`, `DragAndDropPortal`,
  `FileHandoffPortal`, `ScreenCapturePortal`, `UriOpenPortal`, and
  `NotificationPortal` still insert directly by transfer ID. Consolidating
  those public admission paths is a future hardening candidate, not work
  admitted by this documentation review.
- The repository has deterministic transition tests but no TLA+ module and no
  unified authority-transition ledger. A formal model remains an optional
  validation candidate whose toolchain and reproducible command must be
  admitted before implementation. A future privacy-safe transition ledger or
  trace would be observational only: it may correlate opaque identities,
  generations, actions, settlements, and submissions, but must never replay
  authority effects or retain protocol identities, payloads, metadata, or
  pixels.
- Terminology now reserves committed visual state for post-retirement truth,
  distinguishes earlier accepted or prepared state, treats presentation as
  output-scoped rather than globally simultaneous, and applies CALM locally:
  each authority orders its own non-monotonic decisions; only decisions that
  bind several ownership domains require cross-authority coordination.

<!-- END IMPORTED BODY -->
