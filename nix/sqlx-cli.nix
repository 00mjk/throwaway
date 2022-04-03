self: super:

let
  inherit (builtins) fromTOML readFile;
  inherit (super.pkgs) fetchFromGitHub;
in
{
  sqlx-cli = super.sqlx-cli.overrideAttrs (old: rec {
    name = "sqlx-cli-${version}";
    version = "0.5.11";

    cargoSha256 = "sha256-EKuRaVxwotgTPj95GJnrQGbulsFPClSettwS5f0TzoM=";
    src = fetchFromGitHub {
      owner = "launchbadge";
      repo = "sqlx";
      rev = "v${version}";
      sha256 = "sha256-Tz7YzGkQUwH0U14dvsttP2GpnM9kign6L9PkAVs3dEc=";
    };
  });
}
