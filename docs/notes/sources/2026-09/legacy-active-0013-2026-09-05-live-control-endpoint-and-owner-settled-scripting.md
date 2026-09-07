---
id: legacy-active-0013
date: 2026-09-05
recorded_date: 2026-09-05
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security"]
---
# 2026-09-05: live control endpoint and owner-settled scripting

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 440–478. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Implemented the opt-in Linux session service and `sophia msg`. The desktop
  profile accepts startup-only `session.control "disabled"|"host-admin"`;
  disabled remains the default. Host launches receive the session's fresh
  private socket path. No installed profile or running session was changed.
- Scoped admission to tested Linux evidence: session UID, socket-derived
  `SO_PEERPIDFD`, pinned proc/namespace handles, matching user/mount/PID
  namespaces, supervised identity exclusion, and dispatch-time recheck.
  Sophia's protected roles are excluded through their namespaces, mount grants,
  cleared environments, and descriptor allowlists. This does not attest
  third-party sandboxes sharing those namespaces or prevent trusted host
  proxies; the broader earlier OS-confinement wording is superseded.
- Kept byte parsing and peer inspection in a bounded control worker (32 peers,
  16 active calls, one outstanding request per peer). Owner tickets preserve
  exact catalog mapping and ordered action correlation through Engine commit.
  Queued deadline/disconnect cancellation cannot later dispatch; dispatched
  timeout reports indeterminate and retains bounded settlement ownership.
- Exposed policy actions and confirmed `restart-wm`; left `reload-profile`
  unadvertised until transactional recovery is repaired. Scripted restart moves
  process/transport waits off the owner and completes only after the intended
  replacement commits. WM/shell role wires and spatial ownership are unchanged.
- The supervised owner test exposed two lifecycle defects: a full policy event
  queue could deadlock worker Drop, and bubblewrap parent-death protection
  killed a replacement when its temporary spawning thread returned. Shutdown
  now disconnects the event receiver before join; each scripted replacement
  retains its launch thread until that process is retired. Consecutive
  restarts transfer that lifetime without moving process waits onto rendering.
- Evidence: generated valid/malformed vectors, independent Python and Rust real
  clients, CLI outcomes, strict config opt-in, namespace denial with a visible
  socket, ancillary FD cleanup (including peek/truncation), sequencing,
  pressure, deadlines, and supervised live-owner commit/restart correlation.
  `tools/check_control_protocol.sh --live-owner` reproduces these checks with
  temporary endpoints. Optional installed input/render observation remains
  separate and does not reopen the 36-row physical matrix.
- Final checks passed: offline workspace tests, all 299 native-session unit
  tests, the control gate with supervised owner proof, native output ownership
  tests, native CLI Clippy with warnings denied, formatting, and diff checks.

<!-- END IMPORTED BODY -->
