## present

**present** is a tool that lets you interpolate arbitrary scripts that get
interpreted by the shell into your markdown documents.

For example, here is a file called `foo.md`. You can place commands within
a fenced code block:

````
foo!

```present:'cargo run -- --help'

```
````

which can then get transformed by invoking the `present` binary on `foo.md`:

```bash
$ present --in-place --path foo.md
```

and the document gets modified in-place:

````
foo!

```present:'cargo run -- --help'
present 0.0.0
A tool that lets you interpolate arbitrary scripts into your markdown documents.

USAGE:
    present [OPTIONS]

OPTIONS:
    -h, --help           Print help information
        --in-place       Modify documents in place.
        --interactive    Interactively present markdown documents.
        --path <PATH>    A file or directory path to present.
        --recursive      Recursively present markdown documents.
        --remove         Remove commands within markdown documents.
    -V, --version        Print version information
```
````
