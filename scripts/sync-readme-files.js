#! /usr/bin/env node
const console = require('console')
const path = require('path')
const fs = require('fs')

const mainReadme = path.resolve(__dirname, '../README.md')
const nodejsDir = path.resolve(__dirname, '../nodejs')

for (const name of fs.readdirSync(nodejsDir)) {
  const targetReadme = path.join(nodejsDir, name, 'README.md')
  if (!fs.existsSync(targetReadme)) continue
  console.info('→', name)
  fs.copyFileSync(mainReadme, targetReadme)
}
