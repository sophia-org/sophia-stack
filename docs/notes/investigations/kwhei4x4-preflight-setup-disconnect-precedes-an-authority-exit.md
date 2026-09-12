---
id: kwhei4x4
date: 2026-09-12
kind: investigation
status: investigating
tags: [investigation, x11, authority, session, containment]
---
# Preflight setup disconnect precedes an authority exit

## Scope and evidence status

This is a separate Sophia authority-exit investigation, discovered during t082
acceptance preparation. It is not a pinentry application failure, an accepted
production pinentry result, or a revival of the historical GUI destructor theory.
No real pinentry candidate GUI was launched or installed by either agent.

The observed desktop exit and its timing are confirmed. Installed source supports
an uncontained setup-prefix EOF explanation. The retained structured events do
not contain the original error text, and no isolated confirmation is claimed
here. Runtime owner w9:p4 is independently checking that explanation offline.

## Immutable evidence

Preserved directory: `.artifacts/t082-preflight-crash-20260912`.
`preservation.json` lists eleven original paths, copies and SHA-256 values; all
eleven copied files were independently rehashed and matched. The README records
source reasoning and timing limits. Original logs and preserved copies were not
modified by this filing.

Session: `00000001789234962235-83db78c2-038d-4f78-8dd3-bf0280951fde`.
Installed/running release: `702efef161ddf758affdbf68bf139b00aa21a5be`.
X server PID: 31052, UID 1000, local display `:77`.
Executable SHA-256:
`77f5a4579f1431f8326bca3a736393b540fa81a9c1494c7527e7d59517bfc025`.
The installed executable and application candidate remained unchanged.

## Trigger and timing

Executed once from `/home/niltempus/dev/sophia-stack`, with host process visibility:

```sh
python3 -B tools/probes/t082_pinentry/foreground.py --candidate-manifest .artifacts/t082-upstream/candidate-c9c1eda9068a/manifest.json --preflight-only
```

The probe connected to `/tmp/.X11-unix/X77`, read `SO_PEERCRED`, and closed the
connection without sending an X11 handshake or any application bytes. It then
read the identified process executable/stat, verified the installed executable
hash, and saved identity metadata. It exited zero with “Identity preflight PASS;
no candidate launched.” No error diagnostic appeared in combined tool output;
separate stderr was not retained.

All times are UTC on 2026-09-12:

| Time | Evidence |
| --- | --- |
| 18:02:45.394816 | foreground.py write, lower bound before execution |
| 18:02:45.468732 | running-server.json write, upper bound after socket observation |
| 18:02:45.501 | events.0.log record 129187: owner-loop runtime fatal, unclassified |
| 18:02:45.776 | record 129196: session failure, phase authority, unclassified |
| 18:02:47.397 | outcome: failed, exit status 1 |

The first two timestamps are filesystem bounds, not exact execution or network
timestamps. Fatal detection followed the saved preflight identity by approximately
32 ms. This is strong temporal correlation; it does not supply the missing error
message. The separate command/output audit is preserved at
`/tmp/sophia-pinentry-trace/preflight-audit-20260912T180245.json`.

## Installed-source explanation awaiting isolated confirmation

At installed commit 702efef161dd:

- `crates/sophia-x-authority/src/x11_socket/connection/io.rs`,
  `read_x11_setup_request`: setup-prefix `read_exact` errors use
  `X11SetupSocketError::new`; setup-auth reads and parse errors also use it.
- `crates/sophia-x-authority/src/x11_socket.rs`: that constructor sets neither
  client-disconnect nor client-failure classification.
- `crates/sophia-x-authority/src/x11_socket/frontend/service.rs`,
  `reap_client_worker`: tagged client failures/disconnects are contained;
  untagged errors propagate from the worker result.

Thus a peer closing before supplying the setup prefix has a source-supported
route out of client containment. This is a setup-read classification issue,
separate from the previously repaired peer-write paths. A repair must preserve
fatal shared-authority and lock failures rather than classifying every error as
a departed peer.

