"""Private filesystem fixtures only: no live /proc, display or socket queries."""
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import foreground


class PassiveIdentityTests(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory()
        self.addCleanup(self.scratch.cleanup)
        self.root = Path(self.scratch.name)
        self.proc = self.root / 'proc'
        (self.proc / 'net').mkdir(parents=True)
        self.release = self.root / 'release'
        (self.release / 'target/release').mkdir(parents=True)
        self.binary = self.release / 'target/release/sophia'
        self.binary.write_bytes(b'fixture executable, never launched')
        self.row = '000: 00000002 00000000 00010000 0001 01 456 /tmp/.X11-unix/X918\n'
        self.table(self.row)
        self.process(123)
        env = patch.dict(os.environ, {'DISPLAY': ':918.0'})
        env.start(); self.addCleanup(env.stop)
        socket = patch('socket.socket', side_effect=AssertionError('Socket construction forbidden'))
        self.socket = socket.start(); self.addCleanup(socket.stop)

    def table(self, rows):
        (self.proc / 'net/unix').write_text('Num RefCount Protocol Flags Type St Inode Path\n' + rows)

    def process(self, pid, ticks='42'):
        path = self.proc / str(pid)
        (path / 'fd').mkdir(parents=True)
        (path / 'fd/3').symlink_to('socket:[456]')
        (path / 'exe').symlink_to(self.binary)
        (path / 'stat').write_text(f'{pid} (name with ) spaces) ' + ' '.join(['S'] + ['0'] * 18 + [ticks]))
        return path

    def identify(self):
        return foreground.server_identity(self.release, self.proc)

    def test_exact_listener_executable_and_no_socket_constructor(self):
        self.table(self.row + self.row.replace('X918', 'X9180').replace('456', '999'))
        record = self.identify()
        self.assertEqual(record['pid'], 123)
        self.assertEqual(record['listener_inode'], '456')
        self.assertEqual(record['process_start_ticks'], '42')
        self.assertEqual(record['method'], 'passive-proc-listener')
        self.socket.assert_not_called()

    def test_missing_ambiguous_and_non_listener_socket_fail(self):
        for rows in ('', self.row * 2, self.row.replace('00010000', '00000000'),
                     self.row.replace('0001 01', '0002 01'), self.row.replace('X918', 'X919')):
            self.table(rows)
            with self.assertRaises(RuntimeError): self.identify()

    def test_multiple_owners_or_no_owner_fail(self):
        other = self.process(124)
        with self.assertRaises(RuntimeError): self.identify()
        (other / 'fd/3').unlink()
        (self.proc / '123/fd/3').unlink()
        with self.assertRaises(RuntimeError): self.identify()

    def test_wrong_executable_or_hash_fails(self):
        with patch('foreground.digest', side_effect=['running', 'installed']):
            with self.assertRaises(RuntimeError): self.identify()
        exe = self.proc / '123/exe'
        exe.unlink(); exe.symlink_to('/not-the-installed-binary')
        with self.assertRaises(RuntimeError): self.identify()

    def test_process_reuse_during_hashing_fails(self):
        original = foreground.digest
        def changed(path):
            result = original(path)
            (self.proc / '123/stat').write_text('123 (replacement) ' + ' '.join(['S'] + ['0'] * 18 + ['43']))
            return result
        with patch('foreground.digest', side_effect=changed):
            with self.assertRaises(RuntimeError): self.identify()

    def test_listener_replacement_during_hashing_fails(self):
        original = foreground.digest
        def changed(path):
            result = original(path)
            self.table(self.row.replace('456', '789'))
            return result
        with patch('foreground.digest', side_effect=changed):
            with self.assertRaises(RuntimeError): self.identify()

    def test_unowned_and_inaccessible_processes_fail_closed(self):
        with patch('foreground.os.getuid', return_value=os.getuid() + 1):
            with self.assertRaises(RuntimeError): self.identify()
        with patch('foreground.owns_socket', side_effect=PermissionError('fixture')):
            with self.assertRaises(RuntimeError): self.identify()

    def test_remote_display_refused(self):
        with patch.dict(os.environ, {'DISPLAY': 'remote:0'}):
            with self.assertRaises(RuntimeError): self.identify()


if __name__ == '__main__':
    unittest.main()
