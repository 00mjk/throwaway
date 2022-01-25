#!/usr/bin/env bash
set -euo pipefail

cargo test --all --tests --all-targets --all-features -- --nocapture
