#!/usr/bin/env sh
set -e

until curl --output /dev/null --silent http://0.0.0.0:8000/health; do
  sleep 1
done

curl --silent http://0.0.0.0:8000/health | jq .
echo "API is up!"
