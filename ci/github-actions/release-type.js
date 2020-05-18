#! /usr/bin/env node
const console = require('console')
const path = require('path')
const fs = require('fs')
const { exit } = require('process')
const toml = require('toml')
const git = require('isomorphic-git')
const { setOutput, setFailed } = require('@actions/core')
const { dbg } = require('string-template-format-inspect')

async function main() {
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

  const gitdir = path.resolve(__dirname, '../../.git')
  const tags = await git.listTags({ fs, gitdir })

  if (tags.includes(version)) {
    console.log(`tag ${version} already exists.`)
    setOutput('release_type', 'none')
    setOutput('is_release', 'false')
    setOutput('is_prerelease', 'false')
    setOutput('release_tag', version)
    return
  }

  console.log(`tag ${version} does not exist.`)
  console.log('new release detected!')

  if (/^[0-9]+\.[0-9]+\.[0-9]+-.+$/.test(version)) {
    console.log('release type: prerelease')
    setOutput('release_type', 'prerelease')
    setOutput('is_release', 'true')
    setOutput('is_prerelease', 'true')
    setOutput('release_tag', version)
    return
  }

  if (/^[0-9]+\.[0-9]+\.[0-9]+$/.test(version)) {
    console.log('release type: official')
    setOutput('release_type', 'official')
    setOutput('is_release', 'true')
    setOutput('is_prerelease', 'false')
    setOutput('release_tag', version)
    return
  }

  setFailed(`Unrecognized version syntax: ${version}`)
}

main().catch(error => {
  setFailed(error)
  exit()
})
