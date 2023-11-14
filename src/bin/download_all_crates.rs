//! Tool to download all the crates from crates.io.

use std::process::Command;
use tracing::debug;
use tracing::Level;

// TODO: also add a file containing the date that the crates were downloaded
// or add the date to the name of the downloads dir

// TODO: download dir should be provided by a command line argument
const DOWNLOAD_DIR: &str = "/workdisk/shank/crates";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // create the download dir
    let _ = Command::new("mkdir").arg("-p").arg(DOWNLOAD_DIR).status()?;

    // now download the crates
    let mut index = crates_index::GitIndex::new_cargo_default()?;
    println!("Updating index…");
    index.update()?;

    let index_config = index.index_config()?;

    for crate_releases in index.crates() {
        if let Some(ver) = crate_releases.highest_normal_version() {
            if let Some(download_url) = ver.download_url(&index_config) {
                debug!(
                    "downloading `{}` from {}",
                    crate_releases.name(),
                    download_url
                );

                // TODO: add retries for failed downloads

                let mut cmd = Command::new("wget");
                cmd.arg("-c")
                    .arg("--content-disposition") // to keep the crate name
                    .arg("--directory-prefix")
                    .arg(DOWNLOAD_DIR)
                    .arg(download_url);
                let exit_code = cmd.status()?.code();
                debug!(?exit_code);
            };
        }
    }

    Ok(())
}
