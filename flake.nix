{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system overlays };
        rustVersion = pkgs.rust-bin.stable.latest.default;
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
            (rustVersion.override {
              extensions = [
                "rust-src"
              ];
            })
          ];
        };
      }
    );
}
