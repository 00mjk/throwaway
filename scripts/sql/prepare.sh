#!/usr/bin/env bash
set -euo pipefail

export DATABASE_URL="postgresql://postgres:password@localhost:5432/postgres"

cargo sqlx prepare -- --bin throwaway
