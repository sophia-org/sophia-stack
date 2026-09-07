---
id: legacy-active-0109
date: 2026-08-07
recorded_date: 2026-08-07
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "policy", "architecture"]
---
# 2026-08-07: The public policy protocol is the extension point

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 3515–3642. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Hagia is Sophia's first planned native WM, not a privileged Engine component or
the definition of Sophia policy. Sophia will publish independently
implementable, role-specific local IPC so other WMs and shells may be written
in any language. The first native proof, the Rust X11 WM bridge, and an
independently compiled C client must use the same wire and semantic conformance
suite.

River supplies the decisive architectural precedent: its compositor and WM are
separate processes joined by a stable protocol, which permits replacement and
hot-swap without moving rendering into policy. Sophia does not adopt River's
Wayland runtime. Unlike River, Sophia has retired its production Wayland
frontend and already has opaque generational IDs, bounded binary framing, and
Engine transactions. Importing a Wayland server solely for WM IPC would add a
second object/runtime model without improving Sophia's narrower blind-policy
boundary.

The wire remains the dependency floor. Clients need no Sophia library,
generator, Rust crate, Wayland stack, CBOR codec, or schema runtime. A narrow
checked-in KDL description will generate retained Rust and C99 codecs,
normative tables, and golden vectors; normal builds do not run the generator.
CBOR remains inappropriate for this authority path because its flexible maps,
duplicate keys, nesting, tags, and multiple equivalent encodings would require
a Sophia-specific restricted profile while buying little for fixed bounded
projection records.

The session, not the policy process, will host owner-only role sockets. This
aligns endpoint admission and hot-swap with session authority. Interface
versions are independent of the common frame version. The current Rust WM API
v7 is experimental; the first stable public family will be
`sophia_wm_v1` after Hagia and the bridge pass the same projection and recovery
suite. Once published, a stable revision remains accepted unless an explicit
security amendment retires it. Old revisions normalize at the IPC edge so
Engine retains one current internal projection model.

Complete scene snapshots and complete affected-output projections are the
semantic baseline. Strict begin/chunk/end transfer permits those records to
cross the existing 64-KiB frame boundary without exposing partial state.
Engine validates and atomically commits the affected logical outputs, preserves
the last projection on every failure, and permits a surface on at most one
output. Hagia privately owns nonempty tag sets, stable `ViewId` values, ordered
per-output views, focus history, reconnect affinity, and a session-local
checkpoint. Engine stores none of them. Mirroring remains a later separate
capability.

Two bounded TLA+ models precede production changes. `PolicyConnection` checks
negotiation, capabilities, transfer assembly, connection epochs, timeout,
disconnect, and replacement. `PolicyProjection` checks snapshots, stale
proposals, validation, multi-output atomicity, focus, removal, and
last-committed recovery. Wire offsets remain codec/golden-vector work rather
than TLA+ state.

The initial model checks exposed two protocol-level ambiguities before Rust
implementation. A transfer keyed only by client and connection epoch collided
with a later transaction on the same connection, so admitted work is identified
by client, epoch, and transaction, and a transaction cannot be reused within an
epoch. Separately, accepting any proposal whose declared base generation
equaled the current scene let a client guess a future generation and become
accidentally valid after a scene change. The session must issue the request for
the current generation, and a proposal must answer that exact outstanding
request. With those requirements, `PolicyConnection` passes 2,177 distinct
states to depth 23 and `PolicyProjection` passes 524,396 distinct states to
depth 18, including safety and liveness checks.

The first retained wire slice now derives ten draft handshake and transfer
message layouts from `protocol/sophia-wm-v1.kdl`. The generator emits the Rust
codec, an allocation-free C99 codec, normative byte tables, and shared valid
and malformed frame corpora. Generation is an explicit developer operation;
the ordinary build consumes only retained outputs. One check rejects generated
drift, round-trips every golden frame through both language implementations,
and requires the same fail-closed result for truncation, bad magic or version,
unknown kind, excessive payload, reserved data, trailing data, and invalid
transaction identity. Transfer ordering and semantic record validation remain
owned by the next bounded-assembler slice rather than the scalar codec.

The next draft slice implements that boundary without changing the installed
API v7 session. A Linux session-owned endpoint creates a new mode-0700 role
directory and mode-0600 WM socket, authenticates the exact supervised UID and
PID through peer credentials, and admits one exclusive client. Its connection
reducer negotiates one epoch, prohibits transaction reuse, accepts one bounded
transfer at a time, verifies ordinals and declared category totals, caps total
assembly memory, and discards complete queued work if its epoch disconnects
before Engine intake. Snapshot and projection assemblers share generated
fixed-width semantic records; all scalar and record bytes still round-trip
through the same retained Rust and allocation-free C99 artifacts.

Engine now has a dormant canonical projection reducer. It validates complete
scene snapshots, exact server-issued request identity, live surface
generations, constraints, geometry, output membership, global surface
uniqueness, and visible focus before replacing all affected outputs in one
mutation. Rejection, timeout, and disconnect preserve committed layout; scene
removal prunes only dead surfaces and invalid focus. A focused adapter converts
an API v7 workspace plan into this canonical shape, but production v7 remains
the installed owner until Milestone 12 promotion permits migration.
Deterministic tests preserve both formal counterexamples and a real Unix-socket
handshake-to-semantic-projection path.

The first ordinary client conversion exposed two missing facts in the draft
wire. A projection request named only its affected-output count, so an
independent policy could not know which complete outputs it had to replace. A
snapshot surface named current geometry but not its committed output, and no
snapshot record named current focus. A new policy therefore could not
distinguish hidden surfaces or reconstruct the active output state. The schema
now carries the bounded affected-output ID vector, an optional current output
(`0` means hidden), and optional per-output focus; regenerated Rust, C,
documentation, and golden artifacts agree on those fields.

Three non-production clients now exercise the corrected boundary. The dormant
Rust reference WM completes an authenticated snapshot/request/projection
cycle. The generic X11 bridge translates a real synthetic-X layout response
through API v7 and then the canonical reducer. A standalone C99 client assembles
the strict snapshot, tiles two opaque surfaces, and has its proposal accepted
by that reducer while linking only the retained C codec and libc. The protocol
gate retains the scalar malformed corpus and adds this live cross-language
cycle.

An initial uncommitted Triad clone was discarded when the project boundary was
clarified. Hagia starts with independent history, a Nim-only manifest, and no
River/Wayland dependency, binary, configuration, or build scaffolding. Its
long-term purpose remains a standalone Sophia port of Triad's useful policy and
experience, but that port is deliberately deferred. Hagia is currently a thin
independent protocol challenger: its decoder passes Sophia's valid and
malformed corpus, and its proof client completes strict snapshot assembly,
projection encoding, and committed outcome through the authenticated socket
and canonical reducer. Its private tag/view model remains incomplete, and none
of these draft paths changes the installed Milestone 12 candidate.

<!-- END IMPORTED BODY -->
