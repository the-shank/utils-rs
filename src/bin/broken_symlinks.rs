use clap::Parser;
use std::path::PathBuf;
use std::env;
use color_eyre::eyre::Result;

extern crate utils_rs;
use utils_rs::common::parsers;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_parser=parsers::parse_dir)]
    root: Option<PathBuf>,
}


fn main() -> Result<()>{
    let args = Args::parse();
    let root_dir = args.root.unwrap_or(env::current_dir()?);
    dbg!(&root_dir);

    Ok(())
}
