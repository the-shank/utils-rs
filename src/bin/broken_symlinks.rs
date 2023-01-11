use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use std::time::Instant;

extern crate utils_rs;
use utils_rs::common::parsers;

/// A simple utility to find broken symlinks
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[arg(value_parser=parsers::parse_dir)]
    root: Option<PathBuf>,

    #[arg(default_value_t = false, short, long)]
    verbose: bool,
}

/// Config for the application
struct Opts {
    verbose: bool,
}

impl Opts {
    pub(crate) fn new(args: &Args) -> Self {
        Self {
            verbose: args.verbose,
        }
    }
}

fn main() -> Result<()> {
    // setup color_eyre panic and error report handlers
    color_eyre::install()?;

    // parse args
    let args = Args::parse();
    // TODO: keep only a reference to root (later)
    let root_dir = args.root.clone().unwrap_or(env::current_dir()?);

    // construct options from the args
    let opts = Opts::new(&args);

    // fire off!
    let start = Instant::now();
    process_dir(&root_dir, &opts)?;
    println!("Completed in: {:.2?}", start.elapsed());

    Ok(())
}

fn process_dir(dir: &PathBuf, opts: &Opts) -> Result<()> {
    if opts.verbose {
        println!("Processing: {}", dir.display());
    }

    for entry in read_dir(dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            // process directory
            process_dir(&entry.path(), opts)?;
        } else if file_type.is_symlink() {
            // process symlink
            let target = entry.path();
            match target.try_exists() {
                Ok(_) => println!("Broken Symlink: {}", target.display()),
                Err(_) => Err(eyre!(
                    "Failed to check if symlink exists: {}",
                    target.display()
                ))?,
            }
        }
    }

    Ok(())
}
