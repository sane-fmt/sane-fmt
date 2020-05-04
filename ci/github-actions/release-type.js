#! /usr/bin/env node
const process = require('process')
const { setOutput, error } = require('@actions/core')
const { RELEASE_TAG } = process.env

if (!RELEASE_TAG) {
  error('Environment variable RELEASE_TAG is required but missing')
  throw process.exit(1)
}

if (/^[0-9]+.[0-9]+.[0-9]+-.+$/.test(RELEASE_TAG)) {
  setOutput('release_type', 'prerelease')
  setOutput('is_release', 'true')
  setOutput('is_prerelease', 'true')
  throw process.exit(0)
}

if (/^[0-9]+.[0-9]+.[0-9]+$/.test(RELEASE_TAG)) {
  setOutput('release_type', 'official')
  setOutput('is_release', 'true')
  setOutput('is_prerelease', 'false')
  throw process.exit(0)
}

setOutput('release_type', 'none')
setOutput('is_release', 'false')
