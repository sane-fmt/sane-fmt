#! /usr/bin/env node
const console = require('console')
const path = require('path')
const fs = require('fs')
const process = require('process')
const toml = require('toml')
const run = require('./lib/run')

const cargoManifest = require.resolve('../Cargo.toml')
const version = toml.parse(fs.readFileSync(cargoManifest)).package?.version
const nodejsDir = path.resolve(__dirname, '../nodejs')

if (typeof version !== 'string') {
  console.error(
    `Expecting Cargo.toml#package.version to be a string but received a ${typeof version} instead`,
  )
  throw process.exit(1)
}

let shouldUpdate = false
for (const target of fs.readdirSync(nodejsDir)) {
  const targetManifest = path.join(nodejsDir, target, 'package.json');
  const targetData = require(targetManifest);
  if (version === targetData.version) continue
  shouldUpdate = true
  console.info(
    `Version mismatch: ${version} vs ${wasmData.version}. Correcting...`,
  )
  targetData.version = version
  const json = JSON.stringify(targetData, undefined, 2) + '\n';
  fs.writeFileSync(targetManifest, json)
}

if (shouldUpdate) {
  console.info('Change detected. Updating manifest files...')

  console.info('Updating Cargo.lock')
  run('cargo', 'build')

  console.info('Commit changes')
  run('git', 'add', '.')
  run('git', 'commit', '-m', version)

  console.info('Creating git tag')
  run('git', 'tag', version)
}
