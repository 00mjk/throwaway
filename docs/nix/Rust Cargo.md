# Rust / Cargo
Basic template

```nix
{ lib
, stdenv
, fetchFromGitHub
, rustPlatform
}:

with rustPlatform;

buildRustPackage rec {
  meta = with lib; {
    description = "";
    homepage = "";
    license = with licenses; [ mit asl20 ];
  };

  pname = "";
  version = "";

  cargoBuildFlags = [
    "--bin" ""
  ];

  src = fetchFromGitHub {
    owner = "";
    repo = "";
    rev = "v${version}";
    sha256 = lib.fakeSha256;
  };

  cargoSha256 = lib.fakeSha256;
  doCheck = false;
}
```
