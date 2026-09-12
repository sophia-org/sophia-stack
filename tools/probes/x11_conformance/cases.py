"""Externally observed obligations, independent of Sophia dispatch internals."""
import time
from wire import Client


def client(context):
    return Client(context['socket'], context['order'], context['deadline'])


def peer_client(context):
    order = '>' if context['order'] == '<' else '<'
    return Client(context['socket'], order, context['deadline'])


def setup(context):
    with client(context) as a, peer_client(context) as b:
        assert a.base != b.base and a.mask == b.mask
        assert a.root == b.root
        a.sync()
        b.sync()


def window_tree(context):
    with client(context) as c:
        parent = c.window()
        child = c.window(parent)
        reply = c.reply(15, c.pack('I', parent))
        assert c.u32(reply, 8) == c.root
        assert c.u16(reply, 16) == 1 and c.u32(reply, 32) == child
        geometry = c.reply(14, c.pack('I', child))
        assert c.unpack('hhHH', geometry, 12) == (7, 9, 80, 60)


def window_transition(context):
    with client(context) as c:
        wid = c.window()
        step = context['case']
        if step in ('map', 'unmap'):
            c.send(8, c.pack('I', wid))
            c.sync()
            c.event(19, lambda e: c.u32(e, 8) == wid)
            reply = c.reply(3, c.pack('I', wid))
            assert reply[26] == 2, 'mapped window is not Viewable'
        if step == 'configure':
            c.send(12, c.pack('IHHIIII', wid, 15, 0, 21, 23, 101, 79))
            c.sync()
            event = c.event(22, lambda e: c.u32(e, 8) == wid)
            assert c.unpack('hhHH', event, 16) == (21, 23, 101, 79)
            assert c.unpack('hhHH', c.reply(14, c.pack('I', wid)), 12) == (21, 23, 101, 79)
        if step == 'unmap':
            c.send(10, c.pack('I', wid))
            c.sync()
            c.event(18, lambda e: c.u32(e, 8) == wid)
            assert c.reply(3, c.pack('I', wid))[26] == 0
        if step == 'destroy':
            sequence = c.send(4, c.pack('I', wid))
            c.sync()
            event = c.event(17, lambda e: c.u32(e, 8) == wid)
            assert c.u32(event, 4) == wid and c.u16(event, 2) == sequence
            c.completion(c.send(3, c.pack('I', wid)), error=3, opcode=3, resource=wid)
            c.sync()


def reply_errors(context):
    with client(context) as c:
        wid = c.window()
        bad = c.xid()
        c.completion(c.send(3, c.pack('I', bad)), error=3, opcode=3, resource=bad)
        c.completion(c.send(3), error=16, opcode=3)  # BadLength, then healthy request.
        c.completion(c.send(255), error=1, opcode=255)
        c.send(127)  # NoOperation has no reply; next sequence must complete.
        assert c.u16(c.reply(14, c.pack('I', wid)), 16) == 80


def destroy_subscribers(context):
    with client(context) as owner, peer_client(context) as watcher, client(context) as silent:
        parent = owner.window(events=1 << 19)
        wid = owner.window(parent, events=1 << 17)
        # Both masks on one subscriber; a different connection performs destroy.
        for target, mask in [(parent, 1 << 19), (wid, 1 << 17)]:
            watcher.send(2, watcher.pack('III', target, 1 << 11, mask))
        watcher.sync()
        silent.sync()
        owner.events.clear()
        owner.send(4, owner.pack('I', wid))
        owner.sync()
        forms = [watcher.event(17), watcher.event(17)]
        assert {(watcher.u32(e, 4), watcher.u32(e, 8)) for e in forms} == {(wid, wid), (parent, wid)}
        own_forms = [owner.event(17), owner.event(17)]
        assert {(owner.u32(e, 4), owner.u32(e, 8)) for e in own_forms} == {(wid, wid), (parent, wid)}
        # The subscriber event barrier proves routing has happened before the
        # no-subscription check; a quiet read alone could race dispatch.
        silent.sync()
        assert not any(e[0] & 127 == 17 for e in silent.events)
        watcher.sync()
        assert not any(e[0] & 127 == 17 for e in watcher.events), 'duplicate DestroyNotify'


