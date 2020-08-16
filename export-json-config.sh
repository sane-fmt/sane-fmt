#! /bin/bash
set -o errexit pipefail nounset

cd "$(dirname "$0")"
mkdir -pv exports
cargo run --bin=sane-fmt-json typescript --output=exports/sane-fmt.typescript.json
cargo run --bin=sane-fmt-json dprintrc --output=exports/sane-fmt.dprintrc.json
