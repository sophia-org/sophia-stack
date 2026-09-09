#!/usr/bin/env bash
set -euo pipefail

# Real Mesa clients negotiate buffers against a private X server, then the
# native renderer captures and reads their pixels. No KMS or live display.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

node="${SOPHIA_PIXMAP_TEST_DEVICE:-}"
if [[ -z "$node" ]]; then
    for candidate in /dev/dri/renderD*; do
        if [[ -r "$candidate" && -w "$candidate" ]]; then
            node="$candidate"
            break
        fi
    done
fi
if [[ -z "$node" || ! -r "$node" || ! -w "$node" ]]; then
    echo "No writable render node; the client first-frame proof cannot run here." >&2
    exit 2
fi

cd "$ROOT_DIR"
echo "GLX and EGL first-frame pixels on $node"
test_args=(--quiet --offline -p sophia-session --all-features --lib
    first_buffer_with_measured_modifiers)
listed="$(cargo test "${test_args[@]}" -- --ignored --list)"
[[ "$(printf '%s\n' "$listed" | grep -c ': test$')" == 2 ]] || {
    echo "Expected both GLX and EGL first-frame tests; refusing an empty or partial proof." >&2
    exit 1
}
SOPHIA_PIXMAP_TEST_DEVICE="$node" cargo test "${test_args[@]}" -- --ignored --nocapture
echo "GLX and EGL first-frame pixels passed"
