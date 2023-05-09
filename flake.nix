# This is devenv, but with Flakes - https://devenv.sh/guides/using-with-flakes/
# And is configured so that it automatically enters a nix-shell via nix-direnv
# when you change into the directory.
#
# https://www.tweag.io/blog/2022-09-22-rust-nix/
{
  inputs = {
    # I ran into issues with "version 'GLIBC_ABI_DT_RELR' not found" when I set to "unstable"
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    systems.url = "github:nix-systems/default";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    devenv.url = "github:cachix/devenv";

    # https://devenv.sh/blog/2022/12/22/devenv-05/#languages
    # Necessary for specifying "stable" for language.rust.version.
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, devenv, flake-utils, rust-overlay, systems, ... } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        rsBuild = rustPlatform.buildRustPackage {
          pname = "gitrs";
          version = "0.1.0";
          src = ./.;

          # https://dev.to/johnreillymurray/rust-environment-and-docker-build-with-nix-flakes-19c1
          # just for the host building the package
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.cmake
            pkgs.clang
            pkgs.gcc
            pkgs.openssl
            pkgs.zlib
          ];
          # packages needed by the consumer
          buildInputs = [ ];

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          RUST_BACKTRACE = "full";
        };
      in
      {
        defaultPackage = rsBuild;

        devShell = devenv.lib.mkShell
          {
            inherit inputs pkgs;

            # When devenv is used with Flakes, this is where you configure your
            # devenv shells
            #
            # Docs:
            # - https://nixos.wiki/wiki/Rust#devenv.sh_support
            # - https://devenv.sh/reference/options/
            # - https://devenv.sh/guides/using-with-flakes/#modifying-your-flakenix-file
            # - https://github.com/cachix/devenv/blob/main/src/modules/languages/rust.nix
            modules = [
              {
                languages.rust = {
                  enable = true;
                  version = "stable";
                };

                # https://nixos.wiki/wiki/Rust#Building_Rust_crates_that_require_external_system_libraries
                env.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

                packages = [
                  pkgs.openssl
                  pkgs.pkg-config
                  pkgs.vale
                ];

                # https://devenv.sh/reference/options/?query=rust#pre-commithooks
                pre-commit.hooks = {
                  clippy.enable = true;
                  rustfmt.enable = true;
                };
              }
            ];
          };
      });
}
