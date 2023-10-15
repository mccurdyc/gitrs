# https://www.tweag.io/blog/2022-09-22-rust-nix/
# https://ryantm.github.io/nixpkgs/languages-frameworks/rust/
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    systems,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      rustVersion = pkgs.rust-bin.stable.latest.default;

      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustVersion;
        rustc = rustVersion;
      };

      rsBuild = rustPlatform.buildRustPackage {
        pname = "gitrs";
        version = "v0.3.6";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };
    in {
      defaultPackage = rsBuild;

      formatter = pkgs.alejandra;

      devShells = {
        default = import ./shell.nix {inherit pkgs;};
      };
    });
}
