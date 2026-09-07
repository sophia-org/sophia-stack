---
id: legacy-active-0072
date: 2026-08-10
recorded_date: 2026-08-10
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy"]
---
# 2026-08-10: The trusted profile barrier now shares Hagia's proven semantics

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 2229–2374. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Sophia already validated and staged exact fragments for all seven desktop
  authorities, but the executable prepare/activate/rollback reducer existed
  only in Hagia's model-checked reference implementation.
- `sophia-config` now ports that transition shape over Sophia's canonical
  `DesktopAuthority`, `ConfigGeneration`, and `ConfigDigest` types. Effects are
  emitted in deterministic authority order; all preparations are required
  before activation; any prepare or activation failure rolls every authority
  back; and the active identity changes only after all matching activations.
- Rejected generations remain consumed, stale completions are inert, and
  generation exhaustion cannot wrap. Mirrored Rust tests cover success,
  rejection, partial activation, stale completion, failed rollback, and the
  rejected-generation reuse defect previously found through Hagia's formal
  verification. Authority protocols, production effect handlers, and watched
  reload remain deliberately disconnected.
- The CLI now has the corresponding injected startup executor seam, porting
  Hagia's typed-completion pattern without importing its runtime. Prepare,
  activate, and rollback dispatch to separate handlers and return the exact
  authority/generation/digest completion message. Only fakes populate those
  handlers today; production authority transports and watched reload remain
  deliberately disconnected.
- A synchronous startup driver now drains those effects through both barriers.
  It stops unused prepare or activation work after the first failure, attempts
  rollback for all seven authorities even if one rollback fails, preserves the
  previous active profile, and returns the exact pending recovery model.
  Deterministic tests enumerate every authority as each failure point, prove
  initial activation from empty state, and prove that rejected generations
  cannot be retried. The driver adds no asynchronous interleaving beyond the
  lifecycle already exhaustively checked in Hagia's TLA+ model.
- A reusable authority-local participant reducer now gives every authority the
  same transition discipline without copying handlers: strict monotonic
  prepare, exact-key idempotent prepare/activate/rollback, previous-active
  restoration, and inert strictly older cleanup. It retains the last admitted
  generation and digest after cleanup, which closes the ambiguity between an
  exact rollback retry and a same-generation digest mismatch. Tests apply the
  initial activation and rollback invariants to all seven authorities and
  cover retries, mismatch rejection, prior-active restoration, generation
  consumption, and exhaustion.
- Participant activation is not yet externally visible. Transactional startup
  may eventually use it behind the existing graphical launch gate, where no
  client observes partial activation. Watched reload needs a distinct global
  visibility/recovery protocol and remains out of scope for this milestone.
- Refinement testing found that the synchronous driver stops the remaining
  prepare batch after one rejection but deliberately rolls all authorities
  back. A participant skipped by that batch had treated the future rollback as
  an identity mismatch, which would turn every early preparation rejection
  into a false recovery failure and leave the skipped generation locally
  reusable. Idle or previously activated participants now consume that exact
  unseen rollback as a no-state tombstone while retaining the active identity;
  conflicting prepared work and same-generation digest mismatches still fail
  closed.
- The test-only executor now drives the coordinator's real effect sequence
  through seven independent authority-local candidate slots rather than
  identity-only participant models. Across every authority as the prepare,
  activation, and rollback failure, it proves successful identity and payload
  agreement, complete last-known-good payload restoration, rejected-generation
  consumption, exact single-authority divergence, and deterministic recovery.
  A staged integration case then loads the exact owner-safe fragments for all
  seven authorities through the shared admission function and proves semantic
  payload promotion under the coordinator's exact activation key.
- Startup tracing found one avoidable split in candidate ownership: the shared
  preparation result was partially moved into session fields, while public
  policy setup parsed the shortcut candidate again from the raw profile. The
  preparation boundary now checks all seven candidate identities against the
  profile generation/digest, session configuration retains one immutable typed
  bundle, and policy setup clones its shortcut value. This gives future
  authority handlers one canonical input without labeling preparation as
  activation or changing launch order.
  A later ownership pass partitions that bundle once into session, input, and
  output owner records plus the shortcut transfer payload; the aggregate is no
  longer retained as a second coordinator-owned copy.
