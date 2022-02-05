FROM --platform=linux/arm64 debian:11-slim
MAINTAINER "Cathal Mullan <contact@cathal.dev>"

RUN apt-get update && apt-get install -y libgcc-s1 ca-certificates

COPY target/aarch64-unknown-linux-gnu/debug/throwaway /usr/bin/
CMD ["throwaway"]
