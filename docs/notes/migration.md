# Development history migration, 2026-09-06

The development log had become a poor starting point: 20,621 lines combined
current questions, resolved incidents, design choices, and superseded instructions.
The archive added another 1,479 lines. The user approved linked notes and ADRs,
maintained with `zk`, as their replacement, then included archived milestone work.

The migration preserves 638 active-log entries, 28 archive entries, 29 milestone
history entries, and 26 archived roadmap sections as 721 historical source notes.
The milestone history was 1,044 lines; the archived roadmap was 2,736 lines.
Each second-level log or milestone-history heading starts one entry. The roadmap
also splits its third- and fourth-level headings, retaining parent and child
links so the old ordering and dependencies remain visible. Fenced code is not
interpreted as a heading. No entry has been silently promoted to an accepted
decision or a current open task. The archive's long X Bridge Probe Start
investigation stays coherent instead of becoming arbitrary fragments.

## Dates and provenance

Headings supply dates for 650 entries. At the user's request, Git supplies dates
for the 36 undated research entries and nine undated milestone-history entries.
For each, `git log --reverse -S` identified the first addition of the heading in
its source file; the commit diff was checked for that exact added heading. The
committer timestamp and recorded timezone supply the date. Each note and the
manifest retain the full commit hash, timestamp, and `date_basis`.
The [date audit](indexes/recovered-dates.md) lists all 45 recovered dates.

The 26 archived roadmap sections use the explicit 2026-08-30 snapshot date.
Commit and snapshot dates do not establish event dates, completion dates, or the
dates of later edits. Historical research topic tags are provisional finding aids
derived from headings, with multiple topics allowed; roadmap records carry the
`milestone` tag. These are not audits of current architectural ownership.

The import adds provenance and metadata, promotes the entry heading to a document
title, and adjusts relative links for the new location. Links inherited from the
archived root-level todo are resolved against their intended repository paths.
Links to excluded compatibility indexes use HTML anchors so zk does not report
them as missing notes. Body text is otherwise preserved. Stable `legacy-active-*`,
`legacy-archive-*`, `legacy-milestone-*`, and `legacy-roadmap-*` IDs follow original
entry order, independent of date and topic. The original preambles remain in the
snapshots; they are superseded by the [maintenance guide](README.md).

The <a href="../history/notebook-migration.json">manifest</a> records snapshot hashes,
byte counts, original line ranges, section hashes, migrated body hashes, note
paths, and heading mappings. These maps cover all original entry and nested
heading anchors. Current incoming research links now target individual notes;
the old log and roadmap paths remain compatibility indexes for external links
and frozen documents.

- <a href="../history/research-log-2026-09-06.txt">Original active log</a>
- <a href="../history/research-log-archive-2026-09-06.txt">Original research archive</a>
- <a href="../history/roadmap-history-2026-09-06.txt">Original milestone history</a>
- <a href="../history/roadmap-archive-2026-08-30-2026-09-06.txt">Original roadmap snapshot</a>
- [Date index](indexes/date.md)
- [Topic index](indexes/topic.md)
- [Milestone map](indexes/milestones.md)

## Continuing the work

The first maintained records extract the documented separation of ordinary
desktop readiness from application proofs and Session ownership of desktop
composition. Their ADRs cite the original decisions and the current contracts.
The physical startup check remains pending; this documentation migration supplies
no new runtime evidence. Historical unchecked roadmap items do not re-enter the
active queue without explicit promotion in `todo.md`.

This migration introduces no production command or build dependency. `zk` owns
note creation and search. Its configuration, templates, and all Markdown are
versioned; its disposable database is ignored. The frozen migration indexes need
no generator in the production tooling tree. Agent guidance and the active
roadmap now direct future updates into this notebook.

## Migration verification

All four snapshots match their source files at `3d023c07` byte for byte. The
manifest accounts for every source line after each preserved preamble, without
missing or overlapping sections. All 721 imported bodies match their original
text after normalizing link destinations and formatting. Each legacy heading
has a compatibility anchor, all notebook and compatibility-page local links
resolve, and `zk list docs/notes --broken-links` reports no missing notes.

All four creation aliases and templates were exercised with `--dry-run`, including
an invocation from a notebook subdirectory. They retain titles with spaces and
produce the expected date, kind, and initial status. The local database is ignored
by Git. Documentation whitespace checks pass; no runtime build or graphical
session was required for this migration.
