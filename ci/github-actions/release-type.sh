#! /bin/bash

if [ -z "$RELEASE_TAG" ]; then
  echo '::error ::Environment variable RELEASE_TAG is required but misssing'
  exit 1
fi

case "$RELEASE_TAG" in
  *.*.*-*)
    echo '::set-output name=release_type::prerelease'
    echo '::set-output name=is_release::true'
    echo '::set-output name=is_prerelease::true'
  ;;

  *.*.*)
    echo '::set-output name=release_type::official'
    echo '::set-output name=is_release::true'
    echo '::set-output name=is_prerelease::false'
  ;;

  *)
    echo '::set-output name=release_type::none'
    echo '::set-output name=is_release::false'
  ;;
esac
