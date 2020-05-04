#! /usr/bin/env node
const console = require('console')
const fs = require('fs')
const process = require('process')
const toml = require('toml')
const run = require('./lib/run')

const cargoManifest = require.resolve('../Cargo.toml')
const wasmManifest = require.resolve('../nodejs/wasm32-wasi/package.json')

const version = toml.parse(fs.readFileSync(cargoManifest)).package?.version
const wasmData = require(wasmManifest)

if (typeof version !== 'string') {
  console.error(
    `Expecting Cargo.toml#package.version to be a string but received a ${typeof version} instead`,
  )
  throw process.exit(1)
}

console.info(`File: ${wasmManifest}`)
if (version !== wasmData.version) {
  console.info(
    `Version mismatch: ${version} vs ${wasmData.version}. Correcting...`,
  )
  wasmData.version = version
  const json = JSON.stringify(wasmData, undefined, 2) + '\n'
  fs.writeFileSync(wasmManifest, json)

  console.info('Updating Cargo.lock')
  run('cargo', 'build')

  console.info('Check if there are untracked files')
  run('git', 'diff', '--exit-code')

  console.info('Creating git tag')
  run('git', 'tag', version)
}
