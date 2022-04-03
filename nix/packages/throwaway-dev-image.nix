{ pkgs
, packages
, cargoToml
, buildInputs
}:

let
  inherit (pkgs) dockerTools;
in
dockerTools.buildLayeredImage {
  name = "throwaway-dev";
  tag = "latest";

  contents = with packages; [
    throwawayZig
  ];

  extraCommands = ''
    # setup /tmp
    mkdir -p tmp
    chmod u+w tmp
  '';

  config = {
    User = "1000:1000";
    Cmd = [
      "${packages.throwawayZig}/bin/throwaway"
    ];
  };
}
