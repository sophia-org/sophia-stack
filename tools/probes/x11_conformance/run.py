#!/usr/bin/env python3
"""Launch only our own private socket host; there is deliberately no DISPLAY option."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import traceback

from cases import CASES
from report import evaluate, load_manifest
from inventory import check_inventory

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def clean_environment():
    return {k: v for k, v in os.environ.items()
            if not k.startswith(('SOPHIA_', 'HAGIA_'))
            and k not in ('DISPLAY', 'XAUTHORITY', 'WAYLAND_DISPLAY', 'WAYLAND_SOCKET',
                          'PYTHONOPTIMIZE')}


def decode_result(status, stdout, stderr):
    if status == 124:
        return {'status': 'TIMEOUT', 'detail': 'client process deadline'}
    try:
        result = json.loads(stdout)
        assert result['status'] in ('PASS', 'FAIL', 'TIMEOUT')
    except (ValueError, KeyError, TypeError, AssertionError):
        return {'status': 'NORESULT', 'detail': f'exit={status}, stderr={stderr[:2000]!r}'}
    if status != 0 and result['status'] == 'PASS':
        return {'status': 'FAIL', 'detail': 'PASS with nonzero client exit'}
    return result


def bounded(command, timeout, **kwargs):
    """Kill only our new process group, including descendants, on an absolute timeout."""
    with subprocess.Popen(command, start_new_session=True, **kwargs) as child:
        try:
            stdout, stderr = child.communicate(timeout=timeout)
            return child.returncode, stdout, stderr
        except subprocess.TimeoutExpired:
            os.killpg(child.pid, signal.SIGKILL)
            stdout, stderr = child.communicate()
            return 124, stdout, stderr


def one_case(host, manifest, case, order, timeout, log):
    # Private directory has mode 0700, no global /tmp/.X11-unix listener.
    with tempfile.TemporaryDirectory(prefix='sophia-x11-') as tmp:
        sock = Path(tmp) / 'authority.sock'
        with log.open('wb') as output:
            server = subprocess.Popen([str(host), str(sock)], env=clean_environment(),
                                      stdout=output, stderr=output, start_new_session=True)
            try:
                ready_deadline = time.monotonic() + 5
                while not sock.exists():
                    if server.poll() is not None:
                        return {'status': 'FAIL', 'detail': f'host exited {server.returncode} before bind'}
                    if time.monotonic() >= ready_deadline:
                        return {'status': 'TIMEOUT', 'detail': 'host bind deadline'}
                    time.sleep(0.01)
                # A separate client process enforces even non-socket hangs.
                command = [sys.executable, '-B', str(HERE / 'run.py'), '--child', str(sock),
                           '--case', case, '--order', order, '--timeout', str(timeout)]
                status, stdout, stderr = bounded(command, timeout + 1, env=clean_environment(),
                                                 stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                result = decode_result(status, stdout, stderr)
                if server.poll() is not None:
                    return {'status': 'FAIL', 'detail': f'host exited unexpectedly {server.returncode}'}
                return result
            finally:
                if server.poll() is None:
                    os.killpg(server.pid, signal.SIGTERM)
                try:
                    server.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    os.killpg(server.pid, signal.SIGKILL)
                    server.wait()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    if not __debug__:
        raise SystemExit('conformance assertions require Python without -O')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--host', type=Path)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--timeout', type=float, default=3)
    parser.add_argument('--child', type=Path, help=argparse.SUPPRESS)
    parser.add_argument('--case', help=argparse.SUPPRESS)
    parser.add_argument('--order', choices=['little', 'big'], help=argparse.SUPPRESS)
    args = parser.parse_args()
    if not 0 < args.timeout <= 60:
        parser.error('timeout must be in (0, 60] seconds')
    manifest = load_manifest(HERE / 'manifest.json')
    assert set(CASES) == {case['id'] for case in manifest['cases']}, 'implementation/manifest drift'
    if args.child:
        try:
            CASES[args.case]({'socket': args.child, 'order': '<' if args.order == 'little' else '>',
                             'deadline': time.monotonic() + args.timeout, 'case': args.case,
                             'extensions': manifest['extensions'],
                             'fixture_absence': manifest['fixture_absence'],
                             'denied_extensions': manifest['intentional_absence']})
            result = {'status': 'PASS'}
        except TimeoutError as error:
            result = {'status': 'TIMEOUT', 'detail': str(error)}
        except Exception as error:
            result = {'status': 'FAIL', 'detail': f'{type(error).__name__}: {error}',
                      'traceback': traceback.format_exc(limit=5)}
        print(json.dumps(result))
        return 0 if result['status'] == 'PASS' else 1
    if not args.host or not args.output:
        parser.error('--host and --output are required; --output must be new')
    host = args.host.resolve(strict=True)
    inventory = check_inventory(ROOT, manifest)
    args.output.mkdir(parents=True, exist_ok=False)
    results = []
    for case in manifest['cases']:
        for order in manifest['byte_orders']:
            try:
                result = one_case(host, manifest, case['id'], order, args.timeout,
                                  args.output / f'{case["id"]}-{order}.host.log')
            except Exception as error:
                result = {'status': 'FAIL', 'detail': f'harness/host failure: {type(error).__name__}: {error}'}
            result.update(case=case['id'], byte_order=order)
            results.append(result)
            print(f'{case["id"]}/{order}: {result["status"]}', flush=True)
    report = evaluate(manifest, results)
    report['identity'] = {'host': str(host), 'host_sha256': digest(host),
                          'source_commit': subprocess.check_output(
                              ['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
                          'source_dirty': bool(subprocess.check_output(
                              ['git', 'status', '--porcelain'], cwd=ROOT, text=True)),
                          'harness_sha256': {p.name: digest(p) for p in HERE.iterdir()
                                             if p.is_file() and p.suffix in ('.py', '.json')}}
    report['scope'] = manifest['scope']
    report['request_inventory'] = inventory
    report['xts5'] = {'status': 'NOT_RUN', 'reason': 'separate explicit adapter invocation required'}
    (args.output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k: report[k] for k in ('status', 'required', 'executed', 'failures')}, indent=2))
    return 0 if report['status'] == 'PASS' else 1


if __name__ == '__main__':
    sys.exit(main())
