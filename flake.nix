{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-26.05";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        lib = pkgs.lib;

        craneLib = crane.mkLib pkgs;
        nightlyCraneLib = (crane.mkLib pkgs).overrideToolchain (
          p:
          p.rust-bin.selectLatestNightlyWith (
            toolchain:
            toolchain.default.override {
              extensions = [
                "rust-analyzer"
                "rust-src"
              ];
            }
          )
        );

        unfilteredRoot = ./.;
        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            ./testdata.in
            ./testdata.out
          ];
        };
      in
      {
        packages.default = craneLib.buildPackage {
          inherit src;
        };
        devShells.default = nightlyCraneLib.devShell {
          packages = [
          ];
        };
      }
    );
}
