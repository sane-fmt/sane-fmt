#!/bin/sh
./clippy.sh || exit $?
cargo build || exit $?
exec cargo test
