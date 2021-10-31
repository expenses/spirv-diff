# `spirv-diff`

A tool for diffing SPIR-V files

This is a CLI command that takes in the path of two SPIR-V files, disassembles them and then diffs them with `git diff --no-index`. This is something that's extremely trivial to do with a shell script on a Unix, but is more complex on Windows.

## Usage

```
spirv-diff 0.1.0
A tool for diffing SPIR-V files

USAGE:
    spirv-diff <file-a> <file-b>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file-a>    The path of the first SPIR-V file to diff
    <file-b>    The path of the second SPIR-V file to diff
```

As it just uses `git diff`, the diff viewer you have set in git is used. Additionally, you can simply pipe the output into files or other commands.
