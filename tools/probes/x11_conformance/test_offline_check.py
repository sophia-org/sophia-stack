"""Offline wrapper regressions; these do not execute cargo xtask check."""
from pathlib import Path
import subprocess
import json
import os
import sys
import tempfile
import unittest
from unittest.mock import patch

import offline_check as gate


class OfflineCheckTests(unittest.TestCase):
    def repository(self, base):
        source = base / 'original'
        source.mkdir()
        gate.git(source, '-c', 'init.templateDir=', 'init', '--quiet')
        (source / 'tracked.txt').write_text('the committed fixture\n')
        gate.git(source, 'add', 'tracked.txt')
        gate.git(source, '-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                 '-c', 'commit.gpgSign=false', 'commit', '--quiet', '-m', 'fixture')
        gate.git(source, '-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                 '-c', 'commit.gpgSign=false', 'commit', '--quiet', '--allow-empty', '-m', 'child fixture')
        return source

    def test_snapshot_preserves_exact_commit_without_worktree_links(self):
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            source = self.repository(base)
            destination = base / 'copy'
            report = gate.snapshot(source, destination)
            self.assertEqual(report['commit'], gate.git(source, 'rev-parse', 'HEAD'))
            self.assertEqual(report['tree'], gate.git(destination, 'rev-parse', 'HEAD^{tree}'))
            self.assertEqual(gate.git(destination, 'rev-parse', 'HEAD^'),
                             gate.git(source, 'rev-parse', 'HEAD^'))
            self.assertFalse(report['dirty'])
            self.assertEqual(len(report['archive_sha256']), 64)
            self.assertFalse((destination / '.git/commondir').exists())
            self.assertFalse((destination / '.git/objects/info/alternates').exists())
            self.assertEqual(gate.git(destination, 'remote'), '')
            (source / 'tracked.txt').write_text('modified host checkout\n')
            self.assertEqual((destination / 'tracked.txt').read_text(), 'the committed fixture\n')
            self.assertFalse(gate.source_state(destination)['dirty'])

    def test_dirty_tracked_and_untracked_source_refused_before_copy(self):
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            source = self.repository(base)
            for name in ('tracked.txt', 'untracked.txt'):
                with self.subTest(name=name):
                    (source / name).write_text('not committed\n')
                    destination = base / ('copy-' + name)
                    with self.assertRaisesRegex(ValueError, 'dirty'):
                        gate.snapshot(source, destination)
                    self.assertFalse(destination.exists())
                    if name == 'tracked.txt':
                        gate.git(source, 'checkout', '--', name)
                    else:
                        (source / name).unlink()

    def test_changed_head_during_snapshot_is_not_relabelled(self):
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            source = self.repository(base)
            real_state = gate.source_state
            reads = 0
            def changed(path):
                nonlocal reads
                result = real_state(path)
                if path == source:
                    reads += 1
                    if reads > 1:
                        result['commit'] = '0' * 40
                return result
            with patch.object(gate, 'source_state', side_effect=changed):
                with self.assertRaisesRegex(ValueError, 'changed while'):
                    gate.snapshot(source, base / 'copy')

    def test_direct_inside_refused_before_tool_or_device_inspection(self):
        with patch.object(gate, 'validate_entry', side_effect=gate.IsolationError('no activation')):
            with patch.object(gate.subprocess, 'check_output') as command:
                with self.assertRaisesRegex(gate.IsolationError, 'no activation'):
                    gate.inside(-1, True)
                command.assert_not_called()
        result = subprocess.run([sys.executable, '-B', str(Path(gate.__file__)),
                                 '--inside', '--validate-only'],
                                capture_output=True, timeout=5)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b'activation must be a runner pipe', result.stderr)
        self.assertNotIn(b'cargo metadata', result.stderr)

    def test_inside_requires_no_render_nodes_even_after_activation(self):
        with patch.object(gate, 'validate_entry'), patch.object(gate.Path, 'exists', return_value=True):
            with self.assertRaisesRegex(gate.IsolationError, 'render-device'):
                gate.inside(7, True)

    def test_snapshot_git_cannot_use_ambient_configuration(self):
        self.assertEqual(gate.GIT_ENVIRONMENT['GIT_CONFIG_GLOBAL'], '/dev/null')
        self.assertEqual(gate.GIT_ENVIRONMENT['GIT_CONFIG_NOSYSTEM'], '1')
        self.assertEqual(gate.GIT_ENVIRONMENT['GIT_TERMINAL_PROMPT'], '0')
        for name in ('DISPLAY', 'WAYLAND_DISPLAY', 'DBUS_SESSION_BUS_ADDRESS',
                     'SOPHIA_RUN_REAL_GBM_SMOKE', 'HAGIA_SOCKET', 'SSH_AUTH_SOCK'):
            self.assertNotIn(name, gate.GIT_ENVIRONMENT)

    def test_temporary_filesystem_targets_refused(self):
        with patch.object(gate, 'git', return_value='/tmp/fabricated/.git'):
            with self.assertRaisesRegex(ValueError, 'disk-backed'):
                gate.check_paths(Path('/tmp/fabricated'), Path('/tmp/fabricated/.artifacts/output'),
                                 Path('/tmp/fabricated/.artifacts/target'))

    def test_hosts_fixture_is_private_regular_data_without_host_copy(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            gate.private_hosts(root)
            hosts = root / 'etc/hosts'
            self.assertTrue(hosts.is_file())
            self.assertFalse(hosts.is_symlink())
            self.assertEqual(hosts.read_text(), '127.0.0.1 localhost\n::1 localhost\n')
            self.assertEqual(list((root / 'etc').iterdir()), [hosts])

    def test_profile_target_link_names_only_the_owned_disk_target(self):
        with tempfile.TemporaryDirectory() as temporary:
            source = Path(temporary)
            gate.private_target_link(source)
            target = source / 'target'
            self.assertTrue(target.is_symlink())
            self.assertEqual(target.readlink(), Path('/work/target'))
            with self.assertRaises(FileExistsError):
                gate.private_target_link(source)

    def test_loader_cache_uses_private_libraries_and_no_host_configuration(self):
        with patch.object(gate.subprocess, 'run') as command:
            gate.private_loader_cache()
        command.assert_called_once_with(
            ['/usr/bin/ldconfig', '-X', '-i', '-f', '/dev/null',
             '-C', '/etc/ld.so.cache', '/usr/lib'], env=gate.ENVIRONMENT,
            stdin=subprocess.DEVNULL, capture_output=True, text=True,
            check=True, timeout=30)

    def test_missing_required_search_tool_is_a_blocker_not_retired_source_debt(self):
        with patch.object(gate.shutil, 'which', return_value=None):
            with self.assertRaisesRegex(ValueError, 'missing canonical check dependency: rg'):
                gate.required_tool('rg')
        with tempfile.TemporaryDirectory() as temporary:
            executable = Path(temporary) / 'rg'
            executable.write_text('fixture executable path only\n')
            alias = Path(temporary) / 'alias'
            alias.symlink_to(executable)
            with patch.object(gate.shutil, 'which', return_value=str(alias)):
                self.assertEqual(gate.required_tool('rg'), executable)

    def test_preflight_executes_helpers_with_version_only_and_bounded_waits(self):
        with patch.object(gate.subprocess, 'check_output', return_value='fixture version\n') as command:
            report = gate.preflight_versions(gate.ENVIRONMENT)
        self.assertEqual(set(report), {'rustc', 'cargo', 'rg', 'bash', 'git', 'python3',
                                       'cc', 'pkg-config', 'bwrap', 'ldconfig'})
        self.assertEqual(command.call_count, 10)
        for call in command.call_args_list:
            self.assertIn(call.args[0][-1], ('--version', '-Vv'))
            self.assertEqual(call.kwargs['env'], gate.ENVIRONMENT)
            self.assertEqual(call.kwargs['timeout'], 30)
        with patch.object(gate.subprocess, 'check_output', side_effect=FileNotFoundError('missing rg')):
            with self.assertRaises(FileNotFoundError):
                gate.preflight_versions(gate.ENVIRONMENT)


    def test_verification_input_rejects_links_special_files_and_size(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            public = root / 'public'; public.write_bytes(b'certificate bytes')
            self.assertEqual(gate.verification_input(public), b'certificate bytes')
            link = root / 'alias'; link.symlink_to(public)
            fifo = root / 'pipe'; os.mkfifo(fifo)
            empty = root / 'empty'; empty.touch()
            large = root / 'large'; large.write_bytes(b'x' * (gate.MAX_PUBLIC_KEY_BYTES + 1))
            for invalid in (link, fifo, root, empty, large):
                with self.subTest(path=invalid.name):
                    with self.assertRaises((OSError, gate.VerificationError)):
                        gate.verification_input(invalid)

    def test_metadata_without_key_has_no_signature_claim(self):
        with patch.object(gate, 'verify_public_input') as verify:
            environment, report = gate.prepare_verification(True, None, gate.ENVIRONMENT,
                                                            Path('/unused'), Path('/unused'))
        self.assertEqual(environment, gate.ENVIRONMENT)
        self.assertEqual(report['status'], 'NOT_RUN')
        verify.assert_not_called()

    def check_blocked_inside(self, *, has_key, error=None, verification=None, expected_hash=None):
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            for name in ('work/source', 'work/cargo', 'work/evidence', 'usr'):
                (base / name).mkdir(parents=True, exist_ok=True)
            def private_path(value):
                return base / str(value).lstrip('/')
            with patch.object(gate, 'Path', side_effect=private_path), \
                    patch.object(gate, 'validate_entry'), patch.object(gate, 'private_hosts'), \
                    patch.object(gate, 'private_target_link'), patch.object(gate.os, 'chdir'), \
                    patch.object(gate, 'verify_public_input', side_effect=error,
                                 return_value=(gate.ENVIRONMENT, verification)) as verify, \
                    patch.object(gate, 'preflight_versions') as versions, \
                    patch.object(gate.subprocess, 'run') as run:
                result = gate.inside(5, False, Path('/supplied-public') if has_key else None, expected_hash)
                report = json.loads((base / 'work/evidence/inner-report.json').read_text())
                self.assertEqual(result, 2)
                self.assertEqual(report['status'], 'BLOCKED')
                self.assertFalse(report['full_check_executed'])
                versions.assert_not_called(); run.assert_not_called()
                self.assertEqual(verify.call_count, int(has_key))
                return report

    def test_missing_public_key_blocks_before_canonical_command(self):
        self.check_blocked_inside(has_key=False)

    def test_signature_failure_blocks_before_canonical_command(self):
        self.check_blocked_inside(has_key=True, error=gate.VerificationError('invalid signature'))

    def test_changed_public_input_identity_blocks_before_canonical_command(self):
        report = self.check_blocked_inside(has_key=True, expected_hash='0' * 64,
                                          verification={'status': 'PASS', 'public_sha256': '1' * 64})
        self.assertEqual(report['signature_verification']['status'], 'FAIL')

    def test_secret_key_packets_are_refused_before_import(self):
        for packet in (b':secret key packet:', b':secret sub key packet:'):
            with self.subTest(packet=packet), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary); public = root / 'mistaken-input'; public.write_bytes(b'fixture')
                with patch.object(gate, 'verification_command', return_value=packet) as command:
                    with self.assertRaisesRegex(gate.VerificationError, 'secret-key packets'):
                        gate.verify_public_input(public, gate.ENVIRONMENT, root / 'private-home', root)
                self.assertEqual(command.call_count, 1)
                self.assertIn('--list-packets', command.call_args.args[0])
                self.assertNotIn('--import', command.call_args.args[0])
                self.assertEqual(list((root / 'private-home').iterdir()), [])

    def test_public_import_and_signature_commands_use_only_private_home(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); public = root / 'public'; public.write_bytes(b'fixture-public')
            home = root / 'private-home'; fingerprint = 'A' * 40
            listing = ('fpr:::::::::' + fingerprint + ':\n').encode()
            responses = [b':public key packet:', b'gpg (GnuPG) fixture\n', b'imported', listing,
                         b'1' * 40 + b'\n', b'good signature', b'2' * 40 + b'\n', b'good signature']
            with patch.object(gate, 'verification_command', side_effect=responses) as command:
                environment, report = gate.verify_public_input(public, gate.ENVIRONMENT, home, root)
            self.assertEqual(report['status'], 'PASS')
            self.assertEqual(report['public_fingerprints'], [fingerprint])
            self.assertEqual(set(report['commits']), {'HEAD', 'HEAD^'})
            self.assertEqual(environment['GNUPGHOME'], str(home))
            self.assertEqual(home.stat().st_mode & 0o777, 0o700)
            self.assertNotIn('SSH_AUTH_SOCK', environment)
            self.assertNotIn('DBUS_SESSION_BUS_ADDRESS', environment)
            calls = command.call_args_list
            self.assertIn('--list-packets', calls[0].args[0])
            self.assertIn('--version', calls[1].args[0])
            self.assertIn('--import', calls[2].args[0])
            self.assertEqual(report['gpg_version'], 'gpg (GnuPG) fixture')
            for call in calls:
                self.assertEqual(call.args[1]['GNUPGHOME'], str(home))
                if call.args[0][0].endswith('/gpg'):
                    for flag in ('--no-options', '--no-autostart', '--no-auto-key-retrieve'):
                        self.assertIn(flag, call.args[0])
            self.assertIn('--no-autostart', (home / 'verify-gpg').read_text())

    def test_verification_process_deadline_and_output_bound(self):
        with self.assertRaises(gate.VerificationError):
            gate.verification_command([sys.executable, '-c', 'import time; time.sleep(5)'],
                                      gate.ENVIRONMENT, timeout=0.05)
        with patch.object(gate, 'MAX_VERIFICATION_OUTPUT', 32):
            with self.assertRaises(gate.VerificationError):
                gate.verification_command([sys.executable, '-c', 'print("x" * 65536)'],
                                          gate.ENVIRONMENT, timeout=2)


if __name__ == '__main__':
    unittest.main()
