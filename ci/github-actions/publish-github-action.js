#! /usr/bin/env node
const path = require('path')
const fs = require('fs')
const http = require('http')
const process = require('process')
const { setFailed, startGroup, endGroup } = require('@actions/core')
const spawn = require('advanced-spawn-async').default
const { pipe } = require('ts-pipe-compose')
const { dbg } = require('string-template-format-inspect')
const git = require('isomorphic-git')

const {
  RELEASE_TAG,
  GIT_PASSWORD,
  GIT_AUTHOR_NAME,
  GIT_AUTHOR_EMAIL,
} = process.env

if (!RELEASE_TAG) {
  setFailed('Environment variable RELEASE_TAG is required but missing')
  throw process.exit()
}

if (!GIT_PASSWORD) {
  setFailed('Environment variable GIT_PASSWORD is required but missing')
  throw process.exit()
}

if (!GIT_AUTHOR_NAME) {
  setFailed('Environment variable GIT_AUTHOR_NAME is required but missing')
  throw process.exit()
}

if (!GIT_AUTHOR_EMAIL) {
  setFailed('Environment variable GIT_AUTHOR_EMAIL is required but missing')
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

  startGroup('Install pnpm packages...')
  await spawn('pnpm', ['install', '--frozen-lockfile'], { stdio: 'inherit' })
    .onclose
  endGroup()

  const jsonFile = path.join(ACTION_REPO_DIR, 'src/upstream-version.json')

  const content = pipe(
    jsonFile,
    fs.readFileSync,
    String,
    JSON.parse,
  )

  if (typeof content !== 'object') {
    setFailed(
      dbg`Expecting src/upstream-version.json to be an object: ${content}`,
    )
    return
  }

  if (typeof content.upstreamVersion !== 'string') {
    setFailed(
      dbg
        `Expecting src/upstream-version.json#upstreamVersion to be a string: ${content.upstreamVersion}`,
    )
    return
  }

  content.upstreamVersion = RELEASE_TAG
  pipe(
    content,
    x => JSON.stringify(x, undefined, 2) + '\n',
    x => fs.writeFileSync(jsonFile, x),
  )

  startGroup('Build')
  await spawn('pnpm', ['run', 'build'], { stdio: 'inherit' }).onclose
  endGroup()

  startGroup('Test')
  await spawn('pnpm', ['test'], { stdio: 'inherit' }).onclose
  endGroup()

  startGroup('Update')

  for (const filepath of ['dist/index.js', 'src/upstream-version.json']) {
    await git.add({
      fs,
      dir: ACTION_REPO_DIR,
      filepath,
    })
    console.info(`Added ${filepath} to staging`)
  }

  const commit = await git.commit({
    fs,
    dir: ACTION_REPO_DIR,
    author: {
      name: GIT_AUTHOR_NAME,
      email: GIT_AUTHOR_EMAIL,
    },
    message: RELEASE_TAG,
    ref: 'master',
  })
  console.info(`Created commit ${commit}`)

  await git.tag({
    fs,
    dir: ACTION_REPO_DIR,
    ref: RELEASE_TAG,
    object: commit,
  })
  console.info(`Created tag ${RELEASE_TAG} for ref ${commit}`)

  for (const ref of ['master', RELEASE_TAG]) {
    const pushResult = await git.push({
      fs,
      http,
      dir: ACTION_REPO_DIR,
      onAuth,
      remote: 'origin',
      ref,
    })

    if (pushResult.ok) {
      console.info(`Pushed ${ref} to origin`)
    } else {
      setFailed(`Failed to push ${ref} to origin: ${pushResult.error}`)
    }
  }

  endGroup()
}

main().catch(setFailed)
