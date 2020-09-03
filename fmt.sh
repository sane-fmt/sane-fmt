#! /bin/bash
set -o errexit -o pipefail -o nounset

FMT_UPDATE="${FMT_UPDATE:-false}"
cargo_fmt_flag=()
sane_fmt_flag=()

if [ "$FMT_UPDATE" = 'true' ]; then
  cargo_fmt_flag=()
  sane_fmt_flag=('--write')
elif [ "$FMT_UPDATE" = 'false' ]; then
  cargo_fmt_flag=('--check')
  sane_fmt_flag=()
else
  echo "error: \$FMT_UPDATE is neither 'true' or 'false': '$FMT_UPDATE'" > /dev/stderr
  exit 1
fi

cargo run --bin=sane-fmt nodejs/*/src scripts preview ci "${sane_fmt_flag[@]}"
cargo fmt -- "${cargo_fmt_flag[@]}"
