# sane-fmt

[![GitHub Actions Status](https://github.com/sane-fmt/sane-fmt/workflows/Test/badge.svg)](https://github.com/sane-fmt/sane-fmt/actions)
[![Travis Build Status](https://travis-ci.org/sane-fmt/sane-fmt.svg?branch=master)](https://travis-ci.org/sane-fmt/sane-fmt)

Opinionated code formatter for TypeScript and JavaScript.

## Rules

* Prefer single quotes.
* No semicolons.
* Trailing commas for multi-line.
* No function parentheses for arrow function with single argument.
* Multi-line union and intersection use consistent leading separator.

[Preview](https://git.io/JflcU).

Read [rules/mod.rs](https://git.io/JflmV) and [tests/rules.rs](https://git.io/JflOp) for more information.

## Installation

### Download prebuilt binaries

Go to [the release page](https://github.com/sane-fmt/sane-fmt/releases).

### From [crates.io](https://crates.io)

<pre><code>cargo install <a href="https://crates.io/crates/sane-fmt">sane-fmt</a></code></pre>

### From [NPM](https://www.npmjs.com)

#### WASM (all platform)

<pre><code>npm i -g <a href="https://www.npmjs.com/package/@sane-fmt/wasm32-wasi">@sane-fmt/wasm32-wasi</a></code></pre>

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

### From [Arch User Repository](https://aur.archlinux.org)

#### Download source and compile

<pre><code>yay -S <a href="https://aur.archlinux.org/packages/sane-fmt/">sane-fmt</a></code></pre>

#### Download prebuilt binary

<pre><code>yay -S <a href="https://aur.archlinux.org/packages/sane-fmt-bin/">sane-fmt-bin</a></code></pre>

### Use with [Dprint](https://dprint.dev/)

If you already have Dprint, you can skip installing `sane-fmt` binary. Create a `.dprintrc.json` file with the following content:

```json
{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "projectType": "openSource",
  "extends": "https://github.com/sane-fmt/sane-fmt/raw/master/exports/sane-fmt.dprintrc.json",
  "includes": [
    "**/*.js",
    "**/*.ts"
  ],
  "excludes": [
    ".git",
    "node_modules"
  ],
  "plugins": [
    "https://plugins.dprint.dev/typescript-${DPRINT_TYPESCRIPT_VERSION}.wasm"
  ]
}
```

**Notes:**
  * Replace `master` in the `"extends"` line above with appropriate sane-fmt version.
  * Replace `${DPRINT_TYPESCRIPT_VERSION}` above with appropriate [dprint-plugin-typescript](https://github.com/dprint/dprint-plugin-typescript) version.

**See also:**
  * [sane-fmt.dprintrc.json](https://github.com/sane-fmt/sane-fmt/blob/master/exports/sane-fmt.dprintrc.json): Dprint configuration with rules of sane-fmt.
  * [sane-fmt.typescript.json](https://github.com/sane-fmt/sane-fmt/blob/master/exports/sane-fmt.typescript.json): Configuration of dprint-plugin-typescript with rules of sane-fmt.

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

### GitHub Actions

`sane-fmt` also provides a convenient way to integrate with GitHub Actions. To use it, simply add `--log-format=github-actions`, like so:

```sh
sane-fmt --log-format=github-actions --details=diff
```

When this command is executed within a GitHub Actions runner, it will:
* Annotates unformatted files.
* Group diffs by file names (if `--details=diff`).
* Export `total`, `changed`, and `unchanged` as outputs.

_Recommendation:_ This [action](https://github.com/sane-fmt/action) will install `sane-fmt` and execute it for you.

### Print help message

```sh
sane-fmt --help
```

## Become a Patron

[My Patreon Page](https://patreon.com/khai96_).

## Frequently Asked Questions

### What is this program?

`sane-fmt` is an opinionated code formatter for TypeScript and JavaScript powered by [dprint](https://dprint.dev/). You can think of it as a portable Dprint config file that is always up-to-date.

### What is the point of this program?

I want to apply a single, consistent formatting for all my codes regardless of environment without complex tooling.

I have considered using Prettier or Dprint, but that would mean having to set up Node.js even in non-Node.js environments. I also don't like copying my config files back-and-forth to update them.

On the other hand, setting up `sane-fmt` is simple: just download the binary.

### How to customize the rules?

Customization is antithetical to the purpose of this project, and as such, the `sane-fmt` command does not have customization capability.

However, if you still want a copy of `sane-fmt` with your own customized rules, do one of the following:
* Use [`sane-fmt`](https://docs.rs/sane-fmt) as a library crate.
* Fork this project.
* Just use [dprint](#use-with-dprint).

## License

[MIT](https://git.io/Jflmx) © [Hoàng Văn Khải](https://github.com/KSXGitHub/)
