#! /bin/bash
# shellcheck disable=SC2035
set -o errexit -o pipefail -o nounset
cd ./flatten
sha1sum * >../sha1sum.txt
sha256sum * >../sha256sum.txt
sha512sum * >../sha512sum.txt
