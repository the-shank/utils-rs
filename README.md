# utils-rs

Collection of utilities (written in rust)

- `broken_symlinks` : Tool to identify all broken symlinks under a particular directory.
- `download_all_crates` : Tool to download all the crates from crates.io.
- `dlcrate` : Tool to download a specific crate from crates.io.
- `cdf` : Tool to calculate the cumulative distribution function values.

## `download_all_crates`

Sample usage:

```bash
# download all crates starting with "x"
download_all_crates --download-dir=/path/to/crates_x --regexp="^x" --extract
```

```shell
# takes input from stdin and writes to stdout
cat input.txt | cdf

# output will be like:
# 0,0.09090909090909091
# 0.05,0.18181818181818182
# 0.1,0.18181818181818182
# 0.15,0.2727272727272727
# 0.2,0.2727272727272727
# 0.25,0.36363636363636365
# 0.3,0.36363636363636365
# 0.35,0.45454545454545453
# 0.4,0.45454545454545453
# 0.45,0.6363636363636364
# 0.5,0.6363636363636364
# 0.55,0.6363636363636364
# 0.6,0.7272727272727273
# 0.65,0.7272727272727273
# 0.7,0.8181818181818182
# 0.75,0.8181818181818182
# 0.8,0.9090909090909091
# 0.85,0.9090909090909091
# 0.9,1
# 0.95,1
# 1,1
```
