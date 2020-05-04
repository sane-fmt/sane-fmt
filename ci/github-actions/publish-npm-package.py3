#! /usr/bin/env python3
from os import environ, system
from subprocess import Popen
from textwrap import dedent

if 'NPM_AUTH_TOKEN' not in environ:
  print('::error NPM_AUTH_TOKEN is required but missing')
  exit(1)

print('Creating .npmrc')
with open('./nodejs/wasm32-wasi/.npmrc', 'w') as file:
  content = dedent('''
    //registry.npmjs.org/:_authToken=${NODE_AUTH_TOKEN}
    registry=https://registry.npmjs.org/
    always-auth=true
  ''').strip()
  file.write(content)

print('Publishing')
process = Popen(
  ['npm', 'publish', '--access', 'public'],
  cwd='./nodejs/wasm32-wasi/',
)
process.communicate()
exit(process.returncode)
