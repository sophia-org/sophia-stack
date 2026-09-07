# todo.txt and zk cutover, 2026-09-06

The user chose the upstream todo.txt system for task tracking, kept the
`todo.md` filename, and asked for integration with the linked notebook. Task
descriptions and completed-work history must not turn either root file into
another development log.

The cutover moved 58 open rows into one-line todo.txt tasks and retained 19
previously checked rows in a [historical source note](sources/2026-09/todo-cutover-completed.md).
It includes the old candidate and deferred bullets, not just checkbox rows.
Their lanes remain distinct: 23 critical, seven parallel, 22 candidate, and six
deferred tasks. No lane was promoted and no completion date was invented.

Twenty-one [linked plans](indexes/plans.md) retain task detail, dependencies,
stage exits, and the post-CP-15.2 planning checkpoint. Each task has a stable ID,
reviewed ordering key, and a Markdown link to its criteria. The old product-state
narrative is retained as a [dated source](sources/2026-09/todo-cutover-product-state.md),
not promoted to a newly verified description of the current binary.

The <a href="../history/todo-cutover-2026-09-06.txt">pre-cutover snapshot</a>
preserves the complete roadmap as it stood after the notebook migration and
before task conversion. The <a href="../history/todo-cutover.json">mapping</a>
records its hash, every imported row's line range and original hash, stable task
ID, lane, and owning plan. This snapshot includes uncommitted documentation work;
it is identified by its content hash rather than a fabricated commit identity.

## Completion and synchronization

`zk tasks` invokes upstream todo.txt with repository-local configuration and
refreshes the zk index after a successful command. Task state has one owner;
notes do not mirror checkboxes. A task's Markdown link is valid both in
`todo.md` and in a monthly `done-YYYY-MM.md` beside it. The CLI copies task text
unchanged, so keeping these files at the same depth preserves relative links.

Completed lines go directly to the current monthly file. `done.md` is a small
history guide, and `zk completed` finds the dated files. Detailed results and
milestone progress belong in notes before a task is marked complete. The
[tracking contract](../work-tracking.md) defines admission, evidence, handoff,
completion, and manual-edit rules.

## Verification

An isolated notebook copy exercised the real CLI through `zk tasks`. Completing
one task removed it from the 58-row active queue, retained its ID and Markdown
link, added the completion date, wrote the monthly file, and made that file
appear in zk's backlinks to the plan. Adding a task supplied its creation date.
The real queue was unchanged by the test. No live session or runtime build was
needed.

The upstream format and CLI were cloned into `~/src/todo.txt` and
`~/src/todo.txt-cli`. Sophia's agent instructions now use this workflow for the
critical path, milestone progress, development notes, and architectural decisions.
