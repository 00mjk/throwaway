{
  description = "Throwaway";

  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    zig = {
      url = "github:roarkanize/zig-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, naersk, zig }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        localOverlay = import .nix/overlay.nix;
        overlays = [
          localOverlay
        ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        inherit (pkgs) mkShell stdenv lib fetchFromGitHub;
        inherit (pkgs.darwin.apple_sdk.frameworks) SystemConfiguration;

        toolchain = {
          channel = "nightly";
          date = "2022-03-10";
          sha256 = "sha256-wZFBurC0BwL1RbbzZhlLaVTHcKRfHZItTENDm1HlXJ8=";
        };

        rust-toolchain = with fenix.packages.${system}; combine (with toolchainOf toolchain; [
          cargo
          clippy
          rustc
          rustfmt
          rust-src
          rust-std
          (targets.aarch64-apple-darwin.toolchainOf toolchain).rust-std
          (targets.aarch64-unknown-linux-gnu.toolchainOf toolchain).rust-std
        ]);

        naersk-lib = naersk.lib.${system}.override {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };

        zig-master = zig.packages.${system}."0.9.1";

        sqlx-cli = pkgs.sqlx-cli.overrideAttrs (old: rec {
          name = "sqlx-cli-${version}";
          version = "0.5.11";

          src = fetchFromGitHub {
            owner = "launchbadge";
            repo = "sqlx";
            rev = "v${version}";
            sha256 = "sha256-Tz7YzGkQUwH0U14dvsttP2GpnM9kign6L9PkAVs3dEc=";
          };

          cargoSha256 = "sha256-EKuRaVxwotgTPj95GJnrQGbulsFPClSettwS5f0TzoM=";
        });
      in
      rec {
        # `nix develop`
        devShell = mkShell {
          name = "throwaway-shell";

          buildInputs = with pkgs; [ pkgconfig openssl ]
          ++ lib.optional stdenv.isDarwin [
            libiconv
            SystemConfiguration
          ];

          nativeBuildInputs = with pkgs; [
            # Rust
            rust-toolchain

            # Rust Crates
            cargo-deny
            cargo-audit
            sqlx-cli

            cargo-nextest
            cargo-zigbuild

            # Kubernetes
            kubectl
            kube3d # FIXME: v5.2.2
            fluxcd # FIXME: 0.27.0
            tilt
            kubernetes-helm

            # Terraform
            terraform

            # Postgres
            postgresql

            # Zig
            zig-master

            # Nix
            nixpkgs-fmt
          ]
          ++ lib.optional stdenv.isLinux [
            # Rust Crates
            cargo-tarpaulin
          ];
        };
      });
}
