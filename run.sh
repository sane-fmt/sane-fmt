#! /bin/bash
exec cargo run --bin="$1" -- "${@:2}"
