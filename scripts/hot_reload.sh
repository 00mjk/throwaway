#!/usr/bin/env bash
set -euo pipefail

cargo install cargo-watch
cargo build

while true; do
  cargo watch --why --watch src --exec run
done
