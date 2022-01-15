#!/usr/bin/env bash
set -euxo pipefail

export DATABASE_URL="postgresql://postgres:password@localhost:5432/postgres"
cargo install --locked --version 0.5.* sqlx-cli

cargo sqlx prepare
