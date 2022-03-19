final: prev: {
  cargo-nextest = prev.callPackage ./overlays/cargo-nextest.nix { };
  cargo-zigbuild = prev.callPackage ./overlays/cargo-zigbuild.nix { };
}
