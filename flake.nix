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
        packages.default = let
          armPkgs = pkgs.pkgsCross.aarch64-multiplatform;
          openssl = armPkgs.openssl;
          # rustPlatform = armPkgs.makeRustPlatform {
          #   rustc = pkgs.rust-toolchain;
          #   cargo = pkgs.rust-toolchain;
          # };
        in
          armPkgs.rustPlatform.buildRustPackage {
            name = "RegistrationBot";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            # preBuild = ''
            #   export OPENSSL_DIR="${openssl}"
            #   export OPENSSL_LIB_DIR="${openssl}/lib"
            #   export OPENSSL_INCLUDE_DIR="${openssl}/include"
            #   export PKG_CONFIG_PATH="${openssl}/lib/pkgconfig"
            #   export PKG_CONFIG_ALLOW_CROSS=1
            # '';
            nativeBuildInputs = with armPkgs; [
              pkg-config
            ];
            buildInputs = with armPkgs; [
              openssl
            ];
            # target = "aarch64-unknown-linux-gnu";
            # cargoBuildFlags = ["--target=aarch64-unknown-linux-gnu"];
          };

        packages.static = pkgs.pkgsStatic.rustPlatform.buildRustPackage {
          name = "RegistrationBot";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = with pkgs.pkgsStatic; [
            pkg-config
          ];
          buildInputs = with pkgs.pkgsStatic; [
            openssl
          ];
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
        };
      }
    );
}
