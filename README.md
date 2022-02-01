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
brew install kubectl k3d fluxcd/tap/flux terraform
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
