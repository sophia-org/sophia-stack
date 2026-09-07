#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture="$(mktemp -d)"
trap 'rm -rf -- "$fixture"' EXIT
# Only shell builtins are available. With tracing enabled, even an ignored
# external-command attempt is visible and fails this test.
for name in two_xterm milestone3; do
    set +e
    PATH="$fixture" /bin/bash -x "$root/tools/live_session_${name}_hardware_proof.sh" \
        >"$fixture/stdout" 2>"$fixture/stderr"
    status=$?
    set -e
    [[ "$status" == 2 && ! -s "$fixture/stdout" ]]
    grep -Fq 'historical hardware gate is retired' "$fixture/stderr"
    grep -Fq "verify_live_session_${name}_evidence.sh" "$fixture/stderr"
    if grep -E '^\+ ' "$fixture/stderr" | grep -Ev '^\+ (printf |exit 2$)'; then
        echo "retired launcher attempted more than its retirement notice" >&2
        exit 1
    fi
done
# Historical readers must continue to validate their original evidence.
"$root/tools/verify_live_session_two_xterm_evidence.sh" \
    "$root/tools/fixtures/live_session_two_xterm_evidence_pass.log" >/dev/null
sed 's/namespace_profile=classic_shared/namespace_profile=confined/g' \
    "$root/tools/fixtures/live_session_two_xterm_evidence_pass.log" >"$fixture/confined.log"
"$root/tools/verify_live_session_milestone3_evidence.sh" \
    "$root/tools/fixtures/live_session_two_xterm_evidence_pass.log" "$fixture/confined.log" >/dev/null
printf '%s\n' 'retired launcher and historical evidence checks passed'
