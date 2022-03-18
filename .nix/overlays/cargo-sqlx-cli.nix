{ lib
, stdenv
, fetchFromGitHub
, rustPlatform
, SystemConfiguration
}:

with rustPlatform;

buildRustPackage rec {
  meta = with lib; {
    description = "SQLx's associated command-line utility for managing databases, migrations, and enabling offline mode with sqlx::query!() and friends.";
    homepage = "https://github.com/launchbadge/sqlx";
    license = with licenses; [ mit asl20 ];
  };

  pname = "sqlx-cli";
  version = "0.5.11";

  buildNoDefaultFeatures = true;
  buildFeatures = [ "rustls" "postgres" ];

  cargoBuildFlags = [
    "--manifest-path"
    "sqlx-cli/Cargo.toml"
    "--bin"
    "cargo-sqlx"
  ];

  src = fetchFromGitHub {
    owner = "launchbadge";
    repo = "sqlx";
    rev = "v${version}";
    sha256 = "sha256-Tz7YzGkQUwH0U14dvsttP2GpnM9kign6L9PkAVs3dEc=";
  };

  cargoSha256 = "sha256-EKuRaVxwotgTPj95GJnrQGbulsFPClSettwS5f0TzoM=";
  doCheck = false;

  buildInputs = with pkgs; []
  ++ lib.optional stdenv.isDarwin [
    SystemConfiguration
  ];
}
