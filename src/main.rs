use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.with_context(|| format!("could not read line from `{}`", args.path.display()))?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}