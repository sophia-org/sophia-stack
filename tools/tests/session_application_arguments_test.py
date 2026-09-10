"""Execute the launcher's argument assembly without its TTY or process lifecycle."""
from pathlib import Path
import os
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SOURCE = (ROOT / "tools/run_sophia_session.sh").read_text()


def block(start, end):
    if SOURCE.count(start) != 1:
        raise AssertionError(f"ambiguous launcher boundary: {start!r}")
    beginning = SOURCE.index(start)
    finish = SOURCE.index(end, beginning)
    return SOURCE[beginning:finish]


# These production sections select fallbacks and construct argv only. The
# wrapper's input guard, TTY modes, session validation and exec are excluded.
ASSEMBLY = "\n".join([
    'set -euo pipefail',
    'source "$ROOT_DIR/tools/lib/session_terminal.sh"',
    # Simulate a host without installed fallback applications. Explicit test
    # executables remain ordinary filesystem paths and are never executed.
    'command() { if [[ "$1" == -v ]]; then return 1; fi; builtin command "$@"; }',
    block("normal_application_defaults=false\n", 'SESSION_LABEL='),
    block('hagia_browser_bin=""\n', 'lifecycle_phase complete preflight'),
    'if false; then :',
    block('else\n    terminal_bin=', 'session_args=(\n'),
    'input_source_args=()',
    block('session_args=(\n', '# Every requested flag reached the vector.'),
    'printf "%s\\0" "${session_args[@]}"',
])