def destroy_family(context):
    with client(context) as owner, peer_client(context) as watcher:
        parent = owner.window(events=1 << 17)
        child = owner.window(parent, events=1 << 17)
        grandchild = owner.window(child, events=1 << 17)
        if context['case'] == 'destroy_descendants':
            owner.send(4, owner.pack('I', parent))
            owner.sync()
            for wid in (child, grandchild):
                owner.completion(owner.send(3, owner.pack('I', wid)), error=3, opcode=3, resource=wid)
            observed = [owner.u32(owner.event(17), 8) for _ in range(3)]
            assert observed == [grandchild, child, parent], observed
        elif context['case'] == 'destroy_subwindows':
            owner.send(5, owner.pack('I', parent))
            owner.sync()
            observed = [owner.u32(owner.event(17), 8) for _ in range(2)]
            assert observed == [grandchild, child], observed
            assert owner.u16(owner.reply(15, owner.pack('I', parent)), 16) == 0
            assert owner.u16(owner.reply(14, owner.pack('I', parent)), 16) == 80
        elif context['case'] == 'destroy_peer_close':
            for wid in (parent, child, grandchild):
                watcher.send(2, watcher.pack('III', wid, 1 << 11, 1 << 17))
            watcher.sync()
            owner.close()
            observed = [watcher.u32(watcher.event(17), 8) for _ in range(3)]
            assert observed == [grandchild, child, parent], observed
            watcher.sync()
        else:
            owner.send(4, owner.pack('I', grandchild))
            owner.sync()
            owner.event(17)
            for wid in (grandchild, owner.xid()):
                owner.completion(owner.send(4, owner.pack('I', wid)), error=3, opcode=4, resource=wid)
                owner.sync()
                assert not any(e[0] & 127 == 17 for e in owner.events), 'phantom DestroyNotify after BadWindow'


def destroy_subwindows_order(context):
    with client(context) as owner, peer_client(context) as watcher:
        parent = owner.window(events=0)
        first = owner.window(parent, events=0)
        first_leaf = owner.window(first, events=0)
        second = owner.window(parent, events=0)
        second_leaf = owner.window(second, events=0)
        for wid in (first, first_leaf, second, second_leaf):
            watcher.send(2, watcher.pack('III', wid, 1 << 11, 1 << 17))
        watcher.sync()
        # Move the newer sibling below the older one: allocation order must
        # disagree with stack order or an id-sorted implementation would pass.
        owner.send(12, owner.pack('IHHII', second, (1 << 5) | (1 << 6), 0, first, 1))
        owner.sync()
        owner.send(5, owner.pack('I', parent))
        owner.sync()
        observed = [watcher.u32(watcher.event(17), 8) for _ in range(4)]
        assert observed == [second_leaf, second, first_leaf, first], observed
        for wid in (first, first_leaf, second, second_leaf):
            watcher.completion(watcher.send(3, watcher.pack('I', wid)), error=3, opcode=3, resource=wid)
        assert owner.u16(owner.reply(15, owner.pack('I', parent)), 16) == 0
        watcher.sync()
        assert not any(e[0] & 127 == 17 for e in watcher.events), 'duplicate subtree destruction'


def destroy_subwindows_invalid(context):
    with client(context) as owner:
        parent = owner.window(events=(1 << 17) | (1 << 19))
        for _ in range(2):
            owner.send(5, owner.pack('I', parent))  # Empty subtree is a no-op.
            owner.sync()
            assert not any(e[0] & 127 == 17 for e in owner.events), 'empty subtree destroyed its parent'
        child = owner.window(parent)
        owner.send(5, owner.pack('I', parent))
        owner.sync()
        forms = [owner.event(17), owner.event(17)]
        assert {(owner.u32(e, 4), owner.u32(e, 8)) for e in forms} == {(parent, child), (child, child)}
        for wid in (child, owner.xid()):
            owner.completion(owner.send(5, owner.pack('I', wid)), error=3, opcode=5, resource=wid)
            owner.sync()
            assert not any(e[0] & 127 == 17 for e in owner.events), 'phantom event after invalid DestroySubwindows'
        assert owner.u16(owner.reply(14, owner.pack('I', parent)), 16) == 80


