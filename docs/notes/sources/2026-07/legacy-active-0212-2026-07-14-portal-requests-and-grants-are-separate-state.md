---
id: legacy-active-0212
date: 2026-07-14
recorded_date: 2026-07-14
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security"]
---
# 2026-07-14: Portal Requests And Grants Are Separate State

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 7179–7190. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Portal policy decisions no longer need to double as execution authority. A
generic I/O-free lifecycle now retains deadline-bound request facts separately
from single-use grants. Allowed requests create active grants bound to source
generation and broker generation; completion, executor failure, expiry,
namespace disconnect, owner change, and broker restart have explicit terminal
transitions. A caller supplies monotonic time, the active set is capped at 64,
and no payload or operating-system handle enters this state. The first broker
IPC slice will use this reducer for every portal kind while clipboard remains
the first concrete executor.

<!-- END IMPORTED BODY -->
