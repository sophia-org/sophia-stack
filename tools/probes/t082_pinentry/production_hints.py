"""Read-only, bounded live X properties; never select a window by its title."""
import re
import subprocess
import threading
import time


def parse_properties(text, pid, xid):
    owner = re.search(r'^_NET_WM_PID\(CARDINAL\) = (\d+)$', text, re.M)
    if owner is None or int(owner[1]) != pid:
        return None
    window_type = re.search(r'^_NET_WM_WINDOW_TYPE\(ATOM\) = (.*)$', text, re.M)
    transient = re.search(r'^WM_TRANSIENT_FOR.*$', text, re.M)
    return dict(pid=pid, xid=xid, dialog=bool(window_type and
        '_NET_WM_WINDOW_TYPE_DIALOG' in window_type[1].split(', ')),
        transient_absent=bool(transient and ('not found' in transient[0] or
            'no such atom' in transient[0])), observed_monotonic_ns=time.monotonic_ns())


def query(args):
    try:
        reply = subprocess.run(args, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                               timeout=.5, check=False)
        if reply.returncode == 0 and len(reply.stdout) <= 65536:
            return reply.stdout.decode('utf-8', errors='replace')
    except (subprocess.TimeoutExpired, OSError):
        pass
    return ''


class Observation:
    def __init__(self, pid):
        self.pid = pid
        self.result = None
        self.stop = threading.Event()
        self.thread = threading.Thread(target=self.run, daemon=True)
        self.thread.start()

    def run(self):
        deadline = time.monotonic() + 60
        while not self.stop.is_set() and time.monotonic() < deadline:
            tree = query(['/usr/sbin/xwininfo', '-root', '-children'])
            ids = re.findall(r'^\s+(0x[0-9a-fA-F]+)\s', tree, re.M)[:128]
            for xid in ids:
                if self.stop.is_set() or time.monotonic() >= deadline:
                    return
                text = query(['/usr/sbin/xprop', '-id', xid, '_NET_WM_PID',
                              '_NET_WM_WINDOW_TYPE', 'WM_TRANSIENT_FOR'])
                found = parse_properties(text, self.pid, xid)
                if found is not None:
                    self.result = found
                    if found['dialog'] and found['transient_absent']:
                        print(f'Live dialog hints captured: PID {self.pid}, XID {xid}. Perform the requested action now.', flush=True)
                        return
            self.stop.wait(.1)

    def finish(self):
        self.stop.set()
        self.thread.join(timeout=2)
        return self.result if not self.thread.is_alive() else None
