//! Tool to download all the crates from crates.io.

use clap::Parser;
use eyre::{eyre, ContextCompat, Result};
use std::fs;
use std::path::PathBuf;
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
    #[arg(short, long)]
    download_dir: String,

    /// extract the downloaded archive of each crate
    #[arg(short, long, default_value_t = false)]
    extract: bool,
}

fn setup_tracing() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var(LOGGER_NAME)
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn main() -> Result<()> {
    // setup tracing
    setup_tracing();

    // parse args
    let args = Args::parse();
    dbg!(&args);
    // std::process::exit(1);

    // create the download dir
    fs::create_dir_all(&args.download_dir)?;

    // now download the crates
    let mut index = crates_index::GitIndex::new_cargo_default()?;
    info!("Updating index...");
    index.update()?;

    let index_config = index.index_config()?;

    for crate_releases in index.crates() {
        if let Some(ver) = crate_releases.highest_normal_version() {
            if let Some(download_url) = ver.download_url(&index_config) {
                info!(
                    ">> downloading `{}` from {}",
                    crate_releases.name(),
                    download_url
                );

                // TODO: add retries for failed downloads

                let exit_code = Command::new("wget")
                    .arg("-c")
                    .arg("--no-verbose")
                    .arg("--content-disposition") // to keep the crate name
                    .arg("--directory-prefix")
                    .arg(&args.download_dir)
                    .arg(download_url)
                    .stdout(Stdio::null())
                    .status()?
                    .code()
                    .wrap_err_with(|| eyre!("unable to get the exit code for the wget command"))?;

                debug!("wget exit_code : {exit_code}");

                if exit_code != 0 {
                    continue;
                }

                // extract the archive and remove the archive after extracting it
                if args.extract {
                    let downloaded_filename = format!("{}-{}.crate", ver.name(), ver.version());
                    let exit_code = Command::new("tar")
                        .arg("-xf")
                        .arg(&downloaded_filename)
                        .stdout(Stdio::null())
                        .current_dir(&args.download_dir)
                        .status()?
                        .code()
                        .wrap_err_with(|| {
                            eyre!("unable to get the exit_code for the extraction command")
                        })?;

                    debug!("tar exit_code : {exit_code}");

                    if exit_code == 0 {
                        let downloaded_filepath =
                            PathBuf::from_str(&args.download_dir)?.join(downloaded_filename);
                        fs::remove_file(downloaded_filepath)?;
                    }
                }
            };
        }
    }

    Ok(())
}
