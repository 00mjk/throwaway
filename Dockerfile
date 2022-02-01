FROM rustlang/rust:nightly-alpine
MAINTAINER Cathal Mullan <contact@cathal.dev>

ENV RUSTFLAGS "-C target-feature=-crt-static --cfg tokio_unstable"

RUN apk add --update \
    ca-certificates \
    libc-dev \
    openssl-dev \
    curl \
    jq

RUN cargo install cargo-make

WORKDIR app

COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo fetch

HEALTHCHECK \
  --interval="60s" \
  --timeout="6s" \
  --retries="6" \
  CMD curl --silent "http://0.0.0.0:8000/health"

COPY . .

ENTRYPOINT ["cargo"]
CMD ["run"]
