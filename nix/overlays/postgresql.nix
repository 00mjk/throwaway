self: super:

let
  inherit (super.pkgs) fetchFromGitHub;
in
{
  fluxcd = super.fluxcd.overrideAttrs (old: rec {
    name = "fluxcd-${version}";
    version = "0.27.4";

    src = fetchFromGitHub {
      owner = "fluxcd";
      repo = "flux2";
      rev = "v${version}";
      sha256 = "sha256-4JFS3EhdT3XtV1CXxAt3mEbJJoEVabu0PSE/MUYMJRk=";
    };
  });
}
