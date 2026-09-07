#!/usr/bin/env bash
# Historical evidence remains readable; this launcher no longer owns hardware.
printf '%s\n' \
    'This historical hardware gate is retired.' \
    'For current native-session validation, see docs/validation.md (Native Session Integration).' \
    'Read retained evidence with tools/verify_live_session_milestone3_evidence.sh.' >&2
exit 2
