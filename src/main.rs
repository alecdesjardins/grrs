use std::fs::File;
use clap::Parser;
use std::io;
use std::io::BufRead;
use std::path::Path;
use anyhow::{Context, Result};
use grrs::find_matches;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Args {
    /// The pattern to look for
    #[arg(short, long, required = true)]
    patterns: Vec<String>,

    /// The path to the file to read
    path: std::path::PathBuf,

    /// Ignore case distinctions in patterns and data
    #[arg(short, long, required = false, action = clap::ArgAction::SetTrue)]
    ignore_case: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let lines = read_lines(&args.path)
        .with_context(|| format!("Failed to read lines from `{}`", args.path.display()))?;

    for line_result in lines {
        let line = line_result
            .with_context(|| format!("Failed to read a line from `{}`", args.path.display()))?;
        find_matches(&line, &args.patterns, args.ignore_case, &mut io::stdout())
            .with_context(|| format!("Could not search for pattern `{:?}` in file `{}`", args.patterns, args.path.display()))?;
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let path = filename.as_ref().to_owned(); // Clone the path
    let file = File::open(&path).with_context(|| format!("could not read file `{}`", &path.display()))?;
    Ok(io::BufReader::new(file).lines())
}