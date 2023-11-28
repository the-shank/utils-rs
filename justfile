default:
  just --list

build:
  cargo build --release

builddev:
  cargo build

install:
  cargo install --path .
