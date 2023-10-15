# https://nixos.wiki/wiki/Flakes#Super_fast_nix-shell
{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    # https://nixos.wiki/wiki/Rust#Shell.nix_example
    nativeBuildInputs = with pkgs; [
      alejandra
      cargo
      cargo-flamegraph
      clippy
      gcc
      openssl
      openssl.dev
      pkg-config
      rnix-lsp
      rust-analyzer
      rustc
      rustfmt
      statix
    ];

    # https://nixos.wiki/wiki/Rust#Building_Rust_crates_that_require_external_system_libraries
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

    RUST_BACKTRACE = 1;
  }
