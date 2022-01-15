#!/usr/bin/env bash
set -euxo pipefail

cargo check
cargo fmt --all
cargo clippy --all --tests --all-targets --all-features
