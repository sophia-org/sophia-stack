#!/usr/bin/env python3
"""Prepare capture wrappers without starting a session; also used by tests."""
import argparse
from pathlib import Path
import subprocess

def prepare(release, capture, production_manifest=None):
    text = (release / "tools/run_sophia_session.sh").read_text()
    replacements = [
        ('ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"',
         'ROOT_DIR="$T082_RELEASE"\nexport XDG_STATE_HOME="$T082_ORIGINAL_STATE"'),
        ('setsid "${session_launch[@]}" >/dev/null 2>&1 &',
         'setsid "${session_launch[@]}" >>"$T082_CAPTURE/session.raw.log" 2>&1 &'),
        ('setsid "${session_launch[@]}" > >(tee -a "$SESSION_LOG") 2>&1 &',
         'setsid "${session_launch[@]}" >>"$T082_CAPTURE/session.raw.log" 2>&1 &'),
        ('LOG_DIR="${SOPHIA_DIAGNOSTIC_DIR:-$LOG_DIR}"',
         'LOG_DIR="${SOPHIA_DIAGNOSTIC_DIR:-$T082_CAPTURE/wrapper-logs}"'),
        ('session_command=(\n',
         'session_args+=("--session-app=terminal=$T082_CAPTURE/terminal" --session-start=terminal)\nsession_command=(\n'),
    ]
    for old, new in replacements:
        if text.count(old) != 1:
            raise RuntimeError("Installed wrapper anchor mismatch; refusing launch")
        text = text.replace(old, new, 1)
    (capture / "run-session").write_text(text)
    (capture / "terminal").write_text('''#!/usr/bin/env bash
set -euo pipefail
exec /usr/sbin/kitty --config NONE /bin/bash --noprofile --rcfile "$T082_CAPTURE/terminal.rc" -i
''')
    (capture / "terminal.rc").write_text('''printf '%s\\n' 'T082 dummy capture. Use this shell to check ordinary input while the probe runs.'
python3 "$T082_TOOLS/runner.py" --bundle "$T082_BUNDLE" --capture "$T082_CAPTURE/cases" --case instrumented-enter &
''')
    if production_manifest is not None:
        from production import stage_candidate
        import json
        candidate, identity = stage_candidate(production_manifest, capture / "production-source")
        identity["binary_path"] = str(candidate.resolve())
        (capture / "production-candidate.json").write_text(json.dumps(identity) + "\n")
        (capture / "terminal.rc").write_text('''printf '%s\\n' 'T082 production matrix: public dummy values only. Stay on this VT.'
python3 -B "$T082_TOOLS/production.py" --candidate-manifest "$T082_CAPTURE/production-candidate.json" --capture "$T082_CAPTURE/production" --release-manifest "$T082_CAPTURE/release.manifest"
printf '%s\\n' 'Matrix ended. Check ordinary terminal input, then log out normally.'
''')
    for name in ("run-session", "terminal", "terminal.rc"):
        (capture / name).chmod(0o700)
        subprocess.run(["bash", "-n", str(capture / name)], check=True)

if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--release", type=Path, required=True)
    p.add_argument("--capture", type=Path, required=True)
    p.add_argument("--production-manifest", type=Path)
    args = p.parse_args()
    prepare(args.release, args.capture, args.production_manifest)
