[![CI](https://github.com/CathalMullan/throwaway/actions/workflows/ci.yml/badge.svg)](https://github.com/CathalMullan/throwaway/actions/workflows/ci.yml)
[![Coverage](https://github.com/CathalMullan/throwaway/actions/workflows/coverage.yml/badge.svg)](https://github.com/CathalMullan/throwaway/actions/workflows/coverage.yml)
[![Security](https://github.com/CathalMullan/throwaway/actions/workflows/security.yml/badge.svg)](https://github.com/CathalMullan/throwaway/actions/workflows/security.yml)

# Throwaway
A **mock** payments processing system.

(An excuse to play around with certain technologies)

## Getting Started

Repositories

```
git clone git@github.com:CathalMullan/throwaway.git ${HOME}/workspace/throwaway
git clone git@github.com:CathalMullan/throwaway-terraform.git ${HOME}/workspace/throwaway-terraform
```

Cluster

```
export TAG="v5.2.2"
curl --silent --fail https://raw.githubusercontent.com/rancher/k3d/main/install.sh | bash

export FLUX_VERSION="0.25.3"
curl --silent --fail https://fluxcd.io/install.sh | bash

brew install kubectl terraform
cd "${HOME}/workspace/throwaway"
./scripts/cluster/up.sh
```

API (Local)

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
cargo install cargo-make
cd "${HOME}/workspace/throwaway"
cargo run
```

API (Cluster)

```
brew install tilt
tilt up --stream=true
```
