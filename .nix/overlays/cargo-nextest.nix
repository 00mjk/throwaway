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
  version = "0.9.11";

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
    sha256 = "sha256-aXGXn6kxw37v0fn2SPQnGCdTg1CFJK03OIAgFjvaxpc=";
  };

  cargoSha256 = "sha256-zuPWF2sxVmlhSMy70yhRZEV6dgL2W8AGkYRbnbJdMpQ=";
  doCheck = false;
}
