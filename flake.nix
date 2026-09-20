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
          gitrs = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
            pname = "gitrs";
            version = "v0.4.1";

            src = pkgs.fetchFromGitHub {
              owner = "mccurdyc";
              repo = "gitrs";
              rev = finalAttrs.version;
              # nix-shell -p nix-prefetch-git --run "nix-prefetch-git --url https://github.com/mccurdyc/gitrs.git --rev v0.4.1"
              hash = "sha256-YxojhqcP5Jj+GUhZxwyz1WXpRrTc4mZKxJbJdbnEZ48=";
            };

            # cargoHash = pkgs.lib.fakeHash;
            cargoHash = "sha256-uxK7HSP7rTPsnSwgj8pJRdXR2N9xqx21TycTRCjdAGo=";

            nativeBuildInputs = [
              pkgs.pkg-config # for openssl
            ];

            buildInputs = [
              pkgs.openssl.dev
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
              pkgs.libiconv
              pkgs.libz
            ];
          });

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
            gitrs
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
