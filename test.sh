#! /bin/bash
set -o errexit -o pipefail -o nounset
cargo fmt -- --check
./clippy.sh
cargo build
exec cargo test
