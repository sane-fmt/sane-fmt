#! /usr/bin/env node
const path = require('path')
const fs = require('fs')
const process = require('process')
const { setFailed } = require('@actions/core')
const git = require('isomorphic-git')

const {
  GIT_PASSWORD,
} = process.env

if (!GIT_PASSWORD) {
  setFailed('Environment variable GIT_PASSWORD is required but missing')
  throw process.exit()
}

const AUTH = Object.freeze({
  username: 'KSXGitHub',
  password: GIT_PASSWORD,
})
const ACTION_REPO_URL =
  'https://github.com/KSXGitHub/github-actions-sane-fmt.git'
const ACTION_REPO_DIR = path.join(__dirname, '../../action-repo')

const onAuth = () => AUTH

async function main() {
  await git.clone({
    fs,
    dir: ACTION_REPO_DIR,
    url: ACTION_REPO_URL,
    onAuth,
  })

  process.chdir(ACTION_REPO_DIR)
}

main().catch(setFailed)
