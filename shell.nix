with import <nixpkgs> {};
mkShell {
  packages = [
    cargo
    rustc
    rust-analyzer
    rustfmt
    clippy
    #
    ran
  ];
  env.RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
