{
  rustPlatform,
  pkg-config,
  openssl,
}:
rustPlatform.buildRustPackage {
  name = "RegistrationBot";
  src = ./..;
  cargoLock.lockFile = ../Cargo.lock;
  depsBuildBuild = [
    pkg-config
  ];

  depsBuildTarget = [
    openssl
  ];

  NIX_LDFLAGS = "-L${openssl.out}/lib";
  # CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
}
