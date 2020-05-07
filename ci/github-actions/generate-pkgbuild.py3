#! /usr/bin/env python3
from os import environ, makedirs
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

maintainer = '# Maintainer: Hoàng Văn Khải <hvksmr1996@gmail.com>\n'
license_url = 'https://raw.githubusercontent.com/KSXGitHub/sane-fmt/master/LICENSE.md'

opening = maintainer + '\n# This file is automatically generated. Do not edit.\n'

print('Generating PKGBUILD for sane-fmt...')
makedirs('./pkgbuild/sane-fmt', exist_ok=True)
with open('./pkgbuild/sane-fmt/PKGBUILD', 'w') as pkgbuild:
  content = opening + '\n'
  content += 'pkgname=sane-fmt\n'
  content += f'pkgver={release_tag}\n'
  source_url = f'https://github.com/KSXGitHub/sane-fmt/archive/{release_tag}.tar.gz'
  content += f'source=(sane-fmt-{release_tag}.tar.gz::{source_url} {license_url})\n'
  content += 'sha1sums=(SKIP SKIP)\n'
  content += open('./template/sane-fmt/PKGBUILD').read() + '\n'
  pkgbuild.write(content)

print('Generating PKGBUILD for sane-fmt-bin...')
makedirs('./pkgbuild/sane-fmt-bin', exist_ok=True)
with open('./pkgbuild/sane-fmt-bin/PKGBUILD', 'w') as pkgbuild:
  content = opening + '\n'
  content += 'pkgname=sane-fmt-bin\n'
  content += f'pkgver={release_tag}\n'
  source_url = f'https://github.com/KSXGitHub/sane-fmt/releases/download/{release_tag}/sane-fmt-{target}'
  content += f'source=(sane-fmt-{release_tag}::{source_url} {license_url})\n'
  content += f'sha1sums=({checksum} SKIP)\n'
  content += open('./template/sane-fmt-bin/PKGBUILD').read() + '\n'
  pkgbuild.write(content)
