---
id: legacy-active-0195
date: 2026-07-26
recorded_date: 2026-07-26
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-07-26: GetProperty Length Is A Ceiling, Not A Payload

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 6684–6711. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

A physical same-namespace Kitty copy/paste attempt terminated the receiving
Kitty with three opcode-20 X errors. The session used the `classic_shared`
namespace profile, emitted no `CloseFocused` action, and never entered the
portal path. Kitty 0.48.0's X11 selection receiver calls `XGetWindowProperty`
with `long_length=LONG_MAX` and `delete=True` to read the complete property.
Sophia incorrectly multiplied that request ceiling by four, compared it with
the 256 KiB retained-property limit, and returned `BadValue`. Xlib's default
error handling then terminated the client.

Core X11 semantics instead define the returned length as the minimum of the
stored remainder and four times the requested length. The property reducer now
saturates the request conversion and clamps it to already bounded retained
bytes; it never allocates from the request ceiling. The 256 KiB per-value and
4 MiB table limits remain unchanged. An offset beyond the actual property
still fails with `BadValue`.

`GetProperty(delete=True)` is now one explicit authority transition. A
complete type-matching read returns the reply, removes the property, and emits
the existing deletion notification; partial reads, type mismatches, missing
properties, and failed reads preserve state. Reducer, core-dispatch,
same-namespace multi-client, and cross-namespace portal regressions all use the
maximum wire length and prove exact bytes, no protocol error, and post-reply
deletion. No Kitty, xmonad, compositor, namespace-policy, or portal special
case was added. Physical copy/paste and the strict normal-session verifier
remain the promotion gate.

<!-- END IMPORTED BODY -->
