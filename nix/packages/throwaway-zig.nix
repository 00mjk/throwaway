{ self
, pkgs
, nix-filter
, cargoToml
, naerskPlatform
, buildInputs
}:

let
  inherit (pkgs) fetchFromGitHub;
in
naerskPlatform.buildPackage {
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
}
