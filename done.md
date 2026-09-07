# Completed work

Completed task lines live in monthly todo.txt-format files named
`done-YYYY-MM.md` beside `todo.md`. The CLI writes directly to the
current month; this page stays a short guide. Keeping the task files together
preserves their relative Markdown links when upstream todo.txt moves a line. Each line retains its stable task
ID, completion date, and link to the note containing its result and evidence.

Use `zk completed` to list the files, or `zk list --link-to NOTE_PATH` to find
completion records linked to a particular note. `zk tasks` refreshes the index
after each successful command. Direct edits need `zk index` afterward.

[Tracking contract](docs/work-tracking.md) · [Milestone history](docs/notes/indexes/milestones.md)

The [pre-cutover checked tasks](docs/notes/sources/2026-09/todo-cutover-completed.md)
remain historical evidence with their original context; they are not newly
completed tasks.
