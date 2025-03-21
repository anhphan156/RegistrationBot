{
  description = "Rust Project";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [
            (import rust-overlay)
            (final: _: {
              rust-toolchain = final.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            })
          ];
        };
      in {
        packages = rec {
          native-dynamic = pkgs.callPackage ./packages/native.nix {};
          native-static = pkgs.pkgsStatic.callPackage ./packages/native.nix {};
          aarch64-static = pkgs.pkgsCross.aarch64-multiplatform.pkgsStatic.callPackage ./packages/cross.nix {};
          default = aarch64-static;
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust-toolchain
            cargo-watch
            python3
            python3Packages.requests
            ngrok
            openssl
            pkg-config
            postman
          ];
          ROCKET_PROFILE = "debug";
        };
      }
    );
}
