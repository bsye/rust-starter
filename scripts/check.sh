#!/bin/sh
set -eu

cargo fmt --check
cargo clippy --all-targets --no-default-features --features server -- -D warnings
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo test --no-default-features --features server
