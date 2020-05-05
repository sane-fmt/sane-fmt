## Steps to reproduce

0. Clone this repo.
1. Switch to `dprint-issue-semicolons-asi-does-not-work-properly` branch.
2. Open [./scripts/sync-package-version.js](./scripts/sync-package-versions.js).
3. Go to line 30, remove the semicolon.
4. Run `cargo run -- scripts/sync-package-version.js --details=diff --write`

## Expected behavior

It does not modify the file.

## Actual behavior

It put the semicolon back.
