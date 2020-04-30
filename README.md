## Steps to reproduce

0. Make sure `rustc` and `cargo` are installed in your system.
1. Clone this repo.
2. Switch to `dprint-issue-report-intersection-hanging` branch.
3. Run [./reproduce.sh](./reproduce.sh)

## Expected behavior

There shouldn't be any error as [./valid-typescript-file.ts](./valid-typescript-file.ts) is not only correctly formatted, but also a perfectly valid TypeScript file.

## Actual behavior

There is an error.

## Formatting Rules

I use `dprint-typescript-plugin` only (without `dprint-core`).

The formatting rules in located in `fn modify` of [rules/mod.rs:10-18](./src/rules/mod.rs#L10-18).

## About this repo

This repo is an application that I am working on.
I find it convenient to use this repo as bug report so I made a separate branch for it.
