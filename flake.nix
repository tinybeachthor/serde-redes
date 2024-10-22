{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = inputs@{ self, flake-parts, advisory-db, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { system, ... }: let
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

          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (_: my-rust);
          src = craneLib.cleanCargoSource ./.;

          commonArgs = {
            inherit src;
            strictDeps = true;
            doCheck = false; # do not run tests during build, use nextest check instead
          };
          cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
            pname = "serde-redes-deps-only";
            version = "0.0.0";
          });
          individualCrateArgs = crate: commonArgs // rec {
            inherit (craneLib.crateNameFromCargoToml { src = crate; })
              pname version;
            cargoExtraArgs = "-p ${pname}";
            inherit src cargoArtifacts;
          };

      in {
        packages = {
          serde-ast = craneLib.buildPackage (individualCrateArgs ./serde-ast);
          serde-metadata = craneLib.buildPackage (individualCrateArgs ./serde-metadata);
          serde-redes = craneLib.buildPackage (individualCrateArgs ./serde-redes);

          default = pkgs.symlinkJoin {
            name = "serde-redes-all";
            paths = with self.packages.${system}; [
              serde-ast
              serde-metadata
              serde-redes
            ];
          };

          deps = cargoArtifacts;
        };

        checks = {
          build = self.packages.${system}.default;

          nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });

          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });
          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          fmt = craneLib.cargoFmt {
            inherit src;
          };
          fmt-toml = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
          };

          audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };
          deny = craneLib.cargoDeny {
            inherit src;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.git

            my-rust
            pkgs.taplo
            pkgs.cargo-deny
          ];
        };
      };
    };
}
