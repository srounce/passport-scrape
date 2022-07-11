{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        projectCrate = rustPlatform.buildRustPackage {
          src = "./.";

          cargoLock.lockFile = "./Cargo.lock";
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
            (rustVersion.override {
              extensions = [ "rust-src" "rustfmt" ];
            })
            pkgs.rust-analyzer
            pkgs.openssl
            pkgs.pkg-config

            pkgs.treefmt
          ];
        };
      }
    );
}
