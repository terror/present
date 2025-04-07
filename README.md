## present

[![CI](https://github.com/terror/present/actions/workflows/ci.yaml/badge.svg)](https://github.com/terror/present/actions/workflows/ci.yaml)
[![crates.io](https://shields.io/crates/v/present.svg)](https://crates.io/crates/present)
[![docs.rs](https://img.shields.io/docsrs/present)](https://docs.rs/present)
[![dependency status](https://deps.rs/repo/github/terror/present/status.svg)](https://deps.rs/repo/github/terror/present)

**present** is a tool that lets you interpolate the standard output of arbitrary
scripts that get interpreted by the shell into your markdown documents.

Its aim is to provide a nice way to automatically update sections of your
markdown documents that might be the standard output of a command, such as
command-line utility help outputs or benchmarks.

## Demo

Below is a short demo showcasing the main functionality of the program.

[![asciicast](https://asciinema.org/a/6AO2ME0abbvn93dr4Dh4lenM0.svg)](https://asciinema.org/a/6AO2ME0abbvn93dr4Dh4lenM0)

## Usage

You can use `present` from the command-line interface (CLI) or library.

### CLI

You can install the `present` command-line utility with the rust package manager
[cargo](https://github.com/rust-lang/cargo):

```bash
$ cargo install present
```

In addition, pre-built binaries can be found on the
[releases](https://github.com/terror/present/releases) page.

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
is that the test loads the README itself, and runs `present` over it. `present`
is also used throughout the README (to get help-text and version numbers), which
means that when running `cargo test`, the README gets automatically updated.

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

  ````ignore
  foo

  bar
  ````
</td>
</tr>
</table>

## Prior Art

This project is loosely inspired by [`Cog`](https://github.com/nedbat/cog), the
code generation tool. However, as mentioned above, this project's main target is
markdown documents that may benefit to have certain sections automatically
updated, due to being the result of a command invocation.
