{
  imports = [
    ./cpp.nix
    ./dotnet.nix
    ./rust.nix
    ./zig.nix
  ];

  perSystem =
    { config, pkgs, ... }:
    {
      devShells.all = pkgs.mkShell {
        name = "AoC-all";

        inputsFrom = [
          config.devShells.cpp
          config.devShells.dotnet
          config.devShells.rust
          config.devShells.zig
        ];
      };

      devShells.base = pkgs.mkShell {
        name = "Empty base shell for AoC";

        packages = with pkgs; [
          aoc-cli

          # For performance benchmarking
          hyperfine

          # Script shortcuts
          (pkgs.writeScriptBin "di" (builtins.readFile ../scripts/download_current_day_puzzle_input.sh))
        ];
      };
    };
}
