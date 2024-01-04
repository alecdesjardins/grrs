# grss, the Rusty grep

## Description
grss is a simple parser for text files, written in Rust. It leverages the [clap](https://crates.io/crates/clap) crate for efficient command line argument parsing.

## Usage
grss can be utilized to parse single files in various ways, providing flexibility in handling text files. The tool accepts the following command line arguments:
```bash
USAGE:
    grrs --pattern <PATTERN>... <PATH>
```

## Examples
```bash
# Search for a pattern in a file and print the lines that contain it
$ grrs -p foo test.txt
$ grrs --pattern foo test.txt
```

## Testing
The tool can be tested by running the following command:
```bash
$ cargo test
```

It will run all integrations tests in the `tests` directory and unit tests found in the `src` directory.