#! /usr/bin/env node
const fs = require('fs')
const { exit } = require('process')
const toml = require('toml')
const { setFailed } = require('@actions/core')
const { dbg } = require('string-template-format-inspect')

const cargoTomlPath = require.resolve('../../Cargo.toml')
const cargoTomlContent = toml.parse(
  fs.readFileSync(cargoTomlPath, { encoding: 'utf8' }),
)
const { version } = cargoTomlContent.package || {}

if (!version) {
  setFailed('Cargo.toml#package.version is required but missing')
  throw exit()
}

if (typeof version !== 'string') {
  setFailed(
    dbg
      `Cargo.toml#package.version is expected to be a string, not a ${typeof version}: ${version}`,
  )
  throw exit()
}
