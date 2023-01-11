use clap::Parser;
use color_eyre::eyre::{Context, Result};
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

// TODO: collect directories that cannot be read
// TODO: collect entries that cannot be read
fn process_dir(dir: &PathBuf, opts: &Opts) -> Result<()> {
    if opts.verbose {
        println!("Processing: {}", dir.display());
    }

    for entry in
        read_dir(dir).with_context(|| format!("error processing dir: {}", dir.display()))?
    {
        let entry =
            entry.with_context(|| format!("error processing some file in {}", dir.display()))?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            // process directory
            process_dir(&entry.path(), opts)?;
        } else if file_type.is_symlink() {
            // process symlink
            let target = entry.path();
            if !target.try_exists().with_context(|| {
                format!("Failed to check if symlink exists: {}", target.display())
            })? {
                println!("Broken symlink: {}", target.display());
            };
        }
    }

    Ok(())
}
