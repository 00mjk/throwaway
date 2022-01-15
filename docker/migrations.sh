#!/usr/bin/env sh
set -e

POSTGRESQL_CONFIG="$(curl --header "X-Vault-Token: ${VAULT_TOKEN}" "${VAULT_URL}/v1/${VAULT_KV_MOUNT}/data/${VAULT_PATH_POSTGRESQL}")"

USER="$(echo "${POSTGRESQL_CONFIG}" | jq -r .data.data.user)"
PASSWORD="$(echo "${POSTGRESQL_CONFIG}" | jq -r .data.data.password)"
HOST="$(echo "${POSTGRESQL_CONFIG}" | jq -r .data.data.host)"
PORT="$(echo "${POSTGRESQL_CONFIG}" | jq -r .data.data.port)"
DB="$(echo "${POSTGRESQL_CONFIG}" | jq -r .data.data.db)"

export DATABASE_URL="postgresql://${USER}:${PASSWORD}@${HOST}:${PORT}/${DB}"
cargo sqlx prepare

cd sql
cargo sqlx migrate run