- Public policy directory creation and fragment staging previously lived inside
  WM process construction. A linear `PreparedPublicPolicyLaunch` context now
  performs owner-only directory creation, stages and re-admits every authority
  fragment against the exact key, and prepares shortcut state immediately after
  trusted configuration parsing. Tests prove mode, complete fragment presence,
  exact admission, `Prepared` phase, and cleanup without launching a process.
- The synchronous startup driver previously fused its prepare and activate
  phases even though the graphical launch gate needs a stable prepared pause.
  `run_desktop_profile_startup_preparation` now returns a typed
  `Prepared`/`Rejected` report after settling exactly the prepare batch and any
  rollback, with no activation calls. The complete driver reuses it. Tests
  cover success plus every authority as the prepare rejection point.
- Public Hagia startup now invokes that prepare-only driver immediately after
  staging and exact fragment admission. A fixed-field dispatcher borrows the
  separate raw policy/shell/broker owners, the public shortcut owner, and the
  typed session/input/output owners; it owns no slot collection. Success leaves
  all seven participants and the coordinator at the same `Prepared` key before
  display setup. Injected failure at every authority proves generation-wide
  rollback, empty candidates, unchanged active identity, cleanup, and no
  process launch.
- The retained activation driver is now the only public-policy launch path.
  After the complete prepare barrier, six Sophia-local owner slots activate in
  canonical order and Hagia is started with the exact staged Policy identity;
  the coordinator promotes only after Hagia's matching completion. A launch
  timeout, disconnect, rejection, or participant failure drains generation-wide
  rollback before graphical construction. Removing the prepared-only branch is
  deliberately limited to synchronous startup visibility: watched reload still
  needs a separate global visibility and durable-recovery protocol. The former
  `--wm-profile-activation` proof switch remains accepted as a compatibility
  no-op, so installed launch profiles do not break while the unsafe choice is
  gone.
- `load_desktop_profile` had already run the shared preparation boundary before
  returning, so live-session startup's immediate second preparation was pure
  duplication. `load_prepared_desktop_profile` now returns a typed aggregate
  containing the validated provenance-bearing profile and its derived bundle;
  its activation key is projected directly from that same generation/digest.
  The raw loader delegates to this path and discards only the bundle when a raw
  caller explicitly requests that view.
- The next handler prerequisite reuses rather than extends the staged format.
  `load_desktop_authority_fragment` applies Sophia's existing absolute,
  regular, bounded, owner-safe file rules, rejects symlinks, parses exactly one
  assigned authority section, validates its settings, and requires the file's
  generation/digest to equal the coordinator key. Tests round-trip every
  fragment emitted by `stage_desktop_profile` and reject authority crossing,
  generation drift, digest drift, unsafe mode, and symlink substitution. Hagia
  retains its independent Nim reader as cross-implementation evidence.
- A generic authority-local candidate slot now joins admitted payloads to the
  participant reducer without duplicating its identity state machine. It owns
  active, previous-active, and candidate payloads and updates them only after a
  successful pure participant transition. Prepared typed candidates advertise
  their fixed authority and key; raw candidates advertise their assigned
  authority. Exact raw retries compare semantic settings rather than provenance
  paths, allowing in-memory and staged reconstruction of the same payload while
  rejecting changed content at one identity. Focused tests cover prepare,
  activate, restoration, retries, conflicts, invalid identity, unseen rollback
  tombstones, and fragment rejection with unchanged slot state.
- Session startup previously parsed and mutated CLI application overrides in
  the middle of configuration assembly. A typed immutable overlay now owns the
  bounded application additions, argument extensions, startup order, and action
  selectors. One pure preparation function applies the canonical session
  profile candidate and CLI-superior overlay to a cloned trusted registry.
  Existing profile/CLI parity tests still pass; new evidence covers identical
  preparation and rejection with the previously accepted state unchanged.
- The trusted session configuration now retains its canonical typed profile
  payload in the generic authority-local slot. Effective application
  preparation reads the slot's candidate rather than bypassing participant
  state. Tests prove the slot and canonical bundle carry the same payload and
  exact key in `Prepared` phase; startup assembly performs no activation.
- A shared `with_candidate` constructor now centralizes empty-slot creation and
  the initial prepare transition. The public shortcut owner uses it to retain
  the canonical typed shortcut candidate and resolves registrations from the
  slot payload. This keeps session and shortcut participant state with their
  respective owners and avoids a coordinator-owned slot collection.

<!-- END IMPORTED BODY -->
