import os
from pathlib import Path
import shlex
import socket
import subprocess
import tempfile
import threading
import time
import unittest


ROOT = Path(__file__).resolve().parents[2]
VALID = ["--list-only", "--geometry", "0,0,16,16", "--format", "XR24"]


class Dri3LayoutProbeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.directory = tempfile.TemporaryDirectory(prefix="sophia-layout-cli-")
        cls.binary = Path(cls.directory.name) / "probe"
        flags = subprocess.check_output([
            "pkg-config", "--cflags", "--libs", "xcb", "xcb-dri3", "xcb-present", "gbm"
        ], text=True, timeout=10)
        subprocess.run([
            "cc", "-std=c11", "-O2", "-Wall", "-Wextra", "-Werror",
            str(ROOT / "tools/probes/dri3_layout.c"), "-o", str(cls.binary),
            *shlex.split(flags),
        ], check=True, capture_output=True, timeout=30)

    @classmethod
    def tearDownClass(cls):
        cls.directory.cleanup()

    def run_probe(self, arguments, **environment):
        env = os.environ.copy()
        for name in ("DISPLAY", "WAYLAND_DISPLAY", "LD_PRELOAD"):
            env.pop(name, None)
        env["XAUTHORITY"] = "/dev/null"
        env.update(environment)
        return subprocess.run(
            [str(self.binary), *arguments], env=env, capture_output=True,
            text=True, timeout=3, check=False,
        )

    def test_help_and_invalid_arguments_do_not_connect(self):
        help_result = self.run_probe(["--help"])
        self.assertEqual(help_result.returncode, 0, help_result.stderr)
        self.assertIn("--list-only", help_result.stdout)
        invalid = [
            [], ["--list-only"],
            ["--geometry", "0,0,16,16", "--format", "XR24"],
            [*VALID, "--modifier", "0x00ffffffffffffff"],
            [*VALID, "--modifier", "0xffffffffffffffff"],
            [*VALID, "--frames", "0"], [*VALID, "--frames", "121"],
            [*VALID, "--timeout-ms", "0"], [*VALID, "--timeout-ms", "10001"],
            [*VALID, "--unknown", "1"], [*VALID, "--modifier"],
        ]
        for geometry in (
            "0,0,0,16", "0,0,4097,16", "32768,0,16,16", "0,-32769,16,16",
            "0,0,16,16extra", "999999999999999999999999,0,16,16",
        ):
            invalid.append(["--list-only", "--geometry", geometry, "--format", "XR24"])
        for arguments in invalid:
            with self.subTest(arguments=arguments):
                result = self.run_probe(arguments)
                self.assertEqual(result.returncode, 2, result.stderr)
                self.assertIn("usage:", result.stderr)
                self.assertNotIn("stage=authenticated_connect", result.stderr)
                self.assertNotIn("event=finished", result.stdout)

    def test_valid_request_requires_explicit_display(self):
        result = self.run_probe(VALID)
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertIn("stage=display_unset", result.stderr)
        self.assertNotIn("event=finished", result.stdout)

    def test_setup_wait_is_inside_the_process_deadline(self):
        # This endpoint accepts only this child and never answers X setup.
        # Empty Xauthority prevents reading or forwarding any session cookie.
        with socket.socket() as listener:
            listener.bind(("127.0.0.1", 0))
            listener.listen(1)
            listener.settimeout(2)
            port = listener.getsockname()[1]
            self.assertGreaterEqual(port, 6000)
            accepted = threading.Event()
            release = threading.Event()
            failures = []

            def serve():
                try:
                    with listener.accept()[0] as peer:
                        peer.settimeout(2)
                        if peer.recv(4096):
                            accepted.set()
                        release.wait(2)
                except OSError as error:
                    failures.append(type(error).__name__)

            worker = threading.Thread(target=serve)
            worker.start()
            started = time.monotonic()
            try:
                result = self.run_probe(
                    [*VALID, "--timeout-ms", "100"],
                    DISPLAY=f"127.0.0.1:{port - 6000}",
                )
                elapsed = time.monotonic() - started
            finally:
                release.set()
                worker.join(3)
            self.assertFalse(worker.is_alive(), "private endpoint did not terminate")
            self.assertFalse(failures, failures)
            self.assertTrue(accepted.is_set(), "child never attempted X setup")
            self.assertEqual(result.returncode, 124, result.stderr)
            self.assertIn("stage=process_deadline", result.stderr)
            self.assertNotIn("event=finished", result.stdout)
            self.assertLess(elapsed, 2, "setup escaped the watchdog")


if __name__ == "__main__":
    unittest.main()
