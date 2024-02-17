{
  description = "A very basic flake";

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
            app = code.app;
            default = packages.app;
          };
        }
      );
}
