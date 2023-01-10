use clap::Parser;
use color_eyre::eyre::Result;
use std::env;
use std::path::PathBuf;

extern crate utils_rs;
use utils_rs::common::parsers;

#[derive(Parser, Debug)]
struct Args {
    // TODO: add description of the program
    #[arg(value_parser=parsers::parse_dir)]
    root: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let root_dir = args.root.unwrap_or(env::current_dir()?);
    dbg!(&root_dir);

    Ok(())
}
