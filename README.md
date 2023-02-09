# Wrouch

Wrouch is a cli tool that implements `touch` by Rust and supports Windows.

## Install

`cargo install wrouch`

## Usage

```shell
Wrouch is a cli tool that implements `touch` by Rust and supports Windows.

Usage: wrouch [OPTIONS] [FILE_PATHS]...

Arguments:
  [FILE_PATHS]...

Options:
  -a, --access        change only the access time
  -c, --no-create     do not create any files
  -d, --date <DATE>   parse STRING and use it instead of current time, e.g: '2001-01-01 12:02:03'
  -m, --modification  change only the modification time
  -r, --reference <REFERENCE>  use this file's times instead of current time
  -h, --help          Print help information
  -V, --version       Print version information
```