def destroy_peer_close_subscribers(context):
    with client(context) as owner, peer_client(context) as watcher, client(context) as silent:
        parent = owner.window(events=0)
        child = owner.window(parent, events=0)
        # The watcher owns neither resource, and selects both addressed forms.
        for wid, mask in [(owner.root, 1 << 19),
                          (parent, (1 << 17) | (1 << 19)), (child, 1 << 17)]:
            watcher.send(2, watcher.pack('III', wid, 1 << 11, mask))
        watcher.sync()
        silent.sync()
        owner.close()  # No DestroyWindow request: this is solely disconnect cleanup.
        observed = []
        try:
            for _ in range(4):
                event = watcher.event(17)
                observed.append((watcher.u32(event, 4), watcher.u32(event, 8)))
        except TimeoutError as error:
            raise TimeoutError(f'disconnect DestroyNotify forms before deadline: {observed}') from error
        expected = {(child, child), (parent, child), (parent, parent), (watcher.root, parent)}
        assert len(set(observed)) == 4 and set(observed) == expected, observed
        # Check addressing separately from the existing chain-order case.
        for wid in (parent, child):
            watcher.completion(watcher.send(3, watcher.pack('I', wid)), error=3, opcode=3, resource=wid)
        watcher.sync()
        assert not any(e[0] & 127 == 17 for e in watcher.events), 'duplicate disconnect destruction'
        silent.sync()
        assert not any(e[0] & 127 == 17 for e in silent.events), 'unsubscribed peer notified'
        live = watcher.window()
        assert watcher.u16(watcher.reply(14, watcher.pack('I', live)), 16) == 80


def destroy_mapped(context):
    with client(context) as owner:
        wid = owner.window(events=1 << 17)
        owner.send(8, owner.pack('I', wid))
        owner.sync()
        owner.event(19)
        assert owner.reply(3, owner.pack('I', wid))[26] == 2
        owner.events.clear()
        owner.send(4, owner.pack('I', wid))
        owner.sync()
        # Both events precede the barrier reply. Do not filter away a missing
        # automatic UnmapNotify or permit it to arrive after DestroyNotify.
        events = [e for e in owner.events if e[0] & 127 in (17, 18)]
        assert [e[0] & 127 for e in events] == [18, 17], [e.hex() for e in events]
        assert all(owner.unpack('II', e, 4) == (wid, wid) for e in events)
        assert events[0][12] == 0  # from-configure = False
        owner.completion(owner.send(3, owner.pack('I', wid)), error=3, opcode=3, resource=wid)


def property_values(context):
    with client(context) as c:
        wid, atom = c.window(), c.atom('SOPHIA_CONFORMANCE_PROPERTY')
        name = b'SOPHIA_CONFORMANCE_PROPERTY'
        reply = c.reply(17, c.pack('I', atom))
        assert c.u16(reply, 8) == len(name) and reply[32:32+len(name)] == name
        for fmt, values in [(8, b'a\0bc'), (16, [0x1234, 0xabcd]), (32, [0x12345678, 0xfedcba98])]:
            data = values if fmt == 8 else c.pack(('H' if fmt == 16 else 'I') * len(values), *values)
            count = len(data) * 8 // fmt
            c.send(18, c.pack('IIIB3xI', wid, atom, 6, fmt, count) + data)
            reply = c.reply(20, c.pack('IIIII', wid, atom, 6, 0, 32))
            assert reply[1] == fmt and c.u32(reply, 8) == 6
            assert c.u32(reply, 12) == 0 and c.u32(reply, 16) == count
            assert reply[32:32+len(data)] == data
            event = c.event(28, lambda e: c.u32(e, 4) == wid and c.u32(e, 8) == atom)
            assert event[16] == 0
        listed = c.reply(21, c.pack('I', wid))
        assert atom in c.unpack('I' * c.u16(listed, 8), listed, 32)
        c.send(19, c.pack('II', wid, atom))
        c.sync()
        assert c.event(28, lambda e: c.u32(e, 8) == atom)[16] == 1
        assert c.reply(20, c.pack('IIIII', wid, atom, 0, 0, 32))[1] == 0


