default_registry(
  'localhost:5000',
  host_from_cluster='registry:5000'
)

custom_build(
  ref='throwaway-dev',
  command=(
    'nix build .#throwawayDevImage --out-link throwaway-dev-image'
      + ' && docker load --input throwaway-dev-image'
      + ' && docker tag throwaway-dev:latest "$EXPECTED_REF"'
  ),
  deps=['./src/', 'Cargo.lock', 'Cargo.toml'],
)

k8s_yaml(helm('charts/throwaway'))
