use std::{fs::File, io::BufReader};

use clap::Parser;

use crate::args::Args;

mod args;
mod token;
mod tokeniser;

fn main() {
    let args = Args::parse();
    let f = File::open(&args.filename).expect("file should be openable");
    let r = BufReader::new(f);
    let t = tokeniser::Tokeniser::new(&r);
    println!("Hello, world!");
}
