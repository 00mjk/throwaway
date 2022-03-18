#!/usr/bin/env bash
set -euo pipefail

cargo sqlx prepare -- --lib
