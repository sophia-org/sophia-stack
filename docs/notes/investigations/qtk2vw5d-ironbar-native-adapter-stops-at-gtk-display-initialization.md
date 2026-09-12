---
id: qtk2vw5d
date: 2026-09-12
kind: investigation
status: investigating
tags: [investigation, shell, ironbar, gtk, adapter]
---
# Ironbar native adapter stops at GTK display initialization

## Scope and decision gate

The operator approved a staged native ironbar plan: first prove the downstream
GTK adapter without an application-facing display connection, then implement the
specified content lifecycle, independent-client conformance and attended
acceptance. If toolkit initialization blocks the first probe, retain the failure
and stop rather than silently replacing the UI or adding a private display
server. That stop gate has been reached.

This is t081 evidence. It does not concern the pinentry application, t082 or the
preflight-triggered authority exit. No operator display, session bus, native GPU,
installed binary or configuration was touched by these probes.

## Sources and implementation

Ironbar baseline: `e2910c7` on `feat/sophia-workspaces`, clean before work.
Probe commit: `163eefb`, branch `feat/sophia-native-adapter-probe`, isolated
worktree `/tmp/ironbar-native-adapter`. The shared checkout and its revision-6
indicator decoder files were not changed.

- `src/main.rs` initializes Wayland and performs a roundtrip before activation.
- `src/bar.rs` constructs `GtkApplicationWindow`, `GtkCenterBox` and `GtkBox`.
- `src/popup.rs` constructs `GtkPopover` and retains module widget contents.
- The locked gtk4 Rust crate 0.10.1 calls native `gtk_init_check` in `src/rt.rs`
  before ordinary widget use. The C prerequisite probe calls that exact native
  symbol, not the C header's ABI-check wrapper.

The probe intentionally stops before widget construction. It is not a full Rust
ironbar adapter, and no widget rasterization, action dispatch or teardown proof
is claimed. A successful initialization would be reported as initialization-only,
never native adapter acceptance. Normal ironbar code was left intact after the
prerequisite failed; speculative platform refactoring would not remove this gap.

## Retained evidence

- `.artifacts/ironbar-native-adapter-e2910c7`: initial isolated run. Includes a
  missing Fontconfig configuration diagnostic; retained as the initial specimen.
- `.artifacts/ironbar-native-adapter-e2910c7-fonts`: decisive rerun with read-only
  `/etc/fonts`. The Fontconfig diagnostic disappears and all GTK results persist.
- `.artifacts/ironbar-native-probe-sources-20260912`: source copies and checksums.
  The reconstructed initial runner matches the first run's recorded source hash.

Each run retains build commands, base commit, native GTK library and executable
hashes, compiled backend list, exact isolated commands, per-case stdout/stderr
and a structured report. Output directories are never reused.

Native GTK version is **4.22.4**, with compiled backends `broadway wayland x11`.
The second run produced:

| Selection | Result | Exit |
| --- | --- | --- |
| Automatic | gtk_init_check returned false | 2 |
| Broadway | gtk_init_check returned false | 2 |
| Wayland | gtk_init_check returned false | 2 |
| X11 | gtk_init_check returned false | 2 |
| Headless | Unsupported backend; initialization false | 2 |

The explicit headless stderr says `No such backend: headless`. All cases passed
the isolation checks and completed without watchdog termination. Four Python
regressions verify classification, reject invalid isolation and prove that an
initialization-only success cannot be reported as adapter acceptance. C fixtures
build with warnings treated as errors.

## Isolation and observation limits

Bubblewrap unshares network, PID, IPC and mount namespaces, clears environment,
and exposes only `/usr`, read-only font configuration and the probe directory.
`/tmp`, `/run`, HOME and minimal `/dev` are private. Application display paths,
credentials, host session bus and GPU devices are absent. Cairo is selected;
stdin is closed and each owned process has wall/CPU/output bounds.

A bounded libc-connect observer passed its positive control against a nonexistent
socket inside the private namespace. Automatic/Broadway cases logged local
connect failures. It is an interposer, not a complete syscall audit: direct
syscalls may bypass it. Host endpoint exclusion follows from namespaces and
mounts, not the absence of observer records. The fixture reproduces the relevant
native-shell endpoint exclusions; it is not the complete admitted role domain.

## Finding and next boundary

Bypassing ironbar's unconditional Wayland startup does not make this GTK build
usable in the native-shell environment. GTK itself cannot initialize with the
available backends and no display endpoint. `GSK_RENDERER=cairo` changes rendering
selection, not GDK platform initialization.

This establishes a prerequisite failure for the tested build, not a universal
impossibility for all GTK versions or custom backends. Continuing with GTK needs
explicit downstream GDK platform/display integration. Granting a separately
admitted display path or replacing the toolkit are alternative architecture
changes, not automatic fallbacks authorized by this result.

The content lifecycle in ADR 6ndjwffd is still unimplemented, independently of
this GTK blocker. No Engine content work, new application frontend, private
server or replacement UI was introduced. t081 remains open; no panel, popout,
workspace activation or physical acceptance is claimed.

## Connections

- [Content design](../decisions/6ndjwffd-content-capability-design-for-sophia_shell_v1.md)
  defines generic content resources and presentation/input ownership.
- [Indicator delivery gap](wjctvtsk-indicator-delivery-does-not-yet-provide-native-panel-presentation.md)
  separates the implemented data consumer from the missing presentation path.
- [Native shell reference audit](../../shell-reference-client-audit.md) requires
  platform feasibility independent of application-facing display connections.
