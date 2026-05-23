{
  perSystem =
    { config, pkgs, ... }:
    {
      devShells.dotnet = pkgs.mkShell rec {
        name = "AoC-dotnet";
        inputsFrom = [ config.devShells.base ];

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

        shellHook = ''
          DOTNET_ROOT="${dotnetPkg}";
        '';
      };
    };
}
