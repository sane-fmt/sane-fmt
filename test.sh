#! /bin/sh
./clippy.sh || exit $?
./fmt.sh --check || exit $?
cargo build || exit $?
exec cargo test
