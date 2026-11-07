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
        # The cargo build script generates from the Ethos with the same ethos-zero
        # revision the flake pins and asserts the checked-in module equals it, so a
        # stale src/generated/meaning.rs fails this check; nothing here overwrites it.
        remoteCheck = craneLib.cargoTest (common // {
          cargoArtifacts = artifacts;
          cargoTestExtraArgs = "--features datom -- --nocapture";
          nativeBuildInputs = [ pkgs.python3 ];
          postCheck = ''
            bash tests/check-glossary-coverage.sh
          '';
        });
        # Default features only: the datom derives and the datom tests are absent.
        defaultFeatures = craneLib.cargoTest (common // {
          cargoArtifacts = artifacts;
          cargoTestExtraArgs = "";
        });
        # The generator binary the flake pins, run on the authored Ethos, against the
        # checked-in module, byte for byte.
        generatedEquality = pkgs.runCommand "meaning-generated-equality" {
          nativeBuildInputs = [ ethos-zero.packages.${system}.default ];
        } ''
          mkdir generated
          cp ${./ethos/meaning.ethos} meaning.ethos
          ethos-zero "Generate.{ $PWD/meaning.ethos $PWD/generated }"
          diff -u ${./src/generated/meaning.rs} generated/meaning.rs
          touch $out
        '';
      in {
        packages.default = craneLib.buildPackage (common // { cargoArtifacts = artifacts; });
        checks.generation-and-datom-roundtrip = remoteCheck;
        checks.default-features = defaultFeatures;
        checks.generated-equality = generatedEquality;
      });
}
