#!/usr/bin/env bash
set -euxo pipefail

cargo install --locked cargo-audit
cargo audit

cargo install --locked cargo-deny
cargo deny check
