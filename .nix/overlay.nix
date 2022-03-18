final: prev: {
  cargo-nextest = prev.callPackage ./overlays/cargo-nextest.nix { };

  cargo-sqlx-cli = prev.callPackage ./overlays/cargo-sqlx-cli.nix {
    inherit (final.darwin.apple_sdk.frameworks) SystemConfiguration;
  };

  cargo-zigbuild = prev.callPackage ./overlays/cargo-zigbuild.nix { };
}
