#!/usr/bin/env bash
set -euo pipefail

docker build \
  -t nix-throwaway \
  -f nix.Dockerfile \
  .
