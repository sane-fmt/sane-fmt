#! /bin/bash
set -o errexit -o pipefail -o nounset
./clippy.sh
./fmt.sh
cargo build
exec cargo test
