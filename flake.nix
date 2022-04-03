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
        inherit (pkgs) lib mkShell fetchFromGitHub fetchurl dockerTools;
        inherit (pkgs.darwin.apple_sdk.frameworks) SystemConfiguration;

        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            (import nix/fluxcd.nix)
            (import nix/kube3d.nix)
            (import nix/postgresql.nix)
            (import nix/terraform.nix)

            (import nix/sqlx-cli.nix { inherit cargoToml; })

            (import nix/cargo-nextest.nix { inherit naerskPlatform; })
            (import nix/cargo-zigbuild.nix { inherit naerskPlatform; })
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
          throwaway = naerskPlatform.buildPackage {
            inherit buildInputs;

            name = "throwaway";
            pname = "throwaway";
            version = cargoToml.package.version;

            src = nix-filter.lib {
              root = self;
              include = [
                (nix-filter.lib.inDirectory "sql")
                (nix-filter.lib.inDirectory "src")
                "Cargo.lock"
                "Cargo.toml"
                "sqlx-data.json"
              ];
            };

            release = false;
            doCheck = false;
          };

          # nix build .#throwawayZig
          throwawayZig = naerskPlatform.buildPackage {
            inherit buildInputs;

            name = "throwaway";
            pname = "throwaway-zig";
            version = cargoToml.package.version;

            src = nix-filter.lib {
              root = self;
              include = [
                (nix-filter.lib.inDirectory "sql")
                (nix-filter.lib.inDirectory "src")
                "Cargo.lock"
                "Cargo.toml"
                "sqlx-data.json"
              ];
            };

            override = _: {
              preBuild = ''
                # exporting HOME to avoid using `/homeless-shelter/Library/Caches`
                # (read only filesystem on MacOS)
                export HOME=$TMP
              '';
            };

            release = false;
            doCheck = false;

            cargoBuild = x: ''cargo $cargo_options zigbuild $cargo_build_options >> $cargo_build_output_json'';
            cargoBuildOptions = x: x ++ [ "--target" "aarch64-unknown-linux-musl" ];
          };

          # nix build .#throwawayNixImage
          throwawayNixImage = dockerTools.buildLayeredImageWithNixDb {
            name = "throwaway-nix";
            tag = cargoToml.package.version;

            contents = buildInputs;

            config = {
              Entrypoint = [ "nix" "develop" "--verbose" "--command" ];
            };
          };

          # nix build .#throwawayDevImage
          throwawayDevImage = dockerTools.buildLayeredImage {
            name = "throwaway-dev";
            tag = "latest";

            contents = with packages; [
              throwawayZig
            ];

            extraCommands = ''
              # setup /tmp
              mkdir -p tmp
              chmod u+w tmp
            '';

            config = {
              User = "1000:1000";
              Cmd = [ "${packages.throwawayZig}/bin/throwaway" ];
            };
          };
        };

        # nix develop
        devShell = mkShell {
          inherit buildInputs;
          name = "throwaway-shell";
        };
      });
}
