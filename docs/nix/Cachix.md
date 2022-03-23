# Cachix

## Install

```
nix-env -iA cachix -f https://cachix.org/api/v1/install
```

## Authenticate
* https://app.cachix.org/personal-auth-tokens

```
cachix authtoken ${TOKEN}
sudo cachix use throwaway
```
