---
id: legacy-active-0532
date: 2026-08-24
recorded_date: 2026-08-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "x11", "tooling"]
---
# 2026-08-24: the Present source fix holds; XInput1 device enumeration does not

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16353–16379. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The signed rerun on `0505cb19` completed the whole guide. `Super+B` admitted Helium
from a CPU backing snapshot as surface 18874371, and that surface composed through
committed generation 265 on `source=cpu handle=4` -- the exact surface and handle
whose fifth generation ended the previous run. The proof phrase was accepted with 34
of 34 expected events and a confirmed pixel change, and the session shut down through
the ordinary lifecycle path. `MissingCpuSource` appears nowhere in the log. Sourcing a
queued Present from the candidate it plans is confirmed on hardware.

The run is still not promotion evidence, and what failed it is not what the session
did. The completion check counts 29 X protocol errors, the first a `BadRequest` for
major opcode 135 minor opcode 2, which is XInput1 `ListInputDevices`. `decode_x_input`
implements `GetExtensionVersion` at minor 1 and then the XI2 range, so the enumeration
a client performs immediately after the version handshake falls through to the
unknown-request arm. The client recovered on its own -- the browser rendered for the
full session -- but a normal session tolerates no protocol errors, so the run was
failed at the end for something it had already survived.

The X frontend has no device inventory to answer with. The seat enumerates 14 real
udev devices, but none of that crosses into the authority: `XiQueryDevice` synthesizes
a fixed virtual master pair, device 2 as pointer and device 3 as keyboard, and every
XI2 path hardcodes that pair -- grabs, ungrabs, client pointer, and event routing all
gate on 2 and 3. A legacy enumeration therefore has to report those same two virtual
devices. Reporting the real ones would need a frontend/session channel that does not
exist and would contradict the rest of the extension.

<!-- END IMPORTED BODY -->
