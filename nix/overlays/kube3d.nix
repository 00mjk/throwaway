self: super:

let
  inherit (super.pkgs) fetchFromGitHub;
in
{
  kube3d = super.kube3d.overrideAttrs (old: rec {
    name = "kube3d-${version}";
    version = "5.4.1";
    k3sVersion = "1.23.5+k3s1";

    src = fetchFromGitHub {
      owner = "k3d-io";
      repo = "k3d";
      rev = "v${version}";
      sha256 = "sha256-DVQrD4JMei9yRFzuiVb6AcydEupNSlpgYLfGWWRiaao=";
    };

    # k3d moved from the 'rancher' organisation to 'k3d-io', so these build flags needed updating.
    # https://github.com/k3d-io/k3d/blob/v5.4.1/Makefile#L72
    ldflags = [
      "-w"
      "-s"
      "-X"
      "github.com/k3d-io/k3d/v5/version.Version=v${version}"
      "-X"
      "github.com/k3d-io/k3d/v5/version.K3sVersion=v${k3sVersion}"
    ];

    installCheckPhase = ''
      runHook preInstallCheck
      $out/bin/k3d --help
      $out/bin/k3d --version
      $out/bin/k3d --version | grep -e "k3d version v${version}" -e "k3s version v${k3sVersion}"
      runHook postInstallCheck
    '';
  });
}
