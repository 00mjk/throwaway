#!/usr/bin/env bash
set -euxo pipefail

cargo test --all --tests --all-targets --all-features
