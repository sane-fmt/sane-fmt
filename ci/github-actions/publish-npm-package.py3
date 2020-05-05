#! /usr/bin/env python3
from os import environ
from subprocess import Popen
from textwrap import dedent

if 'NPM_AUTH_TOKEN' not in environ:
  print('::error ::NPM_AUTH_TOKEN is required but missing')
  exit(1)

target = environ.get('TARGET')
if not target:
  print('::error ::TARGET is required but missing')
  exit(1)

print('Creating .npmrc')
with open(f'./nodejs/{target}/.npmrc', 'w') as file:
  content = dedent('''
    //registry.npmjs.org/:_authToken=${NPM_AUTH_TOKEN}
    registry=https://registry.npmjs.org/
    always-auth=true
  ''').strip()
  file.write(content)

is_prerelease = environ.get('IS_PRERELEASE')
if not is_prerelease:
  print('::error ::IS_PRERELEASE is required but missing')
  exit(1)

tag_dict = {
  'true': 'prerelease',
  'false': 'latest',
}

tag = tag_dict[is_prerelease.lower()]

print('Publishing')
process = Popen(
  ['npm', 'publish', '--access', 'public', '--tag', tag],
  cwd=f'./nodejs/{target}/',
)
process.communicate()
exit(process.returncode)
