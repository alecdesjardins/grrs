use clap::Parser;
use std::io;
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

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut io::stdout())
        .with_context(|| format!("could not search for pattern `{}` in file `{}`", args.pattern, args.path.display()))?; // ? is a shortcut for try! macro

    Ok(())
}