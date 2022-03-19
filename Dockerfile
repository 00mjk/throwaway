FROM --platform=linux/arm64 debian:11.2-slim
MAINTAINER "Cathal Mullan <contact@cathal.dev>"

COPY target/aarch64-unknown-linux-gnu/debug/throwaway /usr/bin/
CMD ["throwaway"]
