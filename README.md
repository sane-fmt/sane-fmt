# sane-fmt

[![GitHub Actions Status](https://github.com/KSXGitHub/sane-fmt/workflows/Test/badge.svg)](https://github.com/KSXGitHub/sane-fmt/actions)
[![Travis Build Status](https://travis-ci.org/KSXGitHub/sane-fmt.svg?branch=master)](https://travis-ci.org/KSXGitHub/sane-fmt)

Opinionated code formatter for TypeScript and JavaScript.

## Rules

* Prefer single quotes.
* No semicolons.
* Trailing commas for multi-line.
* No function parentheses for arrow function with single argument.

Read [rules/mod.rs](https://git.io/JflmV) for more.

## Installation

### Download prebuilt binaries

Go to [the release page](https://github.com/KSXGitHub/sane-fmt/releases).

### From crates.io

```sh
cargo install sane-fmt
```

### From NPM

#### WASM (all platform)

```sh
npm i -g @sane-fmt/wasm32-wasi
```

#### Native binaries

The WASM package while work on all platform, it is slow to start. For better speed, install one of the following packages instead:

**For Linux:**
  * [@sane-fmt/x86_64-unknown-linux-gnu](https://www.npmjs.com/package/@sane-fmt/x86_64-unknown-linux-gnu)
  * [@sane-fmt/x86_64-unknown-linux-musl](https://www.npmjs.com/package/@sane-fmt/x86_64-unknown-linux-musl)

**For macOS:**
  * [@sane-fmt/x86_64-apple-darwin](https://www.npmjs.com/package/@sane-fmt/x86_64-apple-darwin)

**For Windows:**
  * [@sane-fmt/x86_64-pc-windows-gnu](https://www.npmjs.com/package/@sane-fmt/x86_64-pc-windows-gnu)
  * [@sane-fmt/x86_64-pc-windows-msvc](https://www.npmjs.com/package/@sane-fmt/x86_64-pc-windows-msvc)

### From Arch User Repository

#### Download source and compile

```sh
yay -S sane-fmt
```

#### Download prebuilt binary

```sh
yay -S sane-fmt-bin
```

## Usage

### Format all TypeScript and JavaScript files

```sh
sane-fmt --write
```

This command would reformat all TypeScript and JavaScript files.

### Check for all TypeScript and JavaScript files

```sh
sane-fmt
```

This command would check all TypeScript and JavaScript files.

### Format only some files

```sh
sane-fmt --write foo.ts bar.js
```

This command would only reformat `foo.ts` and `bar.js`.

### Format all TypeScript and JavaScript files in a directory

```sh
sane-fmt --write src/
```

This command would reformat all TypeScript and JavaScript files within `src/` directory.

### Print help message

```sh
sane-fmt --help
```

## License

[MIT](https://git.io/Jflmx) © [Hoàng Văn Khải](https://github.com/KSXGitHub/)
