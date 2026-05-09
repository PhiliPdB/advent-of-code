{
  imports = [
    ./dotnet.nix
    ./rust.nix
  ];

  perSystem =
    { pkgs, ... }:
    {
      devShells.base = pkgs.mkShell {
        name = "Empty base shell for AoC";

        nativeBuildInputs = with pkgs; [
          aoc-cli

          # For performance benchmarking
          hyperfine

          # Script shortcuts
          (pkgs.writeScriptBin "di" (builtins.readFile ../scripts/download_current_day_puzzle_input.sh))
        ];
      };
    };
}