## Required offline confirmation and suspension

Use only an isolated private authority fixture: connect and close before setup,
then require a healthy second client and continued frontend service. Include
truncated prefix/auth inputs, malformed setup and fatal non-peer controls as
appropriate to the verified scope. A helper-only classification assertion is not
proof of frontend continuation. No repetition against the operator display is
authorized.

All live preflight, GUI acceptance, installation and restart work remains
suspended. The existing foreground launcher is not approved for another run.
Pinentry PRs #16 (UTF-8) and #17 (Dialog) remain published. Candidate
`c9c1eda9068a588ee1b8d1b46aab94d3bce7d557` is unchanged, with no new native
acceptance; t082 remains open. Resume decisions must use this separate incident's
result and explicit operator authorization.


## Passive harness replacement

`foreground.py` no longer imports socket or contacts the X endpoint during
preflight. It reads the exact filesystem DISPLAY listener inode from
`/proc/net/unix`, finds a unique owned process holding that socket, compares its
executable path/hash to the installed release, and rechecks process starttime,
listener identity and ownership after hashing. Missing, ambiguous, inaccessible
or changed metadata fails closed. Eight isolated filesystem/mock tests cover
those boundaries and prohibit socket construction. No native rerun was made.

The operator subsequently authorized the application owner to deploy the exact
pinentry candidate locally with backup, atomic replacement, hash verification
and a no-display Assuan smoke only. That changes application deployment
permission, not native acceptance or permission to repeat the preflight. No
Sophia installation, display/VT test or restart is authorized by this update.

## Offline confirmation and repair

Tracked as **t089**. Confirmed and repaired in **5cb58d3a**, headlessly, with no
live server contacted.

The source chain holds as traced. `read_x11_setup_request`
(`x11_socket/connection/io.rs`) built every failure with
`X11SetupSocketError::new`, which leaves `client_disconnect`, `client_failure`
and `service_shutdown` all false. `reap_client_worker`
(`x11_socket/frontend/service.rs:468`) retires a worker quietly only when one of
those is set and otherwise returns the error, which leaves the service. So an
EOF from a departed peer reached the reaper indistinguishable from the server
breaking.

A frontend on a temporary socket reproduces it. Before the repair, a peer that
connected and closed without sending anything produced exactly the shape the
incident recorded:

```
X11SetupSocketError { message: "failed to read X11 setup prefix: failed to fill
whole buffer", client_disconnect: false, client_failure: false,
service_shutdown: false }
```

The repair classifies each stage a peer can reach: reads that end because the
peer is gone, writes of the setup answer into a client that already left, and
bytes that arrived and do not parse. Reads failing for any other reason keep the
unclassified form, so a genuine internal fault is still not absorbed.

Two further triggers of the same defect were found by audit rather than by the
incident, and are repaired with it. A client that completes a valid setup and
closes before reading the answer fails on the **write**, which the read
classification alone does not cover. A client that sends a request header
announcing a length and then leaves fails on the **request payload** read; the
request *header* read already treated a vanished peer as an ordinary end of
stream, so only the payload was exposed.

Each case asserts a healthy second client still completes setup on the same
frontend, because a test watching only the abandoned connection would pass
against a server that had already died. Each fails when its classification is
reverted, verified per site.

### Not established

That this path caused the 18:02:45.501 fatal. The mechanism is now demonstrable
and the timing is close, but the server retained no diagnostic and the kernel
ring buffer was never captured, so the crash itself remains circumstantial. This
host also has a recorded history of amdgpu DCN32 flip stalls, which is not
excluded.

`cargo xtask check` was **not** run for this commit. That gate starts sessions
and drives the GPU on this host, and it is held until the operator decides the
host is safe to exercise. The change is covered by the crate suite: 349 tests in
`x11_wire`, 22 suites green.
