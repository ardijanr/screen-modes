{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        libPath = with pkgs; lib.makeLibraryPath [
          libxkbcommon
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];
      in
      {

        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            cargo-insta
            rust-analyzer
            rustPackages.clippy
            rustc
            rustfmt
          ];
          LD_LIBRARY_PATH = libPath;
        };
      });
}

