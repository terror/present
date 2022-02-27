## present

[![CI](https://github.com/terror/present/actions/workflows/ci.yaml/badge.svg)](https://github.com/terror/present/actions/workflows/ci.yaml)
[![crates.io](https://shields.io/crates/v/present.svg)](https://crates.io/crates/present)

**present** is a tool that lets you interpolate the standard output of arbitrary
scripts that get interpreted by the shell into your markdown documents.

Its aim is to provide a nice way to automatically update sections of your
markdown documents that might be the standard output of a command, such as
command-line utility help sections or benchmarks.

### Demo

Below is a short demo showcasing the main functionality of the program.

[![asciicast](https://asciinema.org/a/Mngwm9d3eJcJWtilQrAvjgh2D.svg)](https://asciinema.org/a/Mngwm9d3eJcJWtilQrAvjgh2D)

### Installation

You can install `present` with the rust package manager Cargo:

```bash
$ cargo install present
```

### Usage

Below is the standard output of `present --help`, interpolated by the `present`
binary itself!

```present cargo run -- --help
present 0.1.1
Interpolate the standard output of arbitrary shell scripts into your markdown files

USAGE:
    present [OPTIONS]

OPTIONS:
    -h, --help           Print help information
        --in-place       Modify documents in place.
        --interactive    Interactively present markdown documents.
        --path <PATH>    A file or directory path to present.
        --pretty         Pretty print documents to the terminal.
        --recursive      Recursively present markdown documents.
        --remove         Remove commands within markdown documents.
    -V, --version        Print version information
```

### Examples

Below are a few examples showcasing what kind of command result interpolations
`present` is currently able to handle.

<table>
<tr>
<td>
  <code>present --path foo.md --in-place</code>
</td>
<td>

  ````
  foo

  ```present echo bar
  ```
  ````
</td>
<td>

  ````
  foo

  ```present echo bar
  bar
  ```
  ````
</td>
</tr>
<td>
  <code>present --path foo.md --in-place --remove</code>
</td>
<td>

  ````
  foo

  ```present echo bar
  ```
  ````
</td>
<td>

  ````
  foo

  bar
  ````
</td>
</tr>
</table>

### Prior Art

This project is loosely inspired by [`Cog`](https://github.com/nedbat/cog), the
code generation tool. However, as mentioned above, this project's main target is
markdown documents that may benefit to have certain sections automatically
updated, due to being the result of a command invocation.
