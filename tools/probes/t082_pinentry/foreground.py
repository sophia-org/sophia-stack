#!/usr/bin/env python3
"""Run acceptance in the existing desktop after identifying its X server."""
import argparse
import json
import os
from pathlib import Path
import re
import tempfile
from production import digest, stage_candidate


def listener_inode(proc_root, socket_path):
    matches = []
    for line in (proc_root / 'net/unix').read_text().splitlines()[1:]:
        fields = line.split(maxsplit=7)
        if len(fields) != 8 or fields[7] != socket_path:
            continue
        # SOCK_STREAM, listening (SO_ACCEPTCON), unconnected listener state.
        if int(fields[3], 16) & 0x10000 and fields[4] == '0001' and fields[5] == '01':
            matches.append(fields[6])
    if len(matches) != 1 or not matches[0].isdigit():
        raise RuntimeError('Missing or ambiguous local DISPLAY listener')
    return matches[0]


def start_ticks(process):
    fields = (process / 'stat').read_text().rsplit(')', 1)[1].split()
    value = fields[19]
    if not value.isdigit():
        raise RuntimeError('Invalid process starttime')
    return value


def owns_socket(process, inode):
    for fd in (process / 'fd').iterdir():
        try:
            if os.readlink(fd) == f'socket:[{inode}]':
                return True
        except FileNotFoundError:
            continue  # Unrelated descriptor closed during inspection.
    return False


def listener_owners(proc_root, inode, uid):
    owners = []
    for process in proc_root.iterdir():
        if not process.name.isdigit():
            continue
        try:
            if process.stat().st_uid != uid:
                continue
            before = start_ticks(process)
            if owns_socket(process, inode):
                if start_ticks(process) != before:
                    raise RuntimeError('Listener process changed during inspection')
                owners.append((process, before))
        except FileNotFoundError:
            continue  # Exited processes cannot remain the final verified owner.
        # Permission errors are not evidence of absence: fail closed.
    return owners


def server_identity(release, proc_root=Path('/proc')):
    display = os.environ.get('DISPLAY', '')
    match = re.fullmatch(r':([0-9]+)(?:\.0)?', display)
    if match is None:
        raise RuntimeError('A local DISPLAY is required; run in the existing desktop terminal')
    socket_path = '/tmp/.X11-unix/X' + match[1]
    uid = os.getuid()
    try:
        inode = listener_inode(proc_root, socket_path)
        owners = listener_owners(proc_root, inode, uid)
        if len(owners) != 1:
            raise RuntimeError('Missing or ambiguous owned DISPLAY listener process')
        process, before = owners[0]
        executable = process / 'exe'
        installed = (release / 'target/release/sophia').resolve(strict=True)
        if executable.resolve(strict=True) != installed:
            raise RuntimeError('Running listener is not the installed Sophia executable')
        actual = digest(executable)
        if actual != digest(installed):
            raise RuntimeError('Running and installed Sophia hashes differ')
        # Recheck after hashing; neither PID reuse, exec, nor listener replacement
        # can turn the earlier observation into a current identity claim.
        if (start_ticks(process) != before or executable.resolve(strict=True) != installed
                or process.stat().st_uid != uid
                or listener_inode(proc_root, socket_path) != inode
                or listener_owners(proc_root, inode, uid) != owners):
            raise RuntimeError('Listener identity changed during inspection')
    except (OSError, ValueError, IndexError) as error:
        raise RuntimeError('Cannot passively verify DISPLAY identity') from error
    return dict(method='passive-proc-listener', pid=int(process.name), uid=uid,
                display=display, socket_path=socket_path, listener_inode=inode,
                executable=str(installed), sha256=actual, process_start_ticks=before)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate-manifest', type=Path, required=True)
    parser.add_argument('--preflight-only', action='store_true')
    args = parser.parse_args()
    if os.geteuid() == 0:
        raise RuntimeError('Run as the desktop user without sudo')
    release = Path('/opt/sophia/current').resolve(strict=True)
    manifest = dict(line.split('=', 1) for line in (release/'manifest').read_text().splitlines())
    if manifest.get('commit') != '702efef161ddf758affdbf68bf139b00aa21a5be':
        raise RuntimeError('Installed release changed; review and retarget before acceptance')
    running = server_identity(release)
    root = Path('/tmp/sophia-pinentry-trace')
    root.mkdir(mode=0o700, exist_ok=True)
    capture = Path(tempfile.mkdtemp(prefix='production-desktop.', dir=root))
    print(f'Capture directory: {capture}\nExisting desktop only; no sudo or restart.', flush=True)
    candidate, identity = stage_candidate(args.candidate_manifest, capture/'source')
    identity['binary_path'] = str(candidate)
    (capture/'candidate.json').write_text(json.dumps(identity)+'\n')
    (capture/'release.manifest').write_bytes((release/'manifest').read_bytes())
    (capture/'running-server.json').write_text(json.dumps(running, indent=2)+'\n')
    if args.preflight_only:
        print('Identity preflight PASS; no candidate launched.')
        return
    for key in list(os.environ):
        if key.startswith(('T082_', 'SOPHIA_', 'HAGIA_')) or key in ('EGUI_INSPECTION', 'WAYLAND_DISPLAY', 'WAYLAND_SOCKET', 'RUST_LOG'):
            del os.environ[key]
    os.execvp('python3', ['python3', '-B', str(Path(__file__).with_name('production.py')),
        '--candidate-manifest', str(capture/'candidate.json'), '--capture', str(capture/'production'),
        '--release-manifest', str(capture/'release.manifest')])

if __name__ == '__main__':
    main()
