#!/usr/bin/env sh
set -e

echo "Running migrations..."
/opt/docker/migrations.sh

echo "Starting API..."
/opt/docker/hot_reload.sh
