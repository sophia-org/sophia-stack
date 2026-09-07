---
id: legacy-active-0500
date: 2026-08-22
recorded_date: 2026-08-22
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security", "architecture"]
---
# 2026-08-22: the broker is hosted behind an executable protection boundary

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 15284–15334. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- A backend-neutral `ProtectionDomainSpec` now makes role composition, network
  denial, inherited descriptors, and filesystem grants part of supervised process
  launch. Blind spatial policy is rejected if it is composed with metadata shell,
  metadata/portal broker, or application-frontend roles. The first production
  backend is Bubblewrap 0.11.2: public Hagia and the metadata broker run with
  cleared environments, stdio-only descriptor inheritance, private
  user/PID/network/IPC/UTS/cgroup namespaces, no ambient home or temporary tree,
  and only declared role paths. Hagia retains one private writable checkpoint
  directory; its staged policy fragment and role sockets are read-only.
- Bubblewrap's JSON status descriptor was tested and rejected for this integration.
  It reports namespace identity, but retaining the status pipe across wrapper
  startup also made the descriptor visible to the role process, contradicting the
  descriptor boundary being proved. Sophia instead performs a bounded startup
  read of the wrapper's single `/proc/<pid>/task/<pid>/children` entry, then uses
  that exact host peer PID for `SO_PEERCRED` role admission. The protected broker
  smoke proves the peer differs from the wrapper and that host-only markers,
  display/session-bus variables, unexpected descriptors, and outbound TCP are not
  available inside the role.
- `sophia_broker_v1` is now a separate revisioned wire family with strict bounded
  codecs, handshake, connection epoch, transaction correlation, and an owner-only
  exact-peer transport. Reply waits are bounded on the session side while the role
  server may remain idle indefinitely; a regression waits beyond the five-second
  reply timeout before completing a request. Production Hagia sessions start the
  real `MetadataBroker` in its protected process. Surface admission returns a
  disclosure rule to the owning X Authority client; the authority aggregates its
  retained metadata and emits only a bounded reduced candidate; and the broker's
  sanitized descriptor commits to Engine's session-owned `ChromeDescriptorTable`.
  `ClassOnly` is the
  production default, so later title changes continue to reduce to the retained
  class rather than clearing or exposing the title. Surface removal retires both
  broker and Engine state. A native shell/display-list consumer is still absent.
- Pnut commit `32044e4a1eb945611686166c5d2422d9325364a7` was audited as the
  stronger long-term Rust backend. It already owns clone3 and pidfd supervision,
  but its public `Sandbox::run()` blocks and hides the role child in `Once` mode;
  `Execve` preserves process identity but cannot supply PID-namespace isolation.
  Sophia therefore cannot adopt it without weakening either exact peer admission
  or supervisor lifecycle. The prepared upstream patch fixes a separate Landlock
  V4 footgun by distinguishing omitted TCP allowlists (unrestricted) from explicit
  empty allowlists (deny all). All 21 focused Landlock network tests pass. Pnut's
  complete workspace suite reaches the same unrelated pre-existing rlimit failure:
  its million-descriptor clamp test cannot raise this host's hard `nofile=4096`
  limit. The backend decision and required spawn API are in
  `docs/pnut-evaluation.md`.
- Formatting, diff hygiene, offline metadata, installer/rollback checks, the
  feature-complete Sophia suite, and the protected broker transport all pass. These
  are local implementation results, not physical promotion evidence. The per-head
  pacing mirror and mixed-output gates remain open and ordered first; the hosted
  broker path still needs its real-session proof after those signed display reruns.

<!-- END IMPORTED BODY -->
