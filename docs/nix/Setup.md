# Nix Intro

## External Dependencies
* curl
* git
* docker

## Install Nix

```
sh <(curl -L https://nixos.org/nix/install)
```

## Enable experimental features

```
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```
