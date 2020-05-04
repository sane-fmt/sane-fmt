#! /bin/sh

compile() {
  dirname=./nodejs/"$1"
  echo Compiling "$dirname"
  cd "$dirname" || exit $?
  pnpx tsc
}

compile wasm32-wasi
