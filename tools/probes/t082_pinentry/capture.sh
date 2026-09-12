#!/usr/bin/env bash
# One-command dummy-only capture. Do not source this script.
set -euo pipefail
umask 077
trace_production=()
if [[ $# == 2 && $1 == --production ]]; then
    [[ -x /usr/sbin/xprop && -x /usr/sbin/xwininfo ]] || { echo 'Production hints require xprop and xwininfo.' >&2; exit 1; }
    trace_production=(--production-manifest "$2")
elif [[ $# != 0 ]]; then
    echo 'Usage: capture.sh [--production /absolute/candidate.json]' >&2; exit 2
fi
[[ $EUID != 0 ]] || { echo 'Capture runs as the ordinary user, without sudo.' >&2; exit 1; }
export T082_TOOLS
T082_TOOLS=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
export T082_RELEASE=/opt/sophia/releases/0.1.0-702efef161dd
export T082_BUNDLE=/home/niltempus/dev/sophia-stack/.artifacts/t082-probe-v8
export T082_CAPTURE
mkdir -p /tmp/sophia-pinentry-trace
T082_CAPTURE=$(mktemp -d /tmp/sophia-pinentry-trace/run.XXXXXX)
printf 'Capture directory: %s\nCapture needs no sudo.\n' "$T082_CAPTURE"
[[ -f $T082_RELEASE/manifest ]] || {
    echo 'Pinned capture release 0.1.0-702efef161dd is absent; verify the installed release and retarget this capture before running.' >&2
    exit 1
}
grep -Fxq 'commit=702efef161ddf758affdbf68bf139b00aa21a5be' "$T082_RELEASE/manifest" || {
    echo 'Release identity mismatch.' >&2; exit 1;
}
(cd "$T082_RELEASE" && sha256sum -c --quiet SHA256SUMS)
trace_tty=$(tty 2>/dev/null || true)
[[ $trace_tty =~ ^/dev/tty[0-9]+$ ]] || { echo 'Start on a local VT after ending the old desktop normally.' >&2; exit 1; }
[[ -n ${XDG_RUNTIME_DIR:-} && -d $XDG_RUNTIME_DIR ]] || { echo 'Login runtime directory missing.' >&2; exit 1; }
export T082_ORIGINAL_STATE=${XDG_STATE_HOME:-$HOME/.local/state}
trace_config=${XDG_CONFIG_HOME:-$HOME/.config}
for trace_name in ${!SOPHIA_@} ${!HAGIA_@}; do unset "$trace_name"; done
export SOPHIA_BIN="$T082_RELEASE/target/release/sophia"
# Use the exact packaged policy, not a mutable policy from a previous install.
export SOPHIA_HAGIA_BIN="$T082_RELEASE/target/release/hagia"
export SOPHIA_HAGIA_SHELL_BIN="$T082_RELEASE/target/release/narthex"
export SOPHIA_TTY_MODE_HELPER="$T082_RELEASE/tools/sophia_tty_mode.py"
export SOPHIA_DESKTOP_PROFILE="$trace_config/sophia/desktop.kdl"
for trace_file in "$SOPHIA_BIN" "$SOPHIA_HAGIA_BIN" "$SOPHIA_HAGIA_SHELL_BIN" /usr/sbin/kitty; do
    [[ -x $trace_file ]] || { printf 'Missing executable: %s\n' "$trace_file" >&2; exit 1; }
done
[[ -f $SOPHIA_DESKTOP_PROFILE ]] || { echo 'Desktop profile missing.' >&2; exit 1; }
python3 -B "$T082_TOOLS/prepare.py" --release "$T082_RELEASE" --capture "$T082_CAPTURE" "${trace_production[@]}"
cp "$T082_RELEASE/manifest" "$T082_CAPTURE/release.manifest"
if [[ ${#trace_production[@]} == 0 ]]; then
    for trace_file in "$T082_BUNDLE/pinentry-baseline" "$T082_BUNDLE/pinentry-instrumented"; do
        [[ -x $trace_file ]] || { printf 'Missing executable: %s\n' "$trace_file" >&2; exit 1; }
    done
    cp "$T082_BUNDLE/identity.json" "$T082_CAPTURE/probe.identity.json"
else
    sha256sum "$T082_TOOLS/production.py" "$T082_TOOLS/production_protocol.py" "$T082_TOOLS/production_hints.py" "$T082_CAPTURE/production-candidate.json" >"$T082_CAPTURE/production.identity.sha256"
fi
sha256sum "$SOPHIA_BIN" "$SOPHIA_HAGIA_BIN" "$SOPHIA_HAGIA_SHELL_BIN" "$SOPHIA_DESKTOP_PROFILE" \
    "$T082_TOOLS/runner.py" "$T082_TOOLS/prepare.py" "$T082_TOOLS/capture.sh" "$T082_CAPTURE/run-session" \
    >"$T082_CAPTURE/identity.sha256"
export SOPHIA_TTY_PROFILE=hagia SOPHIA_BUILD_SESSION=false SOPHIA_MANAGE_KEYD=false
export SOPHIA_INSTALLED_SESSION=true SOPHIA_REQUIRE_RUNTIME_DIR=true SOPHIA_REQUIRE_LOCAL_VT=true
export SOPHIA_INPUT_GUARD_ARMING=automatic SOPHIA_SESSION_STARTUP=terminal
export SOPHIA_SESSION_HANDOFF=display_manager SOPHIA_HAGIA_PROFILE_MODE=user
export SOPHIA_INSTALLED_VERSION=0.1.0 SOPHIA_INSTALLED_COMMIT=702efef161ddf758affdbf68bf139b00aa21a5be
export SOPHIA_DESKTOP_PROFILE_SHA256
SOPHIA_DESKTOP_PROFILE_SHA256=$(sha256sum "$SOPHIA_DESKTOP_PROFILE" | awk '{print $1}')
export SOPHIA_X11_AUTHORITY_TRACE=1 SOPHIA_LIVE_VISUAL_PROGRESS=1
export RUST_LOG=info,sophia_x_authority::x11_socket=debug
export XDG_STATE_HOME="$T082_CAPTURE/state"
mkdir -p "$XDG_STATE_HOME"
if [[ ${#trace_production[@]} != 0 ]]; then
    printf '%s\n' 'Production matrix starts automatically. Use only the displayed public dummy values.' 'Answer each action confirmation in the terminal after the candidate exits. Stay on this VT.'
else
printf '%s\n' 'A terminal and one instrumented dummy prompt start automatically. Type test and press Enter once.' \
    'Stay on this VT during each specimen. Each process is bounded to 60 seconds (15 seconds after traced submission).' \
    'When finished, verify the terminal still accepts input, then Ctrl+Alt+Delete logs out.' \
    'If normal logout is unavailable, Ctrl+Alt+Backspace invokes the existing emergency input guard.'
fi
exec "$SOPHIA_BIN" session _supervise --profile=hagia -- "$T082_CAPTURE/run-session"
