#! /bin/bash
set -o errexit -o pipefail -o nounset

cd "$(dirname "$0")"
mkdir -pv exports

run() {
	./run.sh sane-fmt-export-json-config "$1" --output="exports/sane-fmt.$1.json"
}

run typescript
run dprint
