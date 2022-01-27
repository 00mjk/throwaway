#!/usr/bin/env bash
set -euo pipefail

cargo check
cargo fmt --all
cargo clippy --all --tests
