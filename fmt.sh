#! /bin/bash
set -o errexit -o pipefail

FMT_UPDATE="${FMT_UPDATE:-false}"
cargo_fmt_flag=()
sane_fmt_flag=()

case "$FMT_UPDATE" in
  true)
    cargo_fmt_flag=()
    sane_fmt_flag=('--write')
  ;;
  false)
    cargo_fmt_flag=('--check')
    sane_fmt_flag=()
  ;;
  *)
    echo "error: \$FMT_UPDATE is neither 'true' or 'false': '$FMT_UPDATE'" > /dev/stderr
    exit 1
  ;;
esac

cargo run --bin=sane-fmt nodejs/*/src scripts preview ci "${sane_fmt_flag[@]}"
cargo fmt -- "${cargo_fmt_flag[@]}"
