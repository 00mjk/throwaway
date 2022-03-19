{
  description = "Throwaway";

  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
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
      url = "github:arqv/zig-overlay";
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

        toolchain = {
          channel = "nightly";
          date = "2022-03-10";
          sha256 = "sha256-wZFBurC0BwL1RbbzZhlLaVTHcKRfHZItTENDm1HlXJ8=";
        };

        rust-toolchain = with fenix.packages.${system}; combine (with toolchainOf toolchain; [
          cargo
          rustc
          (targets.aarch64-apple-darwin.toolchainOf toolchain).rust-std
          (targets.aarch64-unknown-linux-gnu.toolchainOf toolchain).rust-std
        ]);

        naersk-lib = naersk.lib.${system}.override {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };

        zig-master = zig.packages.${system}.master.latest;
      in
      rec {
        # `nix develop`
        devShell = pkgs.mkShell {
          name = "throwaway-shell";

          buildInputs = with pkgs; []
          ++ lib.optional stdenv.isDarwin [
            libiconv
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          nativeBuildInputs = with pkgs; [
            # Rust
            rust-toolchain

            # Rust Crates
            cargo-deny
            cargo-audit
            cargo-nextest
            cargo-sqlx-cli
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
          ];
        };
      });
}
