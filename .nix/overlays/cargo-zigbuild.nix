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
    license = with licenses; [ mit ];
  };

  pname = "cargo-zigbuild";
  version = "0.6.7";

  cargoBuildFlags = [
    "--bin"
    "cargo-zigbuild"
  ];

  src = fetchFromGitHub {
    owner = "messense";
    repo = "cargo-zigbuild";
    rev = "v${version}";
    sha256 = "sha256-oqfKlc/J1KjVPzaCsRo/WQQv9L4qUXSFIzTo4YuGUT0=";
  };

  cargoSha256 = "sha256-r9VHjWoD3+6i/jxfsX5G/OCMrnge/X3KIA7G7Yr39U8=";
  doCheck = false;
}
