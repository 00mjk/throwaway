#!/usr/bin/env bash
set -euxo pipefail

# Migrations
cargo install --locked --version 0.5.* sqlx-cli
export DATABASE_URL="postgresql://postgres:password@localhost:5432/postgres"

pushd sql
cargo sqlx migrate run
popd

# Hot Reload
cargo install cargo-watch
cargo build

while true; do
  cargo watch --why --watch src --exec run
done
