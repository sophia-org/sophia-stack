---
id: wzxlxbok
date: 2026-09-12
kind: investigation
status: investigating
tags: [investigation, x11, conformance]
---
# Independent X11 socket conformance exposes missing client completions

## Gate and coverage

On 2026-09-12 the operator assigned Codex the broader independent X11 protocol
gate and subsequently made these protocol gaps the highest priority. t057 owns
the gate ahead of the individual repairs. Priority and open status live in
[todo.md](../../../todo.md), not in this evidence record.

The implementation and runnable commands are in
`tools/probes/x11_conformance/README.md`.
The host uses the production XServerFrontend, routed protocol broker and
concurrent worker paths, with deterministic software output facts and a private
Unix socket. It opens no display, session, device or VT. The Python client uses
independent protocol framing and assertions, not Sophia codecs or observations.
Both byte orders run, including oppositely ordered peers in cross-client cases.

The explicit manifest names mandatory behaviors, their core request numbers,
extension obligations, intentional policy exclusions and fixture limitations.
Missing/unexecuted mandatory results, NORESULT, unsupported/untested verdicts,
duplicates and deadlines fail. A decoder-declaration inventory prevents new
requests from disappearing from the coverage ledger. It currently inventories
76 decoded core requests: 27 have named cases, 49 have explicit coverage debt.
The missing but mandatory DestroySubwindows and NoOperation requests are also
named. This is a substantial selected behavioral gate, not full X11 certification.
Query/version coverage does not certify every operation of an extension.

## Candidate and evidence

The first meaningful baseline used c629cf7f. The final retained baseline used
the production source at acaa8453, including Claude's DestroyNotify repairs
fa23b570 and b3941c04, plus the uncommitted conformance host/harness. The report
records the dirty-source flag, harness/manifest hashes and host SHA256:

`e8f9755719481b28ac3007e4e6aecdac3e557c9dc4a8c0fcec9ac4fa29bc6216`

Retained evidence is under
`.artifacts/x11-conformance/baseline-acaa8453/` in the main checkout. Its
`report.json` contains every verdict and identity; host logs are per case.
Reproduce with the documented one-command gate and a fresh output directory.

**62 mandatory case/order executions: 46 PASS, 16 FAIL/TIMEOUT; gate exit 1.**
The eight failing behaviors fail in both byte orders. Timeouts here mean absent
mandatory completions at the fixed three-second deadline, not a claim that a
driver or the physical desktop hung.

Passing groups include setup resource ranges, window creation/tree/geometry,
map/configure, properties, cross-client selection ownership/transfer, focus,
pointer/keyboard grab contention, disconnect cleanup and grab release,
truncated-peer isolation, extension discovery/version negotiation, selected
SHAPE/SYNC stateful operations and deliberate extension absence.

Claude's explicit DestroyNotify repair independently passes: unmapped destruction,
both event addresses for StructureNotify/SubstructureNotify on a second client,
neither-mask suppression, invalid/repeated destroy without a phantom event,
and XID reuse without inheriting a retired subscriber. Those results do not
close the destruction family.

## UnmapNotify

The `unmap` case receives MapNotify and confirms the window is Viewable, issues
UnmapWindow, completes its following round trip, then receives no UnmapNotify.
This is independently reproduced in both byte orders. The UnmapWindow arm of
`dispatch/core/windows.rs` changes runtime state but returns an empty output
vector on success. The existence of an event encoder on other paths does not
deliver this request's notification.

t084 must implement the successful transition's structure and immediate-parent
notifications, preserve subscription filtering and ensure an already-unmapped
window or invalid ID does not generate a phantom transition. Its exit is the
real routed wire case plus named no-transition/parent-subscription controls.

## NoOperation

The `reply_errors` case establishes BadWindow, BadLength and BadRequest
completions, then sends core opcode 127 followed by a GetGeometry round trip.
Sophia emits BadRequest for the NoOperation sequence. This is not a missing
GetGeometry reply: the extra error arrives first and breaks correct completion
accounting. Opcode 127 is absent from the core decoder.

t085 must accept NoOperation without output, preserve subsequent sequence
completion and handle its permitted padding. Removing the unexpected error
from the test would conceal the omission.

## ListExtensions

The `extensions` case receives an empty ListExtensions response, while the
separate `extension_discovery` case confirms fifteen advertised names and their
distinct opcodes. `client_output/replies/core_early.rs` hardcodes zero names.
t086 must enumerate the actual frontend's advertised surface and keep it
consistent with QueryExtension, including provider-dependent availability.

An early harness expected DRI3 in this software-only host. That expectation was
wrong: `connection/dispatch.rs` explicitly suppresses its advertisement without
a render-device provider. The corrected manifest records DRI3 as a fixture
limitation. No DRI3 runtime defect is filed from that observation.

## Destruction family

