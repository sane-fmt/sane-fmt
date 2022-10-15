#! /bin/bash
set -o errexit -o pipefail -o nounset

compile() {
	dirname=./nodejs/"$1"
	echo Compiling "$dirname"
	cd "$dirname" || exit $?
	pnpm exec tsc
}

compile wasm32-wasi
