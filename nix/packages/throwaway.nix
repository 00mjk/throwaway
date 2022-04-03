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
}
