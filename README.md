# grss, the Rusty grep
## Description
This is a simple parser for Rust files. It is written in Rust and uses the [clap](https://crates.io/crates/clap) crate for command line argument parsing.

## Usage
The parser can be used to parse a single file in various ways. The following command line arguments are available:
```
USAGE:
    grrs <PATTERN> <PATH>
```

## Examples
```
# Search for a pattern in a file and print the lines that contain it
$ grrs foo test.txt