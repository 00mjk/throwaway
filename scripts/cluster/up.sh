#!/usr/bin/env bash
set -euo pipefail

# Cluster
if ! (k3d cluster list | grep -q throwaway); then
  k3d cluster create throwaway \
    --api-port 127.0.0.1:6443 \
    --k3s-arg "--no-deploy=traefik@server:*" \
    --port "80:80@loadbalancer" \
    --port "433:433@loadbalancer" \
    --port "5432:5432@loadbalancer" \
    --port "6379:6379@loadbalancer" \
    --wait

  k3d kubeconfig merge throwaway --kubeconfig-switch-context
else
  kubectl cluster-info
fi

# Infrastructure
flux check --pre
flux install

flux create source git throwaway \
  --url https://github.com/CathalMullan/throwaway-flux \
  --branch master \
  --interval 3m

flux create secret git flux-system \
  --url ssh://git@github.com/CathalMullan/throwaway-flux \
  --private-key-file ~/.ssh/id_ed25519

flux create kustomization throwaway \
  --source GitRepository/throwaway \
  --path "clusters/dev" \
  --prune true \
  --interval 10m

flux reconcile kustomization throwaway --with-source

# Secrets
echo "Waiting for Vault to come up..."
until curl --silent --head --fail http://vault.127.0.0.1.nip.io/v1/sys/health; do
  sleep 15
done

pushd ~/workspace/throwaway-terraform/environment/dev
terraform init -upgrade
terraform apply -auto-approve
popd
