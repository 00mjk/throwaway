load('ext://restart_process', 'docker_build_with_restart')

default_registry(
    'localhost:5000',
    host_from_cluster='registry:5000'
)

local_resource(
    'rust-compile',
    'cargo build --target=aarch64-unknown-linux-gnu',
    deps=['./src/', 'Cargo.lock', 'Cargo.toml'],
    env={
        "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER": "aarch64-unknown-linux-gnu-gcc",
    }
)

docker_build_with_restart(
    ref='throwaway',
    context='.',
    entrypoint=["throwaway"],
    only=[
        './target/aarch64-unknown-linux-gnu/debug/throwaway',
    ],
    live_update=[
        sync('./target/aarch64-unknown-linux-gnu/debug/throwaway', '/usr/bin/throwaway'),
    ]
)

k8s_yaml(helm('charts/throwaway'))