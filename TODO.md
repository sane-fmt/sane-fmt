# TODO

* [x] Integration tests
* [x] Improve messages
  * [x] `--details count` should not print even `scan` messages
  * [x] `--details count` should not print `find` and `skip`
  * [x] ANSI color
  * [x] Refactor: `color.rs` should return `Style` directly
  * [x] Use symbols or emoji for log indicators
  * [x] Diff should have color
  * [x] Log passed files (green)
  * [x] Use actual color for logs (red, green, dim)
* [x] CI
  * [x] GitHub Actions: Test
  * [x] GitHub Actions: Release
  * [x] Travis CI: Test
* [x] Treat symlink like a normal file
* [ ] When a directory name is given, traverse that directory
* [x] Do not use `cp` for test
* [x] Enable `write.rs` for Windows
* [ ] Add a README
* [ ] Change description
* [x] Run sha256 and sha512 for released artifacts
* [ ] Release for Node.js
  * [ ] WASM as main package
  * [ ] Native binary as side packages
  * [ ] Automatic deployment
