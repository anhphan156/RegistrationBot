{
  rustPlatform,
  pkg-config,
  openssl,
}:
rustPlatform.buildRustPackage {
  name = "registration-bot";
  src = ./..;
  cargoLock.lockFile = ../Cargo.lock;
  depsBuildBuild = [
    pkg-config
  ];

  depsBuildTarget = [
    openssl
  ];

  NIX_LDFLAGS = "-L${openssl.out}/lib";
}
