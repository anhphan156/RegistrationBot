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
  ROCKET_CONFIG = ../Rocket.toml;
  ROCKET_PROFILE = "production";
}
