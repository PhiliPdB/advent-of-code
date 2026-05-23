{
  perSystem =
    { config, pkgs, ... }:
    {
      devShells.cpp = pkgs.mkShell {
        name = "AoC-cpp";
        inputsFrom = [ config.devShells.base ];

        nativeBuildInputs = with pkgs; [
          cmake
          clang-tools
        ];

        packages = [
          (pkgs.writeScriptBin "cpp-run" (builtins.readFile ../scripts/cpp_run.sh))
        ];
      };
    };
}
