load('ext://restart_process', 'docker_build_with_restart')

default_registry(
    'localhost:5000',
    host_from_cluster='registry:5000'
)

local_resource(
    'rust-compile',
    'cargo zigbuild --target aarch64-unknown-linux-gnu',
    deps=['./src/', 'Cargo.lock', 'Cargo.toml'],
)

# FIXME: Distroless, Rootless, see https://github.com/tilt-dev/tilt-extensions/pull/97
docker_build_with_restart(
    ref='throwaway',
    context='.',
    dockerfile='tilt.Dockerfile',
    entrypoint=["throwaway"],
    only=[
        './target/aarch64-unknown-linux-gnu/debug/throwaway',
    ],
    live_update=[
        sync('./target/aarch64-unknown-linux-gnu/debug/throwaway', '/usr/bin/throwaway'),
    ]
)

k8s_yaml(helm('charts/throwaway'))
