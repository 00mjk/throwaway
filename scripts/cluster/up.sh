#!/usr/bin/env bash
set -euo pipefail

if ! (k3d cluster list | grep -q throwaway); then
  echo "Starting cluster"

  # FIXME: Look into caching strategies for K3d images - https://github.com/rancher/k3d/issues/906
  k3d cluster create throwaway \
    --api-port 127.0.0.1:6443 \
    --k3s-arg "--no-deploy=traefik@server:*" \
    --registry-create throwaway-registry \
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

echo "Waiting for Vault to come up"
until curl --silent --head --fail --output /dev/null http://vault.127.0.0.1.nip.io/v1/sys/health; do
  sleep 3
done

# FIXME: Really not a fan of this, maybe a submodule if the correct approach?
echo "Provisioning Vault"
if [ "$(uname)" == "Darwin" ]; then
  pushd "${HOME}/workspace/throwaway-terraform/environment/dev"
else
  pushd "/home/runner/work/throwaway/throwaway/throwaway-terraform/environment/dev"
fi

terraform init -upgrade
terraform apply -auto-approve
popd

echo "Waiting for Database to come up"
until psql "host=localhost port=5432 dbname=postgres user=postgres password=password" -c "SELECT 1"; do
  sleep 3
done
