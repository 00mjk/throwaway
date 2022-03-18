{ lib
, stdenv
, fetchFromGitHub
, rustPlatform
}:

with rustPlatform;

buildRustPackage rec {
  meta = with lib; {
    description = "Compile Cargo project with zig as linker.";
    homepage = "https://github.com/messense/cargo-zigbuild";
    license = licenses.mit;
  };

  pname = "cargo-zigbuild";
  version = "0.6.6";

  cargoBuildFlags = [
    "--bin"
    "cargo-zigbuild"
  ];

  src = fetchFromGitHub {
    owner = "messense";
    repo = "cargo-zigbuild";
    rev = "v${version}";
    sha256 = "sha256-YnRIH0IDMg7SOrY6EkFlKluEslzLyIwN5PSoLn/BjY8=";
  };

  cargoSha256 = "sha256-n8DZrSHhHUibMORFIfCZdTi971hbWHvoxnoxIXL0Y5c=";
  doCheck = false;
}
