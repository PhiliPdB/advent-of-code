{
  perSystem =
    { config, pkgs, ... }:
    {
      devShells.zig = pkgs.mkShell {
        name = "AoC-zig";
        inputsFrom = [ config.devShells.base ];

        packages = with pkgs; [
          zig
          zls
        ];
      };
    };
}
