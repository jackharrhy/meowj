#!/usr/bin/env sh

rustup update stable
rustup component add clippy --toolchain stable
cargo +stable clippy --all-features -- -D warnings
