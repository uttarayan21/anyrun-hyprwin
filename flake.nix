{
  description = "A plugin for anyrun to search for hyprland windows";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          pkgs = nixpkgs.legacyPackages.${system};
          code = pkgs.callPackage ./. {inherit nixpkgs system rust-overlay;};
        in rec {
          packages = {
            hyprwin = code.app;
            default = packages.hyprwin;
          };
        }
      );
}
