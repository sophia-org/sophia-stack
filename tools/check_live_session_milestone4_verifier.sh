#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERIFY="$ROOT_DIR/tools/verify_live_session_milestone4_evidence.sh"
FIXTURES="$ROOT_DIR/tools/fixtures"

"$VERIFY" "$FIXTURES/live_session_milestone4_evidence_pass.log"
if "$VERIFY" "$FIXTURES/live_session_milestone4_evidence_no_mixed_export.log" >/dev/null 2>&1; then
    echo "Milestone 4 verifier accepted evidence without a mixed GPU export" >&2
    exit 1
fi

fixture="$(mktemp -d)"
trap 'rm -rf -- "$fixture"' EXIT
sed -e 's/schema=14/schema=16/' \
    -e 's/present_complete_flip=6/present_complete_copy=6 present_complete_flip=0/' \
    -e 's/status=bounded_complete /status=bounded_complete startup_ready_msec=700 /' \
    "$FIXTURES/live_session_milestone4_evidence_pass.log" >"$fixture/current.log"
"$VERIFY" "$fixture/current.log"
expect_refusal() {
    local description="$1" expression="$2" reason="$3" output
    sed "$expression" "$fixture/current.log" >"$fixture/mutated.log"
    if output="$("$VERIFY" "$fixture/mutated.log" 2>&1)"; then
        echo "Milestone 4 verifier accepted $description" >&2
        exit 1
    fi
    [[ "$output" == *"$reason"* ]] || {
        echo "Milestone 4 refused $description for the wrong reason: $output" >&2
        exit 1
    }
}
expect_refusal 'missing Copy count' 's/present_complete_copy=6 //' 'missing field: present_complete_copy'
expect_refusal 'missing startup proof' 's/startup_ready_msec=700 //' 'missing field: startup_ready_msec'
expect_refusal 'unrequested startup proof' 's/startup_ready_msec=700/startup_ready_msec=not_requested/' 'expected numeric startup_ready_msec'
expect_refusal 'normal completion' 's/schema=16/schema=17/' 'requires a supported startup-proof schema'
expect_refusal 'missing mixed export' 's/native_mixed_exports=7/native_mixed_exports=0/' 'expected positive native_mixed_exports'
expect_refusal 'mismatched Idle count' 's/present_idle=7/present_idle=6/' 'unmatched Complete/Idle'
expect_refusal 'missing successful completion' 's/present_complete_copy=6/present_complete_copy=0/' 'requires a successful Present'
expect_refusal 'undrained Present sources' 's/present_live_sources=0/present_live_sources=1/' 'expected zero present_live_sources'
printf '%s\n' 'Milestone 4 evidence verifier checks passed'
