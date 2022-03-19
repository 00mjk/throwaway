#!/usr/bin/env bash
set -euo pipefail

export DATABASE_URL="postgresql://postgres:password@localhost:5432/postgres"

pushd sql
cargo sqlx migrate run
popd
