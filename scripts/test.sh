#!/usr/bin/env bash
set -euo pipefail

# FIXME: Figure out how to start API from test suite.
until curl --output /dev/null --silent http://0.0.0.0:8000/health; do
  sleep 1
done

cargo test --all --tests --all-targets --all-features -- --nocapture
