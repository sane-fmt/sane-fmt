#! /usr/bin/env python
from os import environ
import re

release_tag = environ.get('RELEASE_TAG', None)

if not release_tag:
  print('Environment variable RELEASE_TAG is required but missing')
  exit(1)

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
