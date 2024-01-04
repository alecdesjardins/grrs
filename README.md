# grss, the Rusty grep

## Description
grss is a simple parser for text files, written in Rust. It leverages the [clap](https://crates.io/crates/clap) crate for efficient command line argument parsing.

## Usage
grss can be utilized to parse single files in various ways, providing flexibility in handling text files. The tool accepts the following command line arguments:
```bash
USAGE:
    grrs <PATTERN> <PATH>
```

## Examples
```bash
# Search for a pattern in a file and print the lines that contain it
$ grrs foo test.txt
```