def selection_owner(context):
    with client(context) as a, peer_client(context) as b:
        wa, wb, selection = a.window(), b.window(), a.atom('SOPHIA_CONFORMANCE_SELECTION')
        a.send(22, a.pack('III', wa, selection, 0))
        a.sync()
        assert b.u32(b.reply(23, b.pack('I', selection)), 8) == wa
        b.send(22, b.pack('III', wb, selection, 0))
        b.sync()
        event = a.event(29)
        assert a.u32(event, 8) == wa and a.u32(event, 12) == selection
        assert a.u32(a.reply(23, a.pack('I', selection)), 8) == wb


def selection_transfer(context):
    with client(context) as a, peer_client(context) as b:
        wa, wb = a.window(), b.window()
        sel, prop = a.atom('SOPHIA_CONFORMANCE_SELECTION'), a.atom('SOPHIA_CONFORMANCE_TRANSFER')
        a.send(22, a.pack('III', wa, sel, 0))
        a.sync()
        b.send(24, b.pack('IIIII', wb, sel, 31, prop, 0))
        b.sync()
        request = a.event(30)
        assert a.unpack('IIIII', request, 8) == (wa, wb, sel, 31, prop)
        a.send(18, a.pack('IIIB3xI', wb, prop, 31, 8, 4) + b'data')
        notify = bytes([31, 0]) + a.pack('HIIIIII', 0, 0, wb, sel, 31, prop, 0) + bytes(4)
        a.send(25, a.pack('II', wb, 0) + notify)
        a.sync()
        event = b.event(31)
        assert b.unpack('IIII', event, 8) == (wb, sel, 31, prop)
        assert b.reply(20, b.pack('IIIII', wb, prop, 31, 0, 4))[32:36] == b'data'


def selection_absent(context):
    with client(context) as c:
        wid, sel = c.window(), c.atom('SOPHIA_CONFORMANCE_UNOWNED')
        c.send(24, c.pack('IIIII', wid, sel, 31, 0, 0))
        c.sync()
        event = c.event(31)
        assert c.unpack('IIII', event, 8) == (wid, sel, 31, 0)


def focus(context):
    with client(context) as c:
        wid = c.window()
        c.send(8, c.pack('I', wid))
        c.sync()
        c.send(42, c.pack('II', wid, 0), detail=1)
        result = c.sync()
        assert c.u32(result, 8) == wid and result[1] == 1
        c.event(9, lambda e: c.u32(e, 4) == wid)
        c.send(42, c.pack('II', 0, 0), detail=0)
        assert c.u32(c.sync(), 8) == 0
        c.event(10, lambda e: c.u32(e, 4) == wid)


def grab(context):
    with client(context) as a, peer_client(context) as b:
        wa, wb = a.window(), b.window()
        for c, w in [(a, wa), (b, wb)]:
            c.send(8, c.pack('I', w))
            c.sync()
        keyboard = context['case'] == 'keyboard_grab'
        def acquire(c, w):
            if keyboard:
                return c.reply(31, c.pack('IIBB2x', w, 0, 1, 1))[1]
            return c.reply(26, c.pack('IHBBIII', w, 0, 1, 1, 0, 0, 0))[1]
        assert acquire(a, wa) == 0, 'first grab did not succeed'
        assert acquire(b, wb) == 1, 'competing grab must report AlreadyGrabbed'
        a.send(32 if keyboard else 27, a.pack('I', 0))
        a.sync()
        assert acquire(b, wb) == 0, 'released grab remained owned'
        b.send(32 if keyboard else 27, b.pack('I', 0))
        b.sync()


def disconnect(context):
    with client(context) as healthy:
        peer = client(context)
        wid = peer.window()
        sel = peer.atom('SOPHIA_CONFORMANCE_DISCONNECT')
        peer.send(22, peer.pack('III', wid, sel, 0))
        peer.sync()
        assert healthy.u32(healthy.reply(23, healthy.pack('I', sel)), 8) == wid
        peer.close()
        # Disconnect is asynchronous. Poll a real reply until cleanup becomes
        # observable, bounded by the original absolute deadline (never reset).
        while healthy.u32(healthy.reply(23, healthy.pack('I', sel)), 8) != 0:
            time.sleep(min(0.01, healthy.remaining()))
        healthy.completion(healthy.send(3, healthy.pack('I', wid)), error=3, opcode=3, resource=wid)
        live = healthy.window()
        assert healthy.u16(healthy.reply(14, healthy.pack('I', live)), 16) == 80


