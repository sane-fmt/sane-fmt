#! /usr/bin/env python3
from os.path import dirname, join, realpath
from toml import load
root = dirname(dirname(dirname(realpath(__file__))))
cargo_toml_path = join(root, 'Cargo.toml')
print(f'::set-output name=root::{root}')
print(f'::set-output name=cargo_toml_path::{cargo_toml_path}')
package_info = load(cargo_toml_path)["package"]
for name in ['name', 'version', 'description']:
  print(f'::set-output name={name}::{package_info[name]}')
