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

    nix-filter = {
      url = "github:numtide/nix-filter";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = github:nmattia/naersk;
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    zig = {
      url = "github:roarkanize/zig-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, nix-filter, naersk, fenix, zig }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (builtins) fromTOML readFile;
        inherit (pkgs) lib mkShell callPackage fetchFromGitHub fetchurl dockerTools;
        inherit (pkgs.darwin.apple_sdk.frameworks) SystemConfiguration;

        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            (import nix/overlays/fluxcd.nix)
            (import nix/overlays/kube3d.nix)
            (import nix/overlays/postgresql.nix)
            (import nix/overlays/terraform.nix)

            (import nix/overlays/sqlx-cli.nix { inherit cargoToml; })

            (import nix/overlays/cargo-nextest.nix { inherit naerskPlatform; })
            (import nix/overlays/cargo-zigbuild.nix { inherit naerskPlatform; })
          ];
        };

        cargoToml = fromTOML (readFile ("${self}/Cargo.toml"));

        zig-toolchain = zig.packages.${system}."0.9.1";
        rust-toolchain = fenix.packages."${system}".fromToolchainFile {
          file = "${self}/rust-toolchain.toml";
          sha256 = "sha256-k6wD6/2qNQ7rmIvGi1ddtKSTUXjXFbIr0Sg2mqF2nYg=";
        };

        naerskPlatform = naersk.lib.${system}.override {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };

        buildInputs = with pkgs; [
          # Build
          pkg-config
          libiconv

          # Rust
          rust-toolchain
          cargo-audit
          cargo-deny
          cargo-make
          cargo-nextest
          cargo-zigbuild
          sqlx-cli

          # Coverage
          grcov
          lcov

          # Kubernetes
          kubectl
          kube3d
          fluxcd
          tilt
          kubernetes-helm

          # Terraform
          terraform

          # Postgres
          postgresql

          # Zig
          zig-toolchain

          # Nix
          nixpkgs-fmt

          # Utilities
          curl
          jq
        ] ++ lib.optional (stdenv.isDarwin) [
          # Build
          SystemConfiguration
        ];
      in
      rec {
        packages = {
          # nix build .#throwaway
          throwaway = callPackage nix/packages/throwaway.nix {
            inherit self pkgs nix-filter cargoToml naerskPlatform buildInputs;
          };

          # nix build .#throwawayZig
          throwawayZig = callPackage nix/packages/throwaway-zig.nix {
            inherit self pkgs nix-filter cargoToml naerskPlatform buildInputs;
          };

          # nix build .#throwawayDevImage
          throwawayDevImage = callPackage nix/packages/throwaway-dev-image.nix {
            inherit pkgs packages cargoToml buildInputs;
          };
        };

        # nix develop
        devShell = mkShell {
          inherit buildInputs;
          name = "throwaway-shell";
        };
      });
}
