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

    zig = {
      url = "github:roarkanize/zig-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, zig }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import .nix/overlay.nix)
        ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust-toolchain = fenix.packages."${system}".fromToolchainFile {
          file = "${self}/rust-toolchain.toml";
          sha256 = "sha256-qYjPB1399EkSf+a8LpzelPusG5GZjAhAmuaJ4wDqMkU=";
        };

        zig-toolchain = zig.packages.${system}."0.9.1";

        sqlx-cli = pkgs.sqlx-cli.overrideAttrs (old: rec {
          name = "sqlx-cli-${version}";
          version = "0.5.11";

          src = pkgs.fetchFromGitHub {
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
        devShell = pkgs.mkShell {
          name = "throwaway-shell";

          buildInputs = with pkgs; [
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
            zig-toolchain

            # Nix
            nixpkgs-fmt

            # Utilities
            curl
          ] ++ lib.optional (stdenv.isLinux && stdenv.isx86_64) [
            # Rust Crates
            cargo-tarpaulin
          ];
        };
      });
}
