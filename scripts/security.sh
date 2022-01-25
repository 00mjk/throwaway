#!/usr/bin/env bash
set -euo pipefail

cargo install --locked cargo-audit
cargo audit

cargo install --locked cargo-deny
cargo deny check
