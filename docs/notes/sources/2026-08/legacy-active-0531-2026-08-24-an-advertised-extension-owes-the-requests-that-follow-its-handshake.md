---
id: legacy-active-0531
date: 2026-08-24
recorded_date: 2026-08-24
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "general"]
---
# 2026-08-24: an advertised extension owes the requests that follow its handshake

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 16318–16352. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

Sophia advertises `XInputExtension`, answers the XI1 `GetExtensionVersion` handshake
claiming 2.0, and implements `DeviceBell`, whose own comment calls it the bounded
legacy XInput request. It then refused `ListInputDevices`, the enumeration a client
issues immediately after that handshake. Half an extension is worse than none: the
client recovered, but 29 `BadRequest` replies failed a session that had otherwise
completed its whole guide.

The enumeration is now answered. The interesting part was not the request but the
device table behind it. The frontend has no device inventory, and `XiQueryDevice`
built its fixed virtual master pair inline inside its own match arm, so the obvious
implementation was a second table beside it. That is the shape of the defect this tree
spent three rounds fixing in the Present path -- two views of one fact, free to drift.
The pair is now one passive table that both protocol versions project from. XI1 and
XI2 describe a device differently enough that the projections share no bytes: XI1
names the type with an atom, reports a `DeviceUse`, and has no vocabulary for the
scroll classes XI2 carries. A test walks both wire formats back into device records
and requires them to agree on identity, name, button count, and key range, because
with shapes that different nothing else would notice them parting.

The XI1 reply layout is not documented anywhere in this tree. It was read from the
X.Org protocol description as published in the `x11rb-protocol` crate, which is
already a resolved dependency of `sophia-cli`. That is a protocol description rather
than an X server implementation, so it sits inside the clean-room posture, which is
about not reproducing Xorg's object graph rather than about re-deriving published wire
formats. `sophia-x-authority` gains no dependency; the layout is written as explicit
offsets like every other reply in the crate. Two details differ from the neighbouring
XI2 encoder and are easy to get wrong: the device count is a single byte where XI2
puts a `u16`, and the body is three concatenated sections -- every device record, then
every class info, then every name -- rather than one self-contained record per device.
Names are `Str`, length byte and bytes, with no per-name padding.

The signed installed rerun remains the promotion gate.

<!-- END IMPORTED BODY -->
