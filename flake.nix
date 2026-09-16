{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    git-hooks.url = "github:cachix/git-hooks.nix";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    mccurdyc-preferences.url = "github:mccurdyc/nix-templates?dir=modules";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "x86_64-linux"
      ];

      imports = [
        inputs.git-hooks.flakeModule
        inputs.treefmt-nix.flakeModule
        inputs.mccurdyc-preferences.flakeModules.default

        # Apply rust-overlay to pkgs in a separate module so the
        # perSystem below can use pkgs.rust-bin without circularity.
        {
          perSystem =
            { system, ... }:
            {
              _module.args.pkgs = import inputs.nixpkgs {
                inherit system;
                overlays = [
                  inputs.rust-overlay.overlays.default
                ];
              };
            };
        }
      ];

      perSystem =
        { pkgs, ... }:
        let
          rustToolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "clippy"
              "rustfmt"
            ];
          };

          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

          src = craneLib.cleanCargoSource ./.;

          commonArgs = {
            inherit src;
            strictDeps = true;
            nativeBuildInputs = with pkgs; [
              pkg-config
              makeWrapper
            ];
            buildInputs = with pkgs; [
              openssl
            ];
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in
        {
          mccurdyc.rust = {
            enable = true;
            toolchain = rustToolchain;
          };

          mccurdyc.devshell.extraPackages = with pkgs; [
            pkg-config
            openssl
          ];

          # Work around mccurdyc-preferences default formatter referencing
          # config.mccurdyc.pre-commit.enable without the pre-commit module
          # being imported by flakeModules.default.
          mccurdyc.devshell.formatter = pkgs.nixfmt;

          packages = {
            app = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
            app-doc = craneLib.cargoDoc (
              commonArgs
              // {
                inherit cargoArtifacts;
                RUSTDOCFLAGS = "-D warnings";
              }
            );
          };

          checks = {
            app-clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets --all-features -- " + "--deny warnings";
              }
            );
          };
        };
    };
}
