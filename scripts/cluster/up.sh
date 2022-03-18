#!/usr/bin/env bash
set -euo pipefail

if ! (k3d cluster list | grep -q throwaway); then
  echo "Starting cluster"

  # FIXME: Look into caching strategies for K3d images - https://github.com/rancher/k3d/issues/906
  k3d cluster create throwaway \
    --api-port 127.0.0.1:6443 \
    --image "rancher/k3s:v1.23.1-k3s2" \
    --k3s-arg "--no-deploy=traefik@server:*" \
    --registry-create k3d-throwaway-registry \
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
  --timeout 5m \
  --interval 5m

echo "Applying Flux manifests"
flux create kustomization throwaway \
  --source GitRepository/throwaway \
  --path "clusters/dev" \
  --prune true \
  --timeout 5m \
  --interval 5m

echo "Waiting for Flux to reconcile"
flux reconcile kustomization throwaway --with-source --timeout 5m

echo "Waiting for Flux System to be ready"
kubectl --namespace flux-system wait kustomization/flux-system --for=condition=ready --timeout 5m

echo "Waiting for Vault to come up"
until curl --silent --head --fail --output /dev/null http://vault.127.0.0.1.nip.io/v1/sys/health; do
  sleep 1
done

echo "Waiting for Database to come up"
until pg_isready --quiet --host localhost --port 5432; do
  sleep 1
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