def extensions(context):
    with client(context) as c:
        if context['case'] == 'policy_absence':
            for name in context['denied_extensions']:
                assert c.query_extension(name)[8] == 0, ('intentional absence changed', name)
            assert c.query_extension('SOPHIA-NONEXISTENT-CONFORMANCE')[8] == 0
            return
        if context['case'] == 'extension_discovery':
            opcodes = []
            for name in context['extensions']:
                reply = c.query_extension(name)
                assert reply[8] == 1 and reply[9] >= 128, (name, reply.hex())
                opcodes.append(reply[9])
            assert len(opcodes) == len(set(opcodes)), 'extension opcode collision'
            for name in context['fixture_absence']:
                assert c.query_extension(name)[8] == 0, ('fixture unexpectedly exposes device extension', name)
            return
        result = c.reply(99)
        names, offset = [], 32
        for _ in range(result[1]):
            size = result[offset]
            names.append(result[offset+1:offset+1+size].decode('ascii'))
            offset += size + 1
        expected = set(context['extensions'])
        assert len(names) == len(set(names)) and set(names) == expected, ('extension inventory drift', names)


def extension_versions(context):
    with client(context) as c:
        # Requests from the corresponding public extension specifications.
        versions = {'Present': (0, 'II', (1, 2)),
                    'XFIXES': (0, 'II', (6, 0)), 'RENDER': (0, 'II', (0, 11)),
                    'RANDR': (0, 'II', (1, 6)), 'GLX': (7, 'II', (1, 4)),
                    'XC-MISC': (0, 'HH', (1, 1)), 'XKEYBOARD': (0, 'HH', (1, 0)),
                    'XInputExtension': (47, 'HH', (2, 4)),
                    'Generic Event Extension': (0, 'HH', (1, 0))}
        for name, (minor, fmt, requested) in versions.items():
            op = c.query_extension(name)[9]
            reply = c.reply(op, c.pack(fmt, *requested), detail=minor)
            actual = c.unpack(fmt, reply, 8)
            assert (0, 0) < actual <= requested, (name, requested, actual)
        for name in ('SHAPE', 'MIT-SHM', 'XFree86-VidModeExtension'):
            reply = c.reply(c.query_extension(name)[9])
            assert c.u16(reply, 8) >= 1, (name, reply.hex())
        reply = c.reply(c.query_extension('SYNC')[9], c.pack('BB2x', 3, 1))
        assert (reply[8], reply[9]) == (3, 1)
        reply = c.reply(c.query_extension('BIG-REQUESTS')[9])
        assert c.u32(reply, 8) >= 65535


def extension_errors(context):
    with client(context) as c:
        for name in context['extensions']:
            op = c.query_extension(name)[9]
            assert op >= 128
            c.completion(c.send(op, detail=255), error=1, opcode=op, minor=255)
            c.sync()


def shape(context):
    with client(context) as c:
        op, wid = c.query_extension('SHAPE')[9], c.window()
        c.reply(op)  # QueryVersion
        c.send(op, c.pack('BBBBIhh', 0, 2, 0, 0, wid, 0, 0) +
               c.pack('hhHH', 3, 4, 17, 19), detail=1)
        reply = c.reply(op, c.pack('IB3x', wid, 2), detail=8)
        assert c.u32(reply, 8) == 1
        assert c.unpack('hhHH', reply, 32) == (3, 4, 17, 19)


def sync_counter(context):
    with client(context) as c:
        op = c.query_extension('SYNC')[9]
        c.reply(op, c.pack('BB2x', 3, 1))
        counter = c.xid()
        c.send(op, c.pack('IiI', counter, 0, 41), detail=2)
        reply = c.reply(op, c.pack('I', counter), detail=5)
        assert c.unpack('iI', reply, 8) == (0, 41)
        c.send(op, c.pack('IiI', counter, 0, 1), detail=4)
        assert c.unpack('iI', c.reply(op, c.pack('I', counter), detail=5), 8) == (0, 42)
        c.send(op, c.pack('I', counter), detail=6)
        c.sync()


