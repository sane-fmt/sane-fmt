#! /bin/bash
set -o errexit -o pipefail -o nounset

compile() {
  dirname=./nodejs/"$1"
  echo Compiling "$dirname"
  cd "$dirname" || exit $?
  pnpx tsc
}

compile wasm32-wasi
