use clap::Parser;
use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use std::process::Command;

use utils_rs::common::parsers;

#[derive(Parser, Debug)]
struct Args {
    /// name and version of the crate in the format <name>:<version>
    #[arg(value_parser=parsers::parse_name_version)]
    name_version: (String, String),
}

fn download(name: &str, version: &str) -> Result<()> {
    let url = format!("https://static.crates.io/crates/{name}/{name}-{version}.crate");
    let _ = Command::new("wget")
        .arg(&url)
        .arg("--output-document")
        .arg(format!("{name}-{version}.tar.gz"))
        .status()
        .wrap_err_with(|| eyre!("Failed to download crate from {}", url))?;
    Ok(())
}

fn main() -> Result<()> {
    // setup color_eyre panic and error report handlers
    color_eyre::install()?;

    // parse args
    let args = Args::parse();

    let (crate_name, version) = args.name_version;
    download(&crate_name, &version)
}
