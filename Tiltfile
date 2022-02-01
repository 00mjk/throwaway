default_registry(
    'localhost:5000',
    host_from_cluster='registry:5000'
)

sync_src = sync('src', '/app/src')
docker_build(
    'throwaway',
    context='.',
    dockerfile='Dockerfile',
    live_update=[sync_src],
    entrypoint="cargo run",
)

k8s_yaml(helm('charts/throwaway'))
