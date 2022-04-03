self: super:

let
  inherit (super.pkgs) fetchurl;
in
{
  postgresql = super.postgresql.overrideAttrs (old: rec {
    name = "postgresql-${version}";
    version = "14.2";

    src = fetchurl {
      url = "https://ftp.postgresql.org/pub/source/v${version}/${name}.tar.bz2";
      sha256 = "sha256-LPeLLkaJEvgQHWldtTQM8xPC6faKYS+3FCdSToyal3o=";
    };
  });
}
