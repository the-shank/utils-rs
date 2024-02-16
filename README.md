# utils-rs

Collection of utilities (written in rust)

- `broken_symlinks` : Tool to identify all broken symlinks under a particular directory.
- `download_all_crates` : Tool to download all the crates from crates.io.
- `dlcrate` : Tool to download a specific crate from crates.io.

## `download_all_crates`

Sample usage:

```bash
# download all crates starting with "x"
download_all_crates --download-dir=/path/to/crates_x --regexp="^x" --extract
```
