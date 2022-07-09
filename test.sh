#! /bin/bash
set -o errexit -o pipefail -o nounset
./clippy.sh
cargo build
exec cargo test
