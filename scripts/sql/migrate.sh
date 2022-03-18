#!/usr/bin/env bash
set -euo pipefail

cd sql
cargo sqlx migrate run
