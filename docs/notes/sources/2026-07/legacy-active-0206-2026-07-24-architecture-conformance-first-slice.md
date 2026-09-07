---
id: legacy-active-0206
date: 2026-07-24
recorded_date: 2026-07-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "architecture"]
---
# 2026-07-24: Architecture Conformance First Slice

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7018–7059. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Audited production and test source layout against `docs/style-guide.md` and
  `docs/dod.md`; added an executable exact-path exception ledger so existing
  debt is visible and new unreviewed violations fail validation.
- Advanced the blind WM contract to API v3. Launch requests now carry only an
  opaque nonzero `SessionApplicationId`; executable names and application roles
  remain session-owned. Existing CLI evidence labels remain stable.
- Removed the application-protocol name from the renderer's built-in default
  cursor without changing its dimensions, hotspot, or pixels.
- Removed allocation and sorting from Engine input hit testing and cached the
  backend visual runtime's input-layer projection at authority-update
  boundaries.
- Split the production visual runtime into a 795-line transaction/presentation
  facade plus focused native-scanout and asynchronous-service modules. Present
  feedback now crosses the backend/session boundary through a bounded owned
  queue, and KMS retirement performs explicit Engine commit, protocol feedback,
  and output projection steps instead of callback-owned mutation.
- Removed visual-state seeding from backend and CLI startup. The runtime begins
  empty, accepts initial generation zero only through normal Engine authority
  commit, and rejects a forged nonzero initial generation.
- Centralized authority-transaction layer templates under Engine, preserving
  namespace identity and stack order for both production and deterministic
  backend paths. Moved protocol cursor coverage to the crate boundary and
  removed implementation-only legacy-WM builder and atomic-helper inline tests.
- Replaced direct native-scanout library printing with `tracing` while
  preserving the stable evidence message bodies emitted through CLI-installed
  subscribers.
- Converted native EGL/GBM pixel and lifecycle diagnostics to `tracing` and
  removed the process ID from DMA-BUF lifecycle output. Pixel evidence message
  bodies remain compatible with the existing verifier.
- Converted X authority dispatch, socket-write, close, and input diagnostics to
  `tracing`. Request-byte prefixes, file descriptors, raw XIDs, and key details
  are now redacted; diagnostics retain only opaque client IDs, protocol
  opcodes/counts, routing decisions, and bounded timing.
- Converted the private legacy-WM opcode trace to `tracing`; the source-layout
  ledger now has no direct-library-printing exceptions.
- All workspace targets compile with all features. Focused protocol, Engine,
  renderer, backend, WM, and bridge tests pass. Strict workspace Clippy remains
  a tracked migration gate because pre-existing native renderer argument
  bundles and style warnings are not yet clean.

<!-- END IMPORTED BODY -->
