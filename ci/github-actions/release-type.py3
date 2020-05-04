#! /usr/bin/env python3
from os import environ
import re
import json

release_tag = environ.get('RELEASE_TAG', None)

if not release_tag:
  print('::error ::Environment variable RELEASE_TAG is required but missing')
  exit(1)

with open('./nodejs/wasm32-wasi/package.json') as package_json:
  data = json.load(package_json)

  if type(data) != dict:
    print('::error Content of package.json is not an object')
    exit(1)

  version = data.get('version', None)

  if not version:
    print('::error ::package.json#version is required but missing')
    exit(1)

  if version != release_tag:
    print('::warning ::RELEASE_TAG does not match package.json#version')
    print('::set-output name=release_type::none')
    print('::set-output name=is_release::false')
    exit(0)

if re.match(r'^[0-9]+.[0-9]+.[0-9]+-.+$', release_tag):
  print('::set-output name=release_type::prerelease')
  print('::set-output name=is_release::true')
  print('::set-output name=is_prerelease::true')
  exit(0)

if re.match(r'^[0-9]+.[0-9]+.[0-9]+$', release_tag):
  print('::set-output name=release_type::official')
  print('::set-output name=is_release::true')
  print('::set-output name=is_prerelease::false')
  exit(0)

print('::set-output name=release_type::none')
print('::set-output name=is_release::false')
exit(0)
