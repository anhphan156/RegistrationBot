{
  rustPlatform,
  pkg-config,
  openssl,
}:
rustPlatform.buildRustPackage {
  name = "RegistrationBot";
  src = ./..;
  cargoLock.lockFile = ../Cargo.lock;
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    openssl
  ];
}
