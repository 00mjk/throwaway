{ naerskPlatform }:

self: super:

let
  inherit (super.pkgs) fetchFromGitHub;
in
{
  cargo-nextest = naerskPlatform.buildPackage {
    name = "cargo-nextest";
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
}
