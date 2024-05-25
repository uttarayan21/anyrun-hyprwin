{pkgs, ...}: {
  app = pkgs.rustPlatform.buildRustPackage {
    pname = "hyprwin";
    version = "0.1.0";
    src = ./.;
    # cargoBuildFlags = "";
    cargoLock = {
      lockFile = ./Cargo.lock;
    };
  };
}
