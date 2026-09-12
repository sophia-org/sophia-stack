#!/usr/bin/env python3
"""Run canonical checks in a private namespace with no render nodes or display.

The canonical gate attempts hardware proofs whenever render nodes are writable.
Clearing opt-in environment variables does not prevent that. This wrapper gives
the unchanged gate a fresh /dev without /dev/dri, a committed source snapshot,
and a private Cargo home containing only the explicitly mounted registry cache.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

from isolation import ENVIRONMENT, IsolationError, Mount, launch, validate_entry

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
GIT_ENVIRONMENT = {'PATH': '/usr/bin:/bin', 'HOME': '/nonexistent',
                   'GIT_CONFIG_GLOBAL': '/dev/null', 'GIT_CONFIG_NOSYSTEM': '1',
                   'GIT_TERMINAL_PROMPT': '0', 'LANG': 'C.UTF-8'}


def git(source, *arguments):
    return subprocess.check_output(['git', '-C', str(source), *arguments],
                                   env=GIT_ENVIRONMENT, stderr=subprocess.PIPE,
                                   timeout=120).decode().strip()


def source_state(source):
    return {'commit': git(source, 'rev-parse', 'HEAD'),
            'tree': git(source, 'rev-parse', 'HEAD^{tree}'),
            'dirty': bool(git(source, 'status', '--porcelain=v1', '--untracked-files=all'))}


def archive_digest(source, commit):
    digest = hashlib.sha256()
    with subprocess.Popen(['git', '-C', str(source), 'archive', '--format=tar', commit],
                          env=GIT_ENVIRONMENT, stdout=subprocess.PIPE,
                          stderr=subprocess.DEVNULL) as process:
        while chunk := process.stdout.read(1024 * 1024):
            digest.update(chunk)
        if process.wait(timeout=120):
            raise RuntimeError('could not hash committed source archive')
    return digest.hexdigest()


def snapshot(source, destination):
    before = source_state(source)
    if before['dirty']:
        raise ValueError('source is dirty; commit changes before taking a canonical snapshot')
    destination.mkdir()
    git(destination, '-c', 'init.templateDir=', 'init', '--quiet')
    # Fetch an exact local object, not a branch name, configured remote or shared
    # worktree. The shallow repository owns its object data and index.
    git(destination, '-c', 'protocol.file.allow=always', 'fetch', '--quiet',
        '--no-tags', '--depth=1', str(source.resolve()), before['commit'])
    git(destination, '-c', 'advice.detachedHead=false', 'checkout', '--quiet',
        '--detach', before['commit'])
    if source_state(source) != before:
        raise ValueError('source HEAD or working state changed while taking the snapshot')
    if source_state(destination) != before:
        raise ValueError('snapshot does not match committed source')
    if (destination / '.git/objects/info/alternates').exists() or (destination / '.git/commondir').exists():
        raise ValueError('snapshot has an external Git object/worktree dependency')
    return {**before, 'source': str(source.resolve()),
            'archive_sha256': archive_digest(destination, before['commit'])}


def file_digest(path):
    digest = hashlib.sha256()
    with path.open('rb') as source:
        while chunk := source.read(1024 * 1024):
            digest.update(chunk)
    return digest.hexdigest()


def toolchain_path(source):
    environment = {'PATH': os.environ.get('PATH', '/usr/bin:/bin'),
                   'HOME': str(Path.home())}
    result = subprocess.check_output(['rustc', '--print', 'sysroot'], cwd=source,
                                     env=environment, timeout=30, stderr=subprocess.PIPE)
    return Path(result.decode().strip()).resolve(strict=True)


def required_tool(name):
    path = shutil.which(name)
    if path is None:
        raise ValueError(f'missing canonical check dependency: {name}')
    return Path(path).resolve(strict=True)


def check_paths(source, output, target):
    common = Path(git(source, 'rev-parse', '--path-format=absolute', '--git-common-dir'))
    artifacts = (common.parent / '.artifacts').resolve()
    output, target = output.resolve(), target.resolve()
    for name, path in (('output', output), ('target', target)):
        if (path == artifacts or not path.is_relative_to(artifacts)
                or path.is_relative_to('/tmp')):
            raise ValueError(f'{name} must be a disk-backed child of {artifacts}')
    if output.is_relative_to(target) or target.is_relative_to(output):
        raise ValueError('output and target paths must not overlap')
    if output.exists():
        raise ValueError('output must be new; retained evidence is immutable')
    return output, target


def private_hosts(root):
    # Renderer refusal tests use /etc/hosts solely as a regular, non-device
    # file. Supply private runtime data instead of exposing the host's /etc.
    directory = root / 'etc'
    directory.mkdir(exist_ok=True)
    (directory / 'hosts').write_text('127.0.0.1 localhost\n::1 localhost\n')


def private_target_link(source):
    # Existing canonical profile readers locate binaries relative to the
    # source tree, independently of Cargo's target-dir setting. Link only this
    # private copy to the explicitly mounted disk target, never an installation.
    (source / 'target').symlink_to('/work/target', target_is_directory=True)


def private_loader_cache():
    # Nested production protection domains bind this exact path. Generate it
    # from private, already-allowlisted libraries; never copy host /etc or let
    # ldconfig read host configuration or modify library symlinks.
    subprocess.run(['/usr/bin/ldconfig', '-X', '-i', '-f', '/dev/null',
                    '-C', '/etc/ld.so.cache', '/usr/lib'], env=ENVIRONMENT,
                   stdin=subprocess.DEVNULL, capture_output=True, text=True,
                   check=True, timeout=30)


def preflight_versions(environment):
    # audit_source_layout.sh currently swallows a missing rg in a conditional:
    # the retained 52d20c5d run falsely reported eleven inline-test debts retired.
    # Refuse missing/non-executable helpers before paying for the full suite.
    commands = {
        'rustc': ['/work/toolchain/bin/rustc', '-Vv'],
        'cargo': ['/work/toolchain/bin/cargo', '-Vv'],
        'rg': ['/work/tools/rg', '--version'],
        **{tool: [f'/usr/bin/{tool}', '--version'] for tool in
           ('bash', 'git', 'python3', 'cc', 'pkg-config', 'bwrap', 'ldconfig')},
    }
    return {name: subprocess.check_output(command, env=environment, text=True,
                                         stderr=subprocess.STDOUT, timeout=30).strip()
            for name, command in commands.items()}


def inside(activation_fd, validate_only):
    validate_entry(activation_fd)  # Before changing environment, directories or running tools.
    if Path('/dev/dri').exists():
        raise IsolationError('render-device namespace is not empty')
    private_hosts(Path('/'))
    private_target_link(Path('/work/source'))
    Path('/work/cargo/registry').symlink_to('/work/registry', target_is_directory=True)
    Path('/usr/include').symlink_to('/work/include', target_is_directory=True)
    environment = {**ENVIRONMENT, 'PATH': '/work/toolchain/bin:/work/tools:/usr/bin:/bin',
                   'CARGO': '/work/toolchain/bin/cargo',
                   'RUSTC': '/work/toolchain/bin/rustc',
                   'RUSTDOC': '/work/toolchain/bin/rustdoc',
                   'CARGO_HOME': '/work/cargo', 'CARGO_TARGET_DIR': '/work/target',
                   'CARGO_NET_OFFLINE': 'true', 'GIT_CONFIG_GLOBAL': '/dev/null',
                   'GIT_CONFIG_NOSYSTEM': '1', 'GIT_TERMINAL_PROMPT': '0',
                   'PWD': '/work/source'}
    os.chdir('/work/source')
    try:
        versions = preflight_versions(environment)
        private_loader_cache()
    except (OSError, subprocess.SubprocessError) as error:
        report = {'status': 'BLOCKED', 'full_check_executed': False,
                  'detail': f'private tool preflight failed: {type(error).__name__}: {error}'}
        Path('/work/evidence/inner-report.json').write_text(json.dumps(report, indent=2) + '\n')
        return 2
    Path('/work/evidence/preflight.json').write_text(json.dumps(versions, indent=2) + '\n')
    command = ['/work/toolchain/bin/cargo', 'metadata', '--offline', '--locked',
               '--format-version', '1'] if validate_only else ['/work/toolchain/bin/cargo', 'xtask', 'check']
    with Path('/work/evidence/command.log').open('wb') as log:
        result = subprocess.run(command, env=environment, stdin=subprocess.DEVNULL,
                                stdout=log, stderr=subprocess.STDOUT, check=False)
    report = {'status': 'PASS' if result.returncode == 0 else 'FAIL',
              'command': command, 'command_exit': result.returncode, 'tool_versions': versions,
              'full_check_executed': not validate_only,
              'scope': 'Contained offline checks; no hardware or physical-input acceptance.',
              'render_devices_present': False,
              'private_runtime_data': ['generated /etc/hosts: loopback names only',
                                       'generated /etc/ld.so.cache: private allowlisted libraries only',
                                       'private source/target symlink to owned /work/target'],
              'hardware_proofs': {'status': 'NOT_RUN', 'reason': 'private /dev has no render nodes'},
              'promoted_host_archives': {'status': 'NOT_RUN', 'reason': 'host state directories are not mounted'}}
    Path('/work/evidence/inner-report.json').write_text(json.dumps(report, indent=2) + '\n')
    return result.returncode


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, default=ROOT)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--target-dir', type=Path)
    parser.add_argument('--registry', type=Path, default=Path.home() / '.cargo/registry')
    parser.add_argument('--validate-only', action='store_true', help='versions and full offline metadata only; no check/build/test')
    parser.add_argument('--timeout', type=float, default=1800)
    parser.add_argument('--inside', action='store_true', help=argparse.SUPPRESS)
    parser.add_argument('--activation-fd', type=int, default=-1, help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.inside:
        return inside(args.activation_fd, args.validate_only)
    if not args.output or not args.target_dir:
        parser.error('--output and --target-dir are required')
    if not 0 < args.timeout <= 1800:
        parser.error('timeout must be in (0, 1800]')
    source = args.source.resolve(strict=True)
    if source_state(source)['dirty']:
        parser.error('source is dirty; no canonical snapshot was taken')
    output, target = check_paths(source, args.output, args.target_dir)
    toolchain = toolchain_path(source)
    ripgrep = required_tool('rg')
    tools = ('rustc', 'rustdoc', 'cargo', 'cargo-fmt', 'rustfmt', 'cargo-clippy', 'clippy-driver')
    tool_hashes = {name: file_digest(toolchain / 'bin' / name) for name in tools}
    registry = args.registry.resolve(strict=True)
    if not all((registry / part).is_dir() for part in ('cache', 'index', 'src')):
        parser.error('registry must contain offline cache, index and unpacked src directories')
    output.mkdir(parents=True)
    provenance = snapshot(source, output / 'source')
    for directory in ('evidence', 'cargo'):
        (output / directory).mkdir()
    target.mkdir(parents=True, exist_ok=True)
    mounts = [Mount(HERE, '/work/harness/tools/probes/x11_conformance'),
              Mount(output / 'source', '/work/source', writable=True),
              Mount(output / 'cargo', '/work/cargo', writable=True),
              Mount(output / 'evidence', '/work/evidence', writable=True),
              Mount(target, '/work/target', writable=True), Mount(toolchain, '/work/toolchain'),
              Mount(ripgrep, '/work/tools/rg'),
              Mount(registry, '/work/registry'), Mount(Path('/usr/include'), '/work/include')]
    command = ['/usr/bin/python3', '-B',
               '/work/harness/tools/probes/x11_conformance/offline_check.py', '--inside',
               '--activation-fd', '{activation_fd}']
    if args.validate_only:
        command.append('--validate-only')
    provenance.update(toolchain=str(toolchain), tool_sha256=tool_hashes,
                      auxiliary_tools={'rg': {'source': str(ripgrep), 'sha256': file_digest(ripgrep)}},
                      wrapper_sha256=file_digest(Path(__file__)),
                      isolation_sha256=file_digest(HERE / 'isolation.py'),
                      target=str(target), registry=str(registry), validate_only=args.validate_only)
    (output / 'provenance.json').write_text(json.dumps(provenance, indent=2) + '\n')
    try:
        result = launch(command, mounts=mounts, timeout=args.timeout)
        (output / 'adapter.log').write_bytes(result.stdout + result.stderr)
        inner = output / 'evidence/inner-report.json'
        report = json.loads(inner.read_text()) if inner.exists() else {
            'status': 'NORESULT', 'full_check_executed': False if args.validate_only else None,
            'detail': 'contained command produced no completion report; see adapter.log'}
        expected_block = (result.returncode == 2 and report.get('status') == 'BLOCKED'
                          and report.get('full_check_executed') is False)
        if result.returncode and not expected_block:
            report['status'] = 'TIMEOUT' if result.returncode == 124 else 'FAIL'
            report['adapter_exit'] = result.returncode
    except IsolationError as error:
        report = {'status': 'BLOCKED', 'full_check_executed': False, 'detail': str(error)}
    report['provenance'] = provenance
    report['hardware_proofs'] = {'status': 'NOT_RUN', 'reason': 'render nodes are never mounted'}
    (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))
    return 0 if report['status'] == 'PASS' else 2 if report['status'] == 'BLOCKED' else 1


if __name__ == '__main__':
    raise SystemExit(main())
