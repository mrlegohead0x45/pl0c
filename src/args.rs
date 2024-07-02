use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub(crate) struct Args {
    pub(crate) filename: String,
}
