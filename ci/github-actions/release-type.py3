#! /usr/bin/env python3
from os import environ
import re
import json

release_tag = environ.get('RELEASE_TAG', None)

if not release_tag:
  print('::error ::Environment variable RELEASE_TAG is required but missing')
  exit(1)

tag_prefix = 'refs/tags/'
if release_tag.startswith(tag_prefix):
  release_tag = release_tag.replace(tag_prefix, '', 1)

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
    print(f'::warning ::RELEASE_TAG ({release_tag}) does not match package.json#version ({version})')
    print('::set-output name=release_type::none')
    print('::set-output name=is_release::false')
    print('::set-output name=is_prerelease::false')
    print(f'::set-output name=release_tag::{release_tag}')
    exit(0)

if re.match(r'^[0-9]+\.[0-9]+\.[0-9]+-.+$', release_tag):
  print('::set-output name=release_type::prerelease')
  print('::set-output name=is_release::true')
  print('::set-output name=is_prerelease::true')
  print(f'::set-output name=release_tag::{release_tag}')
  exit(0)

if re.match(r'^[0-9]+\.[0-9]+\.[0-9]+$', release_tag):
  print('::set-output name=release_type::official')
  print('::set-output name=is_release::true')
  print('::set-output name=is_prerelease::false')
  print(f'::set-output name=release_tag::{release_tag}')
  exit(0)

print('::set-output name=release_type::none')
print('::set-output name=is_release::false')
print('::set-output name=is_prerelease::false')
print(f'::set-output name=release_tag::{release_tag}')
exit(0)
