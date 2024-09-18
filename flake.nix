{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { system, pkgs, my-rust, ... }: {
        _module.args = {
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };
          my-rust = pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
            ];
          };
        };

        packages = let
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (_: my-rust);
          src = craneLib.cleanCargoSource ./.;

          commonArgs = {
            inherit src;
            strictDeps = true;
            doCheck = false; # do not run tests during build
          };

          cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
            pname = "serde-redes-deps-only";
            version = "0.0.0";
          });

          fileSetForCrate = crate: pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              crate
            ];
          };

          individualCrateArgs = crate: commonArgs // rec {
            inherit (craneLib.crateNameFromCargoToml { src = crate; })
              pname version;
            src = fileSetForCrate crate;
            cargoExtraArgs = "-p ${pname}";
            inherit cargoArtifacts;
          };

        in {
          serde-ast = craneLib.buildPackage (individualCrateArgs ./serde-ast);
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.git
            my-rust
          ];
        };
      };
    };
}
