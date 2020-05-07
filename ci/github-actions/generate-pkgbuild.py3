#! /usr/bin/env python3
from os import environ
import re

target = environ.get('TARGET')
if not target:
  print('::error ::TARGET is required but missing')
  exit(1)

release_tag = environ.get('RELEASE_TAG')
if not release_tag:
  print('::error ::RELEASE_TAG is required but missing')
  exit(1)

checksum = None
word_splitter = re.compile(r'\s+')
for line in open('checksums/sha1sum.txt').readlines():
  line = line.strip()
  if line.endswith(target):
    checksum, _ = word_splitter.split(line)

print('checksum', checksum)
