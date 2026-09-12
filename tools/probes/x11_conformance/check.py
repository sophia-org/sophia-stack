#!/usr/bin/env python3
"""Build and run the isolated X11 gate, without inherited live-session opt-ins."""
import argparse
from pathlib import Path
import subprocess
import sys

from run import HERE, ROOT, clean_environment


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True, help='new evidence directory')
    parser.add_argument('--target-dir', type=Path, default=ROOT / '.artifacts/x11-conformance-target')
    args = parser.parse_args()
    if args.output.exists():
        parser.error('--output must not exist')
    env = clean_environment()
    env['CARGO_TARGET_DIR'] = str(args.target_dir.resolve())
    for command in ([sys.executable, '-B', '-m', 'unittest', 'discover', '-s', str(HERE), '-p', 'test_gate.py'],
                    ['cargo', 'build', '--offline', '-p', 'sophia-x-authority', '--example', 'x11_conformance_host']):
        result = subprocess.run(command, cwd=ROOT, env=env, timeout=600)
        if result.returncode:
            return result.returncode
    host = args.target_dir.resolve() / 'debug/examples/x11_conformance_host'
    return subprocess.call([sys.executable, '-B', str(HERE / 'run.py'), '--host', str(host),
                            '--output', str(args.output.resolve())], cwd=ROOT, env=env)


if __name__ == '__main__':
    raise SystemExit(main())
