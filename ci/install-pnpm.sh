#! /bin/sh

if [ -z "$PNPM_VERSION" ]; then
	# shellcheck disable=SC2016
	echo '[ERROR] $PNPM_VERSION is undefined' >/dev/stderr
	exit 1
fi

ref=301414cec74a2b6b63c95b42f2ad1790ccb980ed
exec curl -fsSL https://raw.githubusercontent.com/pnpm/self-installer/$ref/install.js | node
