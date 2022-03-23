#!/usr/bin/env bash
set -euo pipefail

docker run \
  --interactive \
  --tty \
  --workdir "/app" \
  --volume "${PWD}:/app" \
  nix-throwaway "${@}"
