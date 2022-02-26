### present

**present** is a tool that lets you interpolate the standard output of arbitrary
scripts that get interpreted by the shell into your markdown documents.

### Usage

```present cargo run -- --help
present 0.0.0
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

### Example

Below is the contents of a file called `foo.md`. You can place commands at the
start of a fenced code block:

````
foo

```present echo bar
```
````

These commands can then get interpreted by the shell by invoking the `present`
binary on `foo.md`:

```bash
$ present --in-place --path foo.md
```

The document gets modified in-place, with the commands `stdout` interpolated
in-between the fenced code block:

````
foo

```present echo bar
bar
```
````
