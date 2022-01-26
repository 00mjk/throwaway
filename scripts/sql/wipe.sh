#!/usr/bin/env bash
set -euo pipefail

export DATABASE_URL="postgresql://postgres:password@localhost:5432/postgres"

psql \
  "host=localhost port=5432 dbname=postgres user=postgres password=password" \
  -c "
    SELECT pg_terminate_backend(pg_stat_activity.pid)
    FROM pg_stat_activity
    WHERE pg_stat_activity.datname = 'postgres' AND pid <> pg_backend_pid();
  "

cd sql
cargo sqlx database reset -y
