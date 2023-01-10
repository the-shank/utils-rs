use clap::Parser;
use std::path::PathBuf;

extern crate utils_rs;
use utils_rs::common::parsers;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_parser=parsers::parse_dir)]
    root: Option<PathBuf>,
}

fn main() {
    println!("Hello (from broken_symlinks)");
}
