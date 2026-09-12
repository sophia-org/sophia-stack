---
id: gl2ooa99
date: 2026-09-12
kind: investigation
status: investigating
tags: [investigation, shell, security]
---
# Shell reservation admission ignores the configured panel depth

## Question

Does Session enforce the operator's `shell { panel N; }` limit against the
admitted shell's reservation candidates?

## Evidence

Source review on 2026-09-12 at Sophia commit `2f38ac75`, prompted by the
ironbar delivery-path review. The working tree was clean before this record.

The [configuration contract](../../configuration.md) says Session decides the
reservation depth, a shell cannot exceed its configuration, and absent or zero
means no reservation. The complete admission path does not enforce that rule:

- `crates/sophia-config/src/desktop_profile.rs:812` extracts the configured
  thickness; zero and absence become `None`. Profile validation bounds the
  configured value, not subsequent client claims.
- `crates/sophia-session/src/live_session.rs:715` passes that value to
  `LiveMetadataShell::start`. In `live_session/metadata_shell.rs:135`, it only
  becomes `SOPHIA_SHELL_BAR_THICKNESS`. The shell session retains no copy of the
  allowance for admission.
- `crates/sophia-protocol/src/ipc/shell_v1.rs:401` validates candidate visibility,
  entries, selection and a nonzero reservation thickness no greater than 512.
- `crates/sophia-runtime/src/shell_transport.rs:261` checks the transaction,
  epoch, output, candidate generation and membership in the issued snapshot.
  It has no configured panel allowance.
- `crates/sophia-session/src/live_session/metadata_shell.rs:409` passes the
  client-supplied reservation directly to `ShellWorkAreaCoordinator::admit`.
- `crates/sophia-engine/src/shell_work_area.rs:92` checks epoch, generation,
  output and geometry. Its arguments and retained state contain no operator
  allowance. `shell_reservation_band` rejects a strip that exhausts the output,
  but cannot compare it with `N`.

An otherwise valid visible switcher candidate can therefore request a 64-pixel
bottom reservation on a 1280x720 output when the profile says `panel 32`.
The same missing check applies when `panel` is absent or zero. This trigger
requires an admitted shell and a requested snapshot containing a usable
descriptor; it is not an unsolicited startup-panel path.

## Finding and resolution

Confirmed missing authorization check by source inspection. The environment
variable is advice to the client, not enforcement against it. A buggy or
hostile admitted shell can exceed the configured allowance while staying
within the wire and output limits. This permits unauthorized work-area
reduction; it does not demonstrate foreign pixel access or an OS sandbox escape.

The defect belongs to Session's reservation admission policy. Engine's topology
checks do not replace the operator limit. It is independent of the unimplemented
content capability and of ironbar's toolkit requirements. No implementation
change or policy relaxation is made by this record.

## Validation and remaining work

The evidence is a trace of the source path, not a live malicious-client
reproduction or physical presentation claim. Existing
`crates/sophia-session/tests/shell_panel_config.rs` checks agreement between the
profile maximum and codec maximum; it does not test a client exceeding its
configured allowance.

Task `t083` in [todo](../../../todo.md) tracks the repair as candidate work.
Closure requires rejecting claims above the configured allowance, including
nonzero claims when the allowance is absent or zero, while preserving valid
claims, explicit withdrawal and the existing coherent presentation lifecycle.

## Connections

The [content-shell contract](../../content-shell.md) separately assigns grants
and budgets to Session. Its future content admission does not repair this
existing descriptor reservation path automatically. The
[content design](../decisions/6ndjwffd-content-capability-design-for-sophia_shell_v1.md)
defines additional allocation and reservation limits; this finding concerns
the implemented `ShellV1Candidate` path.
