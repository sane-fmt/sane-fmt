#! /bin/bash
set -o errexit -o pipefail -o nounset
./clippy.sh
./fmt.sh --check
cargo build
exec cargo test
