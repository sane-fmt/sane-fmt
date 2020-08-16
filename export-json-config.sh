#! /bin/bash
set -o errexit pipefail nounset

cd "$(dirname "$0")"
mkdir -pv exports
cargo run --bin=sane-fmt-export-json-config typescript --output=exports/sane-fmt.typescript.json
cargo run --bin=sane-fmt-export-json-config dprintrc --output=exports/sane-fmt.dprintrc.json
