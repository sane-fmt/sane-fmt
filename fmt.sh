#!/bin/bash

if [ "$FMT_UPDATE" = 'true' ]; then
  cargo_fmt_flag=()
  sane_fmt_flag=('--write')
elif [ "$FMT_UPDATE" = 'false' ] || [ "$FMT_UPDATE" = '' ]; then
  cargo_fmt_flag=('--check')
  sane_fmt_flag=()
fi

cargo run nodejs/*/src "${sane_fmt_flag[@]}"
cargo fmt -- "${cargo_fmt_flag[@]}"
