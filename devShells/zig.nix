{
  perSystem =
    { config, pkgs, ... }:
    {
      devShells.zig = pkgs.mkShell {
        name = "AoC-zig";

        nativeBuildInputs = config.devShells.base.nativeBuildInputs;

        packages = with pkgs; [
          zig
          zls
        ];
      };
    };
}
