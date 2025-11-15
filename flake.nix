{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      flake-parts,
      nixpkgs,
      rust-overlay,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem =
        { system, pkgs, ... }:
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];

            config.permittedInsecurePackages = [
              "dotnet-sdk-6.0.428" # Year 2016 is written with dotnet 6
              "dotnet-sdk-7.0.410" # Year 2015 is written with dotnet 7
            ];
          };

          devShells =
            let
              globalPackages = with pkgs; [
                aoc-cli

                # For performance benchmarking
                hyperfine

                # Script shortcuts
                (pkgs.writeScriptBin "di" (builtins.readFile ./scripts/download_current_day_puzzle_input.sh))
              ];
            in
            {
              dotnet = pkgs.mkShell rec {
                name = "AoC-dotnet";
                dotnetPkg = (
                  with pkgs.dotnetCorePackages;
                  combinePackages [
                    sdk_6_0
                    sdk_7_0
                  ]
                );

                dependencies = with pkgs; [
                  zlib
                  zlib.dev
                  icu
                  openssl

                  dotnetPkg
                ];

                NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
                  [
                    pkgs.stdenv.cc.cc
                  ]
                  ++ dependencies
                );
                NIX_LD = "${pkgs.stdenv.cc.libc_bin}/bin/ld.so";

                nativeBuildInputs = dependencies;

                packages = globalPackages;

                shellHook = ''
                  DOTNET_ROOT="${dotnetPkg}";
                '';
              };

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

                packages =
                  with pkgs;
                  [
                    cargo-expand
                    cargo-show-asm
                  ]
                  ++ globalPackages;
              };
            };
        };
    };
}
