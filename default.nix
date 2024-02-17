{pkgs, ...}: {
  app = pkgs.rustPlatform.buildRustPackage {
    pname = "anyrun-hyprwin";
    version = "0.1.0";
    src = ./.;
    cargoBuildFlags = "";
    cargoLock = {
      lockFile = ./Cargo.lock;
      outputHashes = {
        "anyrun-interface-0.1.0" = "sha256-hI9+KBShsSfvWX7bmRa/1VI20WGat3lDXmbceMZzMS4=";
      };
    };
  };
}
