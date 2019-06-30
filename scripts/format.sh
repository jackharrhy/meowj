#!/usr/bin/env sh

rustup update nightly
rustup component add rustfmt --toolchain nightly
cargo +nightly fmt --all -- --check
