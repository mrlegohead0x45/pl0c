use std::{fs::File, io::BufReader};

use anyhow::{Context, Result};
use clap::Parser;

use crate::args::Args;

mod args;
mod error;
mod token;
mod tokeniser;

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(&args.filename)
        .with_context(|| format!("Failed to open '{}'", args.filename))?;
    let r = BufReader::new(f);
    let t = tokeniser::Tokeniser::new(&r);
    println!("Hello, world!");
    Ok(())
}
