{
  rustPlatform,
  lib,
  targetPlatform,
  installShellFiles,
}:
rustPlatform.buildRustPackage {
  name = "bangscript";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ./.)) (
      lib.fileset.unions [
        ./src
        ./Cargo.toml
        ./Cargo.lock
      ]
    );
  };

  strictDeps = true;
  cargoLock.lockFile = ./Cargo.lock;

  env.RUSTFLAGS = lib.optionalString (targetPlatform.libc == "musl") "-C target-feature=+crt-static";

  nativeBuildInputs = [
    installShellFiles
  ];

  meta = {
    mainProgram = "bs";
  };
}
