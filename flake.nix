{
  description = "Remote-validated lower proposal for the meaning language";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = {
      url = "github:LiGoldragon/rust-build";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    ethos-zero = {
      url = "github:LiGoldragon/ethos-zero/5a5daca315ab24460ccbcd5038130c453e853e6e";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-build.follows = "rust-build";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-build, ethos-zero }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = rust-build.lib.${system}.fromPkgs pkgs;
        inherit (rust) craneLib toolchain;
        source = rust.cleanSource { root = ./.; extraFilters = [
          (path: type: type == "regular" && (
            pkgs.lib.hasSuffix ".ethos" path || pkgs.lib.hasSuffix ".json" path ||
            pkgs.lib.hasSuffix ".sh" path || pkgs.lib.hasSuffix ".md" path
          ))
        ]; };
        # Cargo.lock is a proposed seed formed from the public Ethos Zero 4bf
        # graph plus this root and its public git identity. Remote Cargo is the
        # authority that accepts or rejects this seed.
        seededSource = source;
        vendor = craneLib.vendorCargoDeps { src = seededSource; };
        common = { src = seededSource; cargoVendorDirectory = vendor; strictDeps = true; };
        artifacts = craneLib.buildDepsOnly common;
        remoteCheck = craneLib.cargoTest (common // {
          cargoArtifacts = artifacts;
          cargoTestExtraArgs = "--features datom -- --nocapture";
          nativeBuildInputs = [ ethos-zero.packages.${system}.default pkgs.python3 ];
          preBuild = ''
            ethos-zero "Generate.{ $PWD/ethos/meaning.ethos $PWD/src/generated }"
          '';
          postCheck = ''
            bash tests/check-glossary-coverage.sh
          '';
        });
      in {
        packages.default = craneLib.buildPackage (common // { cargoArtifacts = artifacts; });
        checks.generation-and-datom-roundtrip = remoteCheck;
      });
}
