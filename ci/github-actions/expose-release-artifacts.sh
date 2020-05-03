#!/bin/bash
mkdir ./flatten

[ -d ./downloads ] || {
  echo Folder ./downloads does not exist > /dev/stderr
  exit 1
}

for folder in ./downloads/*; do
  echo Copying "$folder"/* ...
  cp "$folder"/* ./flatten/ || exit $?
done
