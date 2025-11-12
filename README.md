## present

[![release](https://img.shields.io/github/release/terror/present.svg?label=release&style=flat&labelColor=282c34&logo=github)](https://github.com/terror/present/releases/latest)
[![CI](https://github.com/terror/present/actions/workflows/ci.yaml/badge.svg)](https://github.com/terror/present/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/terror/present/graph/badge.svg?token=7CH4XDXO7Z)](https://codecov.io/gh/terror/present)
[![crates.io](https://shields.io/crates/v/present.svg)](https://crates.io/crates/present)
[![downloads](https://img.shields.io/crates/d/present)](https://crates.io/crates/present)
[![dependency status](https://deps.rs/repo/github/terror/present/status.svg)](https://deps.rs/repo/github/terror/present)

**present** is a tool that lets you interpolate the standard output of arbitrary
scripts that get interpreted by the shell into your markdown documents.

Its aim is to provide a nice way to automatically update sections of your
markdown documents that might be the standard output of a command, such as
command-line utility help outputs or benchmarks.

## Demo

Below is a short demo showcasing the main functionality of the program.

[![asciicast](https://asciinema.org/a/499682.svg)](https://asciinema.org/a/499682)

## Installation

`present` should run on any system, including Linux, MacOS, and the BSDs.

The easiest way to install it is by using [cargo](https://doc.rust-lang.org/cargo/index.html),
the Rust package manager:

```bash
cargo install present
```

Otherwise, see below for the complete package list:

#### Cross-platform

<table>
  <thead>
    <tr>
      <th>Package Manager</th>
      <th>Package</th>
      <th>Command</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href=https://www.rust-lang.org>Cargo</a></td>
      <td><a href=https://crates.io/crates/present>present</a></td>
      <td><code>cargo install present</code></td>
    </tr>
    <tr>
      <td><a href=https://brew.sh>Homebrew</a></td>
      <td><a href=https://github.com/terror/homebrew-tap>terror/tap/present</a></td>
      <td><code>brew install terror/tap/present</code></td>
    </tr>
  </tbody>
</table>

### Pre-built binaries

Pre-built binaries for Linux, MacOS, and Windows can be found on [the releases
page](https://github.com/terror/present/releases).

## Usage

You can use `present` from the command-line interface (CLI) or library.

### CLI

Below is the standard output of `present --help`, interpolated by the `present`
binary itself!

```present cargo run -- --help
Interpolate the standard output of arbitrary shell scripts into your markdown files

Usage: present [OPTIONS] [PATH]

Arguments:
  [PATH]  A file or directory path to present.

Options:
      --recursive    Recursively present markdown documents.
      --in-place     Modify documents in place.
      --interactive  Interactively present markdown documents.
      --pretty       Pretty print documents to the terminal.
      --remove       Remove commands within markdown documents.
  -h, --help         Print help
  -V, --version      Print version
```

### Library

`present` can be used as a library by adding this line to the `[dependencies]`
section in `Cargo.toml`:

```present ./bin/get_version
present = "0.2.3"
```

With `present`, you can create a `File` struct by pointing it to a path. This
will parse all codeblocks with the `present` prefix, and add them as commands to
the struct. From there, you can present the file by using the `File::present`
function, which will modify the internal content. From there, you can use the
`File::print` or `File::save` functions to print the presented document to
stdout or save it back to the original file.

```rust
use std::path::PathBuf;

fn main() {
  let mut file = present::File::new(PathBuf::from("README.md")).unwrap();
  file.present().unwrap();
  file.save();
}
```

> The above snippet is tested with rustdoc. A really cool side effect of this,
> is that the test loads the README itself, and runs `present` over it.
> `present` is also used throughout the README (to get help-text and version
> numbers), which means that when running `cargo test`, the README gets
> automatically updated.

You can read more about using the library on [docs.rs](https://docs.rs/present).

## Examples

Below are a few examples showcasing what kind of command result interpolations
`present` is currently able to handle.

<table>
<tr>
<td>
  <code>present foo.md --in-place</code>
</td>
<td>

````ignore
foo

```present echo bar
```
````

</td>
<td>

````ignore
foo

```present echo bar
bar
```
````

</td>
</tr>
<td>
  <code>present foo.md --in-place --remove</code>
</td>
<td>

````ignore
foo

```present echo bar
```
````

</td>
<td>

```ignore
foo

bar
```

</td>
</tr>
</table>

## Prior Art

This project is loosely inspired by [`Cog`](https://github.com/nedbat/cog), the
code generation tool. However, as mentioned above, this project's main target is
markdown documents that may benefit to have certain sections automatically
updated, due to being the result of a command invocation.
