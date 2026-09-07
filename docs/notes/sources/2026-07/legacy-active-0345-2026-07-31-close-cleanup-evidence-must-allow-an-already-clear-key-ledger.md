---
id: legacy-active-0345
date: 2026-07-31
recorded_date: 2026-07-31
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "validation"]
---
# 2026-07-31: close cleanup evidence must allow an already-clear key ledger

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 10777–10792. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The same-commit QEMU M8 session completed all three close actions, normal
  application exits, clean input drain, and normal cleanup, but its verifier
  rejected Firefox because only two closes emitted nonzero `close_surface`
  key-clear records.
- Firefox had already released the same two keys during an earlier focus
  transition. Closing a surface with no remaining pressed keys correctly emits
  no nonzero key-clear record; requiring one for every close made promotion
  depend on input and lifecycle timing rather than final state.
- The verifier still requires three committed close actions, at least two
  demonstrated nonblocking close-time key clears, positive state-only
  releases, and a final key ledger with zero pending keys or release barriers.
  Its negative regression removes every close-time clear and must fail, while a
  new regression removes one clear to represent an already-clean close.

<!-- END IMPORTED BODY -->
