---
id: 744uylx4
date: 2026-09-07
kind: investigation
status: investigating
tags: [investigation, x11, session]
---
# Explicit pointer grabs must replace their own click lease

## Trigger and candidate

The user reported another Brave freeze after installing `e8573cf1`, session
`00000001788753082918-0a84405d-9e24-433d-950f-d3dce25a2607`. The installed binary
SHA-256 is `c3bb971a6925a9366480bb25eb6155d5fa1ebf8e177705d08f47d7f1787fa74f`.
The new recorder repeatedly shows a routed button, a rejected explicit-grab
request, and confirmation of the click's automatic lease. Releases then retire
that automatic lease. Neither no-target nor policy suppression accounts for
these routed batches. This identifies a reproducible grab failure, but does
not establish that every earlier browser hang or menu pixel defect shares it.

At 23:53:32 EDT, browser PID 12898 and GPU PID 12980 were waiting in
`poll_schedule_timeout.constprop.0`. The newest dump was from 23:50:03 EDT,
before this session started. This recurrence had no new GPU dump when sampled.

## Cause

The frontend installed an automatic grab without its Engine lease identity.
A subsequent explicit grab therefore requested a new lease instead of replacing
the click lease; Engine correctly refused a second owner. Even with the
identity attached, Engine allowed replacement only of explicit leases. Further
button presses also overwrote an existing frontend grab and could turn it into
an automatic grab, while every button release emitted an automatic release.

## Transition model

| Existing state | Event | Result |
| --- | --- | --- |
| No owner | Admitted button press | Automatic grab stores the exact Engine lease; frontend confirms delivery. |
| Automatic owner | Same client's explicit grab | Exact identity, admission, scope, seat, and authority epoch permit a provisional explicit replacement. |
| Automatic owner with acknowledgement in flight | Same explicit request | The same replacement is allowed; a late acknowledgement for the old identity cannot mutate the replacement. |
| Any active grab | Further button press | Preserve that grab's owner, modes, masks, and identity. |
| Automatic grab | Final button release | Release automatic ownership. |
| Explicit grab | Button release | Preserve ownership; no automatic-release update. |
| Explicit grab | Explicit ungrab | Complete the existing exact release handshake. |
| Releasing or foreign owner | Replacement request | Refuse without changing the owner. |

Engine still owns admission and physical routing. The frontend owns X grab
semantics. The WM and shell acquire no new authority or protocol fields.

## Correction and verification

The initial tests fail on the installed source: Engine returns `InvalidOrigin`
for click promotion, and button activation changes an existing owner's grab.
Three Engine lifecycle regressions and seven frontend authority-state tests
pass after the correction. The socket regression runs in both byte orders and
checks exact click identity in Prepare, activation, repeated button delivery,
absence of automatic releases, regrab identity, and explicit ungrab. All five
pointer socket tests pass. `cargo xtask check` passes, including workspace
tests, Clippy, source layout, fixture verifiers, and the host buffer-age proof.
The first full run hit stale PID-based config-test directories; a fresh
`TMPDIR` allowed the complete run to pass without changing those tests.
The final socket-test helper cleanup was verified with all five pointer socket
tests again. Evidence is retained at
`/home/niltempus/.local/state/sophia/development-evidence/t062-e8573cf1-a540da32e470`.
It contains the seven exact source/test files, source identity, full check log,
initial failing regressions, final socket results, and the redacted incident
snapshot. `SHA256SUMS` covers the retained files. The source identity is
`a540da32e4708d86bc97f28258341bfef9bd8a4882aafcddf07efbef5d0780ff`.

Physical acceptance requires ordinary Brave clicks and continued interaction
in an installed candidate, with no repeated own-grab rejections. This is t062; t003 remains the
broader Brave usability gate.

## Connections

- [Interaction diagnostics](ce2b55uy-blank-thunar-menus-and-frozen-brave-need-separate-pixel-and-delivery-evidence.md)
  made these rejections visible; t061 still owns Thunar's menu pixels.
- [Brave watchdogs](h0vxis10-brave-gpu-watchdog-repeats-during-live-use.md)
  remain separate evidence rather than an assumed consequence of these grabs.
- [Target-resolved input](../../target-resolved-input.md) owns the Engine and
  frontend lease contract.

## 2026-09-07: new installed candidate

The next live session is
`00000001788785369819-028cfec4-8b74-46ef-93b3-6c529dea2ddc`, running release
`4299e1cabb650cd481096f43ac8b5186d31ad4f1`. Its binary SHA-256 is
`9b152ebc7919635afb086b288bc4bc1cd747d752c55c077e318690d18c659386`.
The release contains `bed135ea` and subsequent XFIXES work. At the first health
sample, recording was running with zero discarded records and storage errors.
The user has confirmed login, but has not yet reported Brave interaction on this
candidate; this is startup identification, not physical acceptance of the repair.

The newer XFIXES selection-notification follow-up arrived with the same task ID.
It is now t063 in the [parallel plan](../plans/queue-11-parallel-production-readiness.md#t063);
t062 retains its original grab-repair acceptance identity.
