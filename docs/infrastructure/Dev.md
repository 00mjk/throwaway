# Dev
FIXME: How to protect secrets in the repo, SOPS first maybe? Doesn't matter for dev (yet...)

Linters for YAML and TF and Helm (???)
- kubeval

Setup TLS for endpoints, and in transit (DB, Cache)
When is a service mesh "needed" (Linkerd)

Make all components spit out metrics

Create Grafana dashboards for all components

Eventually inject secrets into pod using **magic**
https://learn.hashicorp.com/tutorials/vault/kubernetes-sidecar?in=vault/kubernetes
https://piotrminkowski.com/2021/12/30/vault-on-kubernetes-with-spring-cloud

Split into namespaces (monitoring, secrets, applications, ???)

For lab env, maybe use AWS Secrets Manager to bootstrap Vault unlock.
Then use SOPS/encrypted secrets in repo?

Sealed Secrets vs SOPS -
SOPS allows for decryption?

Support cluster access RO/RW for DB and Cache
Expose connections to each

Move to rootless containers

Consider looking at Postgres Operator (PGO).
Might not be needed if we want to use RDS in ""prod""

Eventually run one of those K8s scanners against the "finished" cluster.

Use the OIDC GitHub connector to apply/push changes.

Example API: https://github.com/stefanprodan/podinfo
Example API: http://vault.127.0.0.1.nip.io/v1/sys/health

CSRF + CORS ?

Cookies?

Should JWT use a key instead? (via Vault PKI)

Database transactions to prevent half-created hierarchies

How to detect required token refresh?
