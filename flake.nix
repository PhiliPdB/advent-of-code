{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { flake-parts, nixpkgs, rust-overlay, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem = { system, pkgs, ... }: {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        devShells =
          let
            globalPackages = with pkgs; [
              # Hyperfine for performance benchmarking
              hyperfine
            ];
          in {
            rust = pkgs.mkShell {
              name = "AoC-rust";

              buildInputs = with pkgs; [
                (rust-bin.stable.latest.default.override {
                  extensions = [
                    "clippy"
                    "rust-analyzer"
                  ];
                })
              ];

              packages = globalPackages;
            };
          };
      };
    };
}
