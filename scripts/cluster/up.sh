#!/usr/bin/env bash
set -euo pipefail

if ! (k3d cluster list | grep -q throwaway); then
  echo "Starting cluster"
  k3d cluster create throwaway \
    --api-port 127.0.0.1:6443 \
    --k3s-arg "--no-deploy=traefik@server:*" \
    --port "80:80@loadbalancer" \
    --port "433:433@loadbalancer" \
    --port "5432:5432@loadbalancer" \
    --port "6379:6379@loadbalancer" \
    --wait
fi

echo "Showing cluster info"
kubectl cluster-info

echo "Running Flux checks"
flux check --pre

echo "Installing Flux"
flux install

echo "Configuring Git source"
flux create source git throwaway \
  --url https://github.com/CathalMullan/throwaway-flux \
  --branch master \
  --interval 3m

echo "Applying Flux manifests"
flux create kustomization throwaway \
  --source GitRepository/throwaway \
  --path "clusters/dev" \
  --prune true \
  --interval 10m

flux reconcile kustomization throwaway --with-source

echo "Waiting for Flux to reconcile"
kubectl --namespace flux-system wait kustomization/flux-system --for=condition=ready --timeout=5m

echo "Waiting for Vault to come up..."
until curl --silent --head --fail http://vault.127.0.0.1.nip.io/v1/sys/health; do
  sleep 15
done

# FIXME: Really not a fan of this...
echo "Provisioning Vault"
if [ "$(uname)" == "Darwin" ]; then
  pushd "${HOME}/workspace/throwaway-terraform/environment/dev"
else
  pushd "/home/runner/work/throwaway/throwaway/throwaway-terraform/environment/dev"
fi

terraform init -upgrade
terraform apply -auto-approve
popd

# FIXME: Move to app itself performing provision
# FIXME: For now, figure out how to cache this and main app build, compiles times are 3x local on CI
echo "Provisioning Database"
cargo install --locked --version 0.5.* sqlx-cli
cd sql
export DATABASE_URL="postgresql://postgres:password@localhost:5432/postgres"
cargo sqlx migrate run
