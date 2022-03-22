#!/usr/bin/env bash
set -euo pipefail

mkdir -p build
cargo tarpaulin --out Html --output-dir build
mv build/tarpaulin-report.html build/index.html
