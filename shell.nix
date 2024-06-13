with import <nixpkgs> {};
mkShell {
  packages = [
    cargo
    rustc
    rust-analyzer
    rustfmt
    python3
  ];
  env.RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
