# Flux Upgrade Process
Create new issue / branch on Flux repository

## Download new Flux CLI locally
Update `README.md` with new version

## Generate Access Token
https://github.com/settings/tokens/new

Note: Throwaway Flux
Expiration: 7 Days
Scopes
    - repo: all

Generate token.

## Bootstrap cluster

```
export GITHUB_TOKEN="TOKEN"

flux bootstrap github \
  --owner CathalMullan \
  --repository throwaway-flux \
  --branch 1-upgrade-to-flux-027 \
  --path "clusters/dev" \
  --private false \
  --personal true \
  --token-auth
```

Cancel initial reconciliation.

Rebase branch onto master.

```
git rebase -X theirs master
```

Remove secret requirement in `gotk-sync.yaml`.

Push to remote branch.

Update `cluster/up.sh` to use new branch.

Run tests suites

```
./cluster/down.sh && ./cluster/up.sh && cargo make verify
```

Open PR for Flux repository and merge.

Delete Access Token (https://github.com/settings/tokens)

Open PR for main repository and merge.
