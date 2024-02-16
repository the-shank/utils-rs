//! Tool to download all the crates from crates.io.

use clap::Parser;
use eyre::{eyre, ContextCompat, Result};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use tracing::level_filters::LevelFilter;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use utils_rs::common::consts::LOGGER_NAME;

// TODO: also add a file containing the date that the crates were downloaded
// or add the date to the name of the downloads dir

/// Tool to download all the crates from crates.io.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// directory where the crates are to be downloaded
    #[arg(long)]
    download_dir: String,

    /// extract the downloaded archive of each crate
    #[arg(long, default_value_t = false)]
    extract: bool,

    /// only donwload packages matching the regexp
    #[arg(long)]
    regexp: Option<String>,

    /// dry-run
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

fn setup_tracing() {
    // let filter = EnvFilter::builder()
    //     .with_default_directive(LevelFilter::INFO.into())
    //     .with_env_var(LOGGER_NAME)
    //     .from_env_lossy();
    // tracing_subscriber::fmt().with_env_filter(filter).init();
    tracing_subscriber::fmt::init();
}

fn download_crate_from_url<P: AsRef<Path>>(url: &str, download_dir: P) -> Result<i32> {
    Command::new("wget")
        .arg("-c")
        .arg("--no-verbose")
        .arg("--content-disposition") // to keep the crate name
        .arg("--directory-prefix")
        .arg(download_dir.as_ref())
        .arg(url)
        .stdout(Stdio::null())
        .status()?
        .code()
        .wrap_err_with(|| eyre!("unable to get the exit code for the wget command"))
}

fn extract_archive<P: AsRef<Path>>(filename: &str, cwd: P) -> Result<i32> {
    Command::new("tar")
        .arg("-xf")
        .arg(filename)
        .stdout(Stdio::null())
        .current_dir(cwd.as_ref())
        .status()?
        .code()
        .wrap_err_with(|| eyre!("unable to get the exit_code for the extraction command"))
}

fn main() -> Result<()> {
    // setup tracing
    setup_tracing();

    // parse args
    let args = Args::parse();
    info!("args: {args:#?}");
    // std::process::exit(1);

    // create the download dir
    fs::create_dir_all(&args.download_dir)?;

    // update the crates index
    let mut index = crates_index::GitIndex::new_cargo_default()?;
    info!("Updating index...");
    index.update()?;
    let index_config = index.index_config()?;

    // regex?
    let regex = if let Some(ref r) = args.regexp {
        Some(Regex::new(r)?)
    } else {
        None
    };

    // apply regexp filtering (if applicable)
    let filtered = index.crates().filter(|crate_release| {
        if let Some(ref rexp) = regex {
            rexp.is_match(crate_release.name())
        } else {
            true
        }
    });

    for crate_releases in filtered {
        if let Some(ver) = crate_releases.highest_normal_version() {
            if let Some(download_url) = ver.download_url(&index_config) {
                info!(
                    ">> downloading `{}` from {}",
                    crate_releases.name(),
                    download_url
                );

                // TODO: add retries for failed downloads

                if !args.dry_run {
                    let exit_code = download_crate_from_url(&download_url, &args.download_dir)?;
                    debug!("wget exit_code : {exit_code}");

                    if exit_code != 0 {
                        continue;
                    }

                    // extract the archive and remove the archive after extracting it
                    if args.extract {
                        let downloaded_filename = format!("{}-{}.crate", ver.name(), ver.version());
                        let exit_code = extract_archive(&downloaded_filename, &args.download_dir)?;
                        debug!("tar exit_code : {exit_code}");

                        if exit_code == 0 {
                            let downloaded_filepath =
                                PathBuf::from_str(&args.download_dir)?.join(downloaded_filename);
                            fs::remove_file(downloaded_filepath)?;
                        }
                    }
                }
            };
        }
    }

    Ok(())
}
