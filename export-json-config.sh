#! /bin/bash
set -o errexit pipefail nounset

cd "$(dirname "$0")"
mkdir -pv exports

run() {
  cargo run --bin=sane-fmt-export-json-config -- "$1" --output="exports/sane-fmt.$1.json"
}

run typescript
run dprintrc