def xfixes_selection(context):
    with client(context) as owner, peer_client(context) as watcher:
        ext = watcher.query_extension('XFIXES')
        op, event_base = ext[9], ext[10]
        watcher.reply(op, watcher.pack('II', 5, 0))
        window, watched = owner.window(), watcher.window()
        selection = owner.atom('SOPHIA_CONFORMANCE_XFIXES')
        watcher.send(op, watcher.pack('III', watched, selection, 1), detail=2)
        watcher.sync()
        owner.send(22, owner.pack('III', window, selection, 0))
        owner.sync()
        event = watcher.event(event_base)
        assert event[1] == 0
        assert watcher.unpack('III', event, 4) == (watched, window, selection)


def disconnect_grab(context):
    with client(context) as healthy:
        peer = client(context)
        a, b = peer.window(), healthy.window()
        for c, wid in ((peer, a), (healthy, b)):
            c.send(8, c.pack('I', wid))
            c.sync()
        def acquire(c, wid):
            return c.reply(26, c.pack('IHBBIII', wid, 0, 1, 1, 0, 0, 0))[1]
        assert acquire(peer, a) == 0
        assert acquire(healthy, b) == 1
        peer.close()
        while acquire(healthy, b) != 0:
            time.sleep(min(.01, healthy.remaining()))
        healthy.sync()


def truncated_peer(context):
    with client(context) as healthy:
        peer = client(context)
        wid = peer.window()
        # Declared two-word GetWindowAttributes, only half its body sent.
        peer.sock.sendall(bytes([3, 0]) + peer.pack('H', 2) + bytes(2))
        peer.close()
        # Ask about the old resource until its cleanup is visible; a failed
        # peer must not take down an already-connected healthy worker.
        while True:
            sequence = healthy.send(3, healthy.pack('I', wid))
            reply = healthy.record()
            if reply[0] == 0:
                assert reply[1] == 3 and healthy.u16(reply, 2) == sequence
                break
            assert reply[0] == 1 and healthy.u16(reply, 2) == sequence
            time.sleep(min(.01, healthy.remaining()))
        healthy.sync()


def destroy_xid_reuse(context):
    with client(context) as owner, peer_client(context) as old_watcher:
        wid = owner.window(events=1 << 17)
        old_watcher.send(2, old_watcher.pack('III', wid, 1 << 11, 1 << 17))
        old_watcher.sync()
        owner.send(4, owner.pack('I', wid))
        owner.sync()
        owner.event(17)
        old_watcher.event(17)
        assert owner.window(events=1 << 17, xid=wid) == wid
        owner.send(4, owner.pack('I', wid))
        owner.sync()
        owner.event(17)
        old_watcher.sync()
        assert not any(e[0] & 127 == 17 for e in old_watcher.events), 'old subscription survived XID reuse'


CASES = {'setup': setup, 'window_tree': window_tree, 'map': window_transition,
         'configure': window_transition, 'unmap': window_transition, 'destroy': window_transition,
         'reply_errors': reply_errors, 'property_values': property_values,
         'selection_owner': selection_owner, 'selection_transfer': selection_transfer,
         'selection_absent': selection_absent, 'focus': focus, 'pointer_grab': grab,
         'keyboard_grab': grab, 'disconnect': disconnect, 'extensions': extensions,
         'destroy_subscribers': destroy_subscribers, 'extension_discovery': extensions,
         'policy_absence': extensions, 'extension_versions': extension_versions,
         'extension_errors': extension_errors, 'shape': shape, 'sync_counter': sync_counter,
         'xfixes_selection': xfixes_selection, 'disconnect_grab': disconnect_grab,
         'truncated_peer': truncated_peer, 'destroy_descendants': destroy_family,
         'destroy_subwindows': destroy_family, 'destroy_peer_close': destroy_family,
         'destroy_invalid': destroy_family, 'destroy_xid_reuse': destroy_xid_reuse,
         'destroy_subwindows_order': destroy_subwindows_order,
         'destroy_subwindows_invalid': destroy_subwindows_invalid,
         'destroy_peer_close_subscribers': destroy_peer_close_subscribers,
         'destroy_mapped': destroy_mapped}
