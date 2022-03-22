FROM nixos/nix:2.7.0
MAINTAINER "Cathal Mullan <contact@cathal.dev>"

RUN : \
  && echo 'experimental-features = nix-command flakes' >> /etc/nix/nix.conf \
  && nix-channel --add https://nixos.org/channels/nixpkgs-unstable nixpkgs \
  && nix-channel --update \
  && :

WORKDIR app

COPY .nix .nix
COPY flake.nix flake.nix
COPY flake.lock flake.lock
RUN nix develop --verbose --command echo 'Nix shell built.'

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN : \
  && mkdir -p src \
  && echo "fn main() {}" > src/main.rs \
  && echo "" > src/lib.rs \
  && nix develop --verbose --command cargo build --release \
  && rm -rf src \
  && :

ENTRYPOINT ["nix", "develop", "--verbose", "--command", "bash"]
