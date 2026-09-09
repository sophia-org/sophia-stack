#!/usr/bin/env bash
set -euo pipefail

# Real Mesa clients negotiate buffers against a private X server, then the
# native renderer captures and reads their pixels. Pixmap exports also preserve
# imported storage and its DRM modifier. No KMS or live display.
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

echo "CPU and imported pixmap texture pixels on $node"
test_args=(--quiet --offline -p sophia-session --all-features --lib
    glx_pixmap_export::egl_pixmap_export::)
listed="$(cargo test "${test_args[@]}" -- --ignored --list)"
[[ "$(printf '%s\n' "$listed" | grep -c ': test$')" == 7 ]] || {
    echo "Expected all seven pixmap export cases; refusing a partial proof." >&2
    exit 1
}
SOPHIA_PIXMAP_TEST_DEVICE="$node" cargo test "${test_args[@]}" -- --ignored --nocapture

echo "Native implicit-modifier pixels on $node"
test_args=(--quiet --offline -p sophia-renderer-native-egl --features gbm-platform
    --test implicit_modifier)
listed="$(cargo test "${test_args[@]}" -- --ignored --list)"
[[ "$(printf '%s\n' "$listed" | grep -c ': test$')" == 1 ]] || {
    echo "Expected the native implicit-import pixel test; refusing an empty proof." >&2
    exit 1
}
SOPHIA_TEST_RENDER_NODE="$node" cargo test "${test_args[@]}" -- --ignored --nocapture
echo "Pixmap exports and native implicit-modifier pixels passed"
