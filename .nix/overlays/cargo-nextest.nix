{ lib
, stdenv
, fetchFromGitHub
, rustPlatform
}:

with rustPlatform;

buildRustPackage rec {
  meta = with lib; {
    description = "A next-generation test runner for Rust.";
    homepage = "https://github.com/nextest-rs/nextest";
    license = with licenses; [ mit asl20 ];
  };

  pname = "cargo-nextest";
  version = "0.9.12";

  cargoBuildFlags = [
    "--manifest-path"
    "cargo-nextest/Cargo.toml"
    "--bin"
    "cargo-nextest"
  ];

  src = fetchFromGitHub {
    owner = "nextest-rs";
    repo = "nextest";
    rev = "cargo-nextest-${version}";
    sha256 = "sha256-E3/AgzLvjlMfbmvAOYx4V1/1wSLKlFo61tGv79ow7XY=";
  };

  cargoSha256 = "sha256-zrYmZG3VAneanHaNoG3txv7LbKCYvqIf60g1W7CmPG8=";
  doCheck = false;
}
