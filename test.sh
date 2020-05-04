#! /bin/sh
./clippy.sh || exit $?
./fmt.sh --check || exit $?
cargo build || exit $?
./compile-typescript.sh || exit $?
exec cargo test
