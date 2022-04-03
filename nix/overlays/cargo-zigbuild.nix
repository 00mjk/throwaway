{ naerskPlatform }:

self: super:

let
  inherit (super.pkgs) fetchFromGitHub;
in
{
  cargo-zigbuild = naerskPlatform.buildPackage {
    name = "cargo-zigbuild";
    version = "0.8.1";

    src = fetchFromGitHub {
      owner = "messense";
      repo = "cargo-zigbuild";
      rev = "v0.8.1";
      sha256 = "sha256-Xd9saaqSc2o8Tl5XSvOb18+t2ru8FGg4LJN3ctVbctI=";
    };
  };
}
