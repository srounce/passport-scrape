{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system overlays; };

        lib = pkgs.lib;
        stdenv = pkgs.stdenv;

        apple_sdk = pkgs.darwin.apple_sdk;

        overlays = [ (import rust-overlay) ];

        rustVersion = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        projectCrate = rustPlatform.buildRustPackage {
          name = "passport_scrape";
          version = "0.1.0";

          src = ./.;

          buildInputs = [
            pkgs.openssl
          ] ++ lib.optionals stdenv.isDarwin [ apple_sdk.frameworks.Cocoa ];

          NIX_CFLAGS_COMPILE = [ ] ++ lib.optionals stdenv.isDarwin [
            # disable modules, otherwise we get redeclaration errors
            "-fno-modules"
            # link AppKit since we don't get it from modules now
            "-framework"
            "Cocoa"
          ];

          cargoLock.lockFile = ./Cargo.lock;
        };
      in
      {
        defaultPackage = projectCrate;

        formatter = pkgs.nixpkgs-fmt;

        devShell = pkgs.mkShell {
          buildInputs = [
            projectCrate

            (rustVersion.override {
              extensions = [ "rust-src" "rustfmt" ];
            })
            pkgs.rust-analyzer

            pkgs.pkg-config

            pkgs.treefmt
          ];
        };
      }
    );
}
