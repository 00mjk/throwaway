#!/usr/bin/env bash
set -euo pipefail

cargo test --future-incompat-report --lib --bins -- --nocapture
