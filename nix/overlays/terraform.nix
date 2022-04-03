self: super:

let
  inherit (super.pkgs) fetchFromGitHub;
in
{
  terraform = super.terraform.overrideAttrs (old: rec {
    name = "terraform-${version}";
    version = "1.1.7";

    src = fetchFromGitHub {
      owner = "hashicorp";
      repo = "terraform";
      rev = "v${version}";
      sha256 = "sha256-E8qY17MSdA7fQW4wGSDiPzbndBP5SZwelAJAWzka/io=";
    };
  });
}
