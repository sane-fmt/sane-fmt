#! /bin/bash
set -o errexit -o pipefail -o nounset

FMT_UPDATE="${FMT_UPDATE:-false}"
cargo_fmt_flag=()
sane_fmt_flag=()

if [ "$FMT_UPDATE" = 'true' ]; then
  cargo_fmt_flag=()
  sane_fmt_flag=('--write')
elif [ "$FMT_UPDATE" = 'false' ] || [ "$FMT_UPDATE" = '' ]; then
  cargo_fmt_flag=('--check')
  sane_fmt_flag=()
fi

cargo run --bin=sane-fmt nodejs/*/src scripts preview ci "${sane_fmt_flag[@]}"
cargo fmt -- "${cargo_fmt_flag[@]}"