class SessionApplicationArguments(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="sophia-launch-arguments-")
        self.directory = Path(self.temporary.name)
        self.terminal = self.directory / 'custom terminal $literal;not-a-command'
        self.browser = self.directory / 'custom browser [literal]'
        for executable in (self.terminal, self.browser):
            executable.write_text('#!/bin/sh\nexit 91\n')
            executable.chmod(0o700)
        self.desktop = self.directory / "desktop.kdl"
        self.desktop.write_text("schema 1\n")

    def tearDown(self):
        self.temporary.cleanup()

    def assemble(self, **changes):
        environment = {
            "PATH": "/usr/bin:/bin",
            "HOME": str(self.directory),
            "ROOT_DIR": str(ROOT),
            "SESSION_PROFILE": "hagia",
            "SESSION_STARTUP": "terminal",
            "TRUECOLOR_PROOF": "false",
            "FIREFOX_M10_ANY_PROOF": "false",
            "FIREFOX_M10_PROOF": "false",
            "FIREFOX_M10_RENDERING_PROOF": "false",
            "FIREFOX_M10_DIALOG_PROOF": "false",
            "FIREFOX_M10_PRIMARY_PROOF": "false",
            "FIREFOX_M10_SELECTION_PROOF": "false",
            "FIREFOX_M10_LIFECYCLE_PROOF": "false",
            "SOPHIA_DESKTOP_PROFILE": str(self.desktop),
            "SOPHIA_TERMINAL_BIN": str(self.terminal),
            "SOPHIA_HAGIA_BROWSER_BIN": str(self.browser),
            "SOPHIA_FIREFOX_BIN": str(self.browser),
            "SOPHIA_HAGIA_BIN": "",
            "SOPHIA_BIN": "/test/sophia",
            "DISPLAY_NAME": ":77",
            "firefox_m10_profile_dir": str(self.directory / "proof-profile"),
        }
        for key, value in changes.items():
            if value is None:
                environment.pop(key, None)
            else:
                environment[key] = value
        result = subprocess.run(
            ["bash", "-c", ASSEMBLY], env=environment, stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False,
        )
        return result

    def args(self, **changes):
        result = self.assemble(**changes)
        self.assertEqual(result.returncode, 0, result.stderr.decode())
        self.assertTrue(result.stdout.endswith(b"\0"))
        return [os.fsdecode(value) for value in result.stdout[:-1].split(b"\0")]

    @staticmethod
    def application_args(arguments):
        return [argument for argument in arguments if argument.startswith((
            "--session-app", "--session-action", "--session-start",
        ))]

    def test_normal_hagia_uses_only_literal_default_commands(self):
        self.assertEqual(self.application_args(self.args()), [
            f"--session-app-default=terminal={self.terminal}",
            "--session-action-default=terminal=terminal",
            "--session-start-default=terminal",
            f"--session-app-default=browser={self.browser}",
            "--session-action-default=browser=browser",
        ])

    def test_no_startup_keeps_defaults_without_overriding_profile_startup(self):
        self.assertEqual(self.application_args(self.args(SESSION_STARTUP="none")), [
            f"--session-app-default=terminal={self.terminal}",
            "--session-action-default=terminal=terminal",
            f"--session-app-default=browser={self.browser}",
            "--session-action-default=browser=browser",
        ])

    def test_a_profile_does_not_require_any_discovered_default_apps(self):
        self.assertEqual(self.application_args(self.args(
            SOPHIA_TERMINAL_BIN=None, SOPHIA_HAGIA_BROWSER_BIN=None,
        )), [])

    def test_explicit_invalid_fallback_paths_are_diagnosed(self):
        for variable, role in (("SOPHIA_TERMINAL_BIN", "terminal"),
                               ("SOPHIA_HAGIA_BROWSER_BIN", "browser")):
            with self.subTest(role=role):
                result = self.assemble(**{variable: str(self.directory / "missing")})
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(f"default {role} is not executable", result.stderr.decode())

    def test_firefox_proof_keeps_explicit_adapters_and_isolated_profile(self):
        arguments = self.application_args(self.args(
            FIREFOX_M10_ANY_PROOF="true", FIREFOX_M10_PROOF="true",
            SOPHIA_TERMINAL_KIND="kitty",
        ))
        self.assertFalse(any("-default=" in argument for argument in arguments))
        self.assertIn(f"--session-app=terminal={self.terminal}", arguments)
        self.assertIn("--session-start=terminal", arguments)
        self.assertIn("--session-app-arg=terminal=--config", arguments)
        self.assertIn("--session-app-arg=terminal=linux_display_server=x11", arguments)
        self.assertIn(f"--session-app=browser={self.browser}", arguments)
        self.assertIn("--session-app-arg=browser=--no-remote", arguments)
        self.assertIn("--session-app-arg=browser=--new-instance", arguments)
        self.assertIn("--session-app-arg=browser=--profile", arguments)
        self.assertIn(f"--session-app-arg=browser={self.directory / 'proof-profile'}", arguments)
        self.assertIn(
            f"--session-app-arg=terminal={ROOT}/tools/fixtures/firefox_m10_kitty_probe.sh",
            arguments,
        )

    def test_truecolor_proof_keeps_palette_and_terminal_fixture(self):
        arguments = self.application_args(self.args(
            TRUECOLOR_PROOF="true", SOPHIA_TERMINAL_KIND="kitty",
        ))
        self.assertFalse(any("-default=" in argument for argument in arguments))
        self.assertIn("--session-start=terminal", arguments)
        self.assertIn("--session-start=palette", arguments)
        self.assertIn("--session-app=palette=/test/sophia", arguments)
        self.assertIn(
            f"--session-app-arg=terminal={ROOT}/tools/fixtures/truecolor_kitty_probe.sh",
            arguments,
        )

    def test_native_proof_retains_xterm_adapter_and_explicit_terminal_role(self):
        arguments = self.application_args(self.args(
            SESSION_PROFILE="native", SOPHIA_TERMINAL_KIND="xterm",
        ))
        self.assertEqual(arguments, [
            f"--session-app=terminal={self.terminal}",
            "--session-app-arg=terminal=-cm",
            "--session-app-arg=terminal=-dc",
            "--session-start=terminal",
            "--session-app-arg=terminal=-title",
            "--session-app-arg=terminal=Sophia Native TTY3",
            "--session-action-app=terminal=terminal",
        ])


if __name__ == "__main__":
    unittest.main()
