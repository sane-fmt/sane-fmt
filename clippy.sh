#! /bin/sh
exec cargo clippy --all-targets -- -D warnings