Three independent cases still fail after b3941c04:

- `destroy_descendants`: GetWindowAttributes on a child still returns a valid
  reply after its parent is destroyed; the child lifecycle has not ended.
- `destroy_subwindows`: opcode 5 returns BadRequest before the round-trip reply.
- `destroy_peer_close`: another subscribed client receives no required
  DestroyNotify after the owner connection closes.

The [destruction-family investigation](ksbt5d8f-the-window-destroy-family-is-incomplete-beyond-destroynotify.md)
owns the source analysis and t087's residual exit. Descendant destruction must
precede truthful descendant notifications; adding only events would announce
destructions that never happened. Keep nested ordering, peer-close parent
subscriptions and resource/subscription retirement in that same lifecycle repair.

## XFIXES selection notification

`xfixes_selection` negotiates XFIXES, successfully selects owner-change events
on another connection, changes ownership, and waits for the advertised event.
No event arrives in either byte order. This independently confirms the existing
t063 finding; no duplicate task was created. Its existing plan remains the
scope owner, with this case supplying external socket evidence.

## Extension error classification

The first failure in `extension_errors` is Present minor 255. Sophia returns
BadImplementation (17), with the correct major/minor/sequence, instead of
BadRequest (1). `wire/extensions/present.rs` categorizes every unmatched minor
as PresentUnimplemented; its dispatcher then returns BadImplementation.

t088 must distinguish an unknown minor from a recognized operation that is
not implemented, preserving explicit completion and healthy-client continuation.
The core [X11 specification](https://xorg.freedesktop.org/archive/X11R7.7/doc/xproto/x11protocol.html)
defines Request for an invalid major/minor opcode; the inspected XLibre
`Xext/present/present_request.c` also returns BadRequest after its dispatch
switch. The current grouped case stops at this first refusal mismatch; it does
not establish that later extensions' error classifications passed.

## XTS and reference-test limits

Reviewed yserver's `xts-run.sh`, `xts-vs-baseline.py` and standalone XCB/Xlib
probes. The comparator accepts baseline PASS becoming NORESULT, UNSUPPORTED,
UNTESTED or NOTINUSE and ignores missing candidate purposes for its exit.
Those semantics were not reused. Its hardcoded /home/jos paths and live-display
launcher were not run.

XLibre's `test/pyxtest` and `test/xi2` provide useful independent framing,
malformed-request and swapped-byte-order patterns. No XLibre/Xorg/Xvfb server
or hardware test was launched, and no external source was copied.

The optional runnable XTS adapter uses a private copied suite, fresh configuration
and journal, isolated socket/network/device namespaces and exact mandatory TET
purpose accounting. Dependency preflight reports missing separate
`~/src/xts/check.sh`, built `xts5`, and TET `tcc`; a real selected scenario and
purpose manifest also require that build. **No actual XTS5 suite ran.**

The adapter was executed with explicitly synthetic fixtures: PASS exits 0;
PASS-to-NORESULT, a missing selected purpose and timeout after a PASS journal
each exit 1. Evidence is in `.artifacts/x11-conformance/synthetic-xts-*`;
the dependency report is in `xts-dependencies`. These prove adapter mechanics,
not XTS coverage. Missing dependencies remain an explicit t057 integration
limit rather than a fabricated skipped-suite success.

## Validation and remaining work

Twenty gate/reporting regressions pass, including absolute timeout despite
continuing output, empty/missing results, duplicate records, unexecuted manifest
obligations, numeric/textual TET disagreement, inventory drift and Python -O
refusal. The new host passes Clippy with warnings denied. `cargo fmt --all
--check` and the X-authority's offline all-target tests pass, with inherited
SOPHIA_/HAGIA_ opt-ins and display endpoints removed.

The full `cargo xtask check` was not run: it invokes render-node hardware proofs,
which the operator explicitly excluded from this assignment. No physical or
pinentry acceptance is claimed. The user-facing gate deliberately remains red
on the retained baseline until the mandatory protocol repairs land.

The new priority order is recorded only in todo.md. Gate implementation,
remaining core/extension coverage, missing XTS dependencies and individual
runtime repairs have separate exits; none is closed by an encoder round trip.

## Connections

- [Destruction-family record](ksbt5d8f-the-window-destroy-family-is-incomplete-beyond-destroynotify.md)
  owns t087 and cites the independently verified partial repair.
- [t063 plan](../plans/queue-11-parallel-production-readiness.md#t063) owns the
  previously filed XFIXES event omission.
- [t082 investigation](iux6ctsy-pinentry-submission-stalls-before-gui-exit-and-input-recovery-remains-blocked.md)
  remains separate. These protocol findings are not proof of its native-loop cause.
- [Family conformance](../plans/queue-09-cp-15-2-one-family-level-conformance-surface.md#t023)
  concerns Sophia's native role protocols; this X11 gate does not close it.
