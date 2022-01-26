FROM rustlang/rust:nightly-alpine
MAINTAINER Cathal Mullan <contact@cathal.dev>

ENV RUSTFLAGS "-C target-feature=-crt-static --cfg tokio_unstable --cfg sqlx_macros_unstable"

RUN apk add --update \
    ca-certificates \
    libc-dev \
    openssl-dev \
    protoc \
    curl \
    jq

RUN cargo install cargo-watch

WORKDIR app

COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo fetch

ADD docker /opt/docker
HEALTHCHECK \
  --interval="60s" \
  --timeout="6s" \
  --retries="6" \
  CMD ["/opt/docker/healthcheck.sh"]

COPY . .

ENTRYPOINT ["/bin/sh"]
CMD ["/opt/docker/command.sh"]
