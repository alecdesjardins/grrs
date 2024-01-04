use clap::Parser;
use std::io;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    /// The pattern to look for
    #[arg(short, long, required = true)]
    patterns: Vec<String>,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.patterns, &mut io::stdout())
        .with_context(|| format!("could not search for pattern `{:?}` in file `{}`", args.patterns, args.path.display()))?; // ? is a shortcut for try! macro

    Ok(())
}