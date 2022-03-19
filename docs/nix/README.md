# Nix

## External Dependencies
* curl
* git
* docker
* direnv

## Install Nix

```
sh <(curl -L https://releases.nixos.org/nix/nix-2.7.0/install)
```

## Enable experimental features

```
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```

## Trust Project

```
direnv allow .
```

## Clean Store

```
nix-store --gc
```

## Update Lockfile

```
nix flake update
```
