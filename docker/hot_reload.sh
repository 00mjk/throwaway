#!/usr/bin/env sh
set -e

cargo build
while true; do
  cargo watch --why --watch src --exec run
done
