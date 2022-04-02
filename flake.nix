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
        inherit (pkgs) lib mkShell fetchFromGitHub dockerTools;
        inherit (pkgs.darwin.apple_sdk.frameworks) SystemConfiguration;

        pkgs = import nixpkgs {
          inherit system;
        };

        zig-toolchain = zig.packages.${system}."0.9.1";
        rust-toolchain = fenix.packages."${system}".fromToolchainFile {
          file = "${self}/rust-toolchain.toml";
          sha256 = "sha256-k6wD6/2qNQ7rmIvGi1ddtKSTUXjXFbIr0Sg2mqF2nYg=";
        };

        naerskPlatform = naersk.lib.${system}.override {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };

        cargoToml = fromTOML (readFile ("${self}/Cargo.toml"));

        sqlx-cli = pkgs.sqlx-cli.overrideAttrs (old: rec {
          name = "sqlx-cli-${version}";
          version = cargoToml.dependencies.sqlx.version;

          cargoSha256 = "sha256-EKuRaVxwotgTPj95GJnrQGbulsFPClSettwS5f0TzoM=";
          src = fetchFromGitHub {
            owner = "launchbadge";
            repo = "sqlx";
            rev = "v${version}";
            sha256 = "sha256-Tz7YzGkQUwH0U14dvsttP2GpnM9kign6L9PkAVs3dEc=";
          };
        });

        cargo-nextest = naerskPlatform.buildPackage {
          pname = "cargo-nextest";
          version = "0.9.12";

          cargoBuildFlags = [
            "--manifest-path"
            "cargo-nextest/Cargo.toml"
            "--bin"
            "cargo-nextest"
          ];

          cargoSha256 = "sha256-zrYmZG3VAneanHaNoG3txv7LbKCYvqIf60g1W7CmPG8=";
          src = fetchFromGitHub {
            owner = "nextest-rs";
            repo = "nextest";
            rev = "cargo-nextest-0.9.12";
            sha256 = "sha256-E3/AgzLvjlMfbmvAOYx4V1/1wSLKlFo61tGv79ow7XY=";
          };
        };

        cargo-zigbuild = naerskPlatform.buildPackage {
          name = "cargo-zigbuild";
          version = "0.8.1";

          src = fetchFromGitHub {
            owner = "messense";
            repo = "cargo-zigbuild";
            rev = "v0.8.1";
            sha256 = "sha256-Xd9saaqSc2o8Tl5XSvOb18+t2ru8FGg4LJN3ctVbctI=";
          };
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
