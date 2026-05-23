{
  perSystem =
    { config, pkgs, ... }:
    {
      devShells.rust = pkgs.mkShell {
        name = "AoC-rust";
        inputsFrom = [ config.devShells.base ];

        nativeBuildInputs = with pkgs; [
          cmake
          rustPlatform.bindgenHook
          pkg-config
        ];

        buildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            extensions = [
              "clippy"
              "rust-analyzer"
              "rust-src"
            ];
          })
          libclang.lib
        ];

        # Needed if using bindgen to wrap C libraries in Rust
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

        packages = with pkgs; [
          cargo-expand
          cargo-show-asm
        ];
      };
    };
}
