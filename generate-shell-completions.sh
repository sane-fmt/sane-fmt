#! /bin/bash
set -o errexit -o pipefail -o nounset

cd "$(dirname "$0")"
mkdir -pv exports

run() {
	cargo run --bin=sane-fmt-generate-shell-completions -- "$1" --output="exports/$2"
}

run bash completion.bash
run fish completion.fish
run zsh completion.zsh
run powershell completion.ps1
run elvish completion.elv
