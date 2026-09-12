#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"

# Stable WM: generated wire/corpora, Rust and retained C clients, Hagia, and
# black-box lifecycle/restart behavior. Missing independent checkouts are fatal.
tools/check_policy_client_matrix.sh

# Experimental shell: every retained revision/capability corpus, independent C
# decoders, Narthex, protected admission and black-box lifecycle behavior.
tools/check_shell_protocol.sh

# Experimental output authority has no declarative schema or independent client
# yet. Retain all of its current Rust codec, transport, service and owner seams
# here without calling them a stability proof.
cargo test --offline -q -p sophia-protocol --test protocol output_
cargo test --offline -q -p sophia-runtime --test output_ipc
cargo test --offline -q -p sophia-runtime --test output_transport
cargo test --offline -q -p sophia-runtime --test output_service
cargo test --offline -q -p sophia-session --test live_output_authority
cargo test --offline -q -p sophia-wm-demo --test output_v1

printf '%s\n' \
    'sophia_native_protocol_family schema=1 status=complete wm=stable shell=experimental output=experimental roles=3 independent_wm=c,hagia independent_shell=c,narthex output_schema=false'
