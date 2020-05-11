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

<pre><code>cargo install <a href="https://crates.io/crates/sane-fmt">sane-fmt</a></code></pre>

### From NPM

#### WASM (all platform)

<pre><code>npm i -g <a href="https://www.npmjs.com/package/@sane-fmt/wasm32-wasi">sane-fmt</a></code></pre>

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

<pre><code>yay -S <a href="https://aur.archlinux.org/packages/sane-fmt/">sane-fmt</a></code></pre>

#### Download prebuilt binary

<pre><code>yay -S <a href="https://aur.archlinux.org/packages/sane-fmt-bin/">sane-fmt-bin</a></code></pre>

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

## Become a Patron

[My Patreon Page](https://patreon.com/khai96_).

## Frequently Asked Questions

### What is this program?

`sane-fmt` is an opinionated code formatter for TypeScript and JavaScript powered by [dprint](https://dprint.dev/).

### What is the point of this program?

I want to apply a single, consistent formatting for all my codes regardless of environment without complex tooling.

To use Prettier or Dprint would mean having to set up Node.js even in non-Node.js environments.

Setting up `sane-fmt` is simple: just download the binary.

### How to customized the rules?

Customization is antithetical to the purpose of this project. Just fork this project if you want your own version of `sane-fmt`, or just use Dprint if you want convenient customization.

I plan to divide this package into multiple reusable crates in the future, this would allow use to supply your own rules to create your own version of `sane-fmt`. However, I will never make the rule configurable for the end-user.

## License

[MIT](https://git.io/Jflmx) © [Hoàng Văn Khải](https://github.com/KSXGitHub/)
