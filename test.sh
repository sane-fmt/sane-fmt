#!/bin/sh
./clippy.sh || exit $?
./fmt.sh || exit $?
cargo build || exit $?
exec cargo test
