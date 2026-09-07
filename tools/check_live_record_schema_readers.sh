#!/bin/sh
# Offline source tripwire; field semantics are covered by verifier fixtures.
set -eu
repo_root="$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)"
case "$#:${1:-}" in
    0:) exec python3 -B "$repo_root/tools/check_live_record_schema_readers.py" ;;
    1:--self-test) exec python3 -B "$repo_root/tools/tests/check_live_record_schema_readers_test.py" ;;
    *) echo 'usage: check_live_record_schema_readers.sh [--self-test]' >&2; exit 2 ;;
esac